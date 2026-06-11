// src/dashboard.rs — solar car telemetry dashboard (revamped)
// Reads from Redis TimeSeries; runs as a separate binary alongside solar_rust.
//
// Run with: cargo run --bin dashboard
// The CAN readers (connect_bus_1 / connect_bus_2) must be running and writing to Redis.
//
// ── What changed vs the old dashboard ────────────────────────────────────────
//   KEY FIXES (these were silently returning 0.0 before):
//     • "Kelly:rpm"  → "Kelly:speed_rpm"   (the only RPM key bus 1 writes)
//     • "Kelly:speed" does not exist        → speed is now DERIVED from RPM
//       via WHEEL_CIRCUMFERENCE_M (direct-drive hub motor assumed; add a gear
//       ratio constant if you have a reduction).
//     • "MOTOR OUT" was Σ(mppt output V·I) — that's charge power INTO the
//       pack, not motor power. Motor draw is now
//       Kelly:battery_voltage × Kelly:motor_current (newly fetched).
//     • All 11 Kelly fault flags are now fetched and displayed (was 4).
//     • Per-MPPT panels: watts in/out, MOSFET temp, and a status dot for each
//       of the 3 units instead of one aggregate boolean.
//     • Switch states (brake / foot / boost) and feedback_status now shown.
//
//   CONNECTION: one MultiplexedConnection is opened at startup and reused
//   every tick (the old code reconnected to Redis 10×/sec). On any fetch
//   error the connection is dropped and re-established on the next tick, and
//   the LINK indicator in the fault column turns red while the last good
//   frame stays on screen.
//
//   Polling is 200 ms — we fetch ~85 keys sequentially per tick; on loopback
//   that's a few ms. If you ever need it faster, batch the TS.GETs into a
//   redis::pipe() (all keys are TS.CREATE'd by the readers so they exist).

use iced::time;
use iced::widget::canvas::{Cache, Frame, Geometry, Path, Stroke};
use iced::widget::{canvas, column, container, row, text};
use iced::{Color, Element, Font, Length, Size, Subscription, Task, Theme};
use redis::aio::MultiplexedConnection;
use std::fmt;
use std::time::Duration;

// ─── Tunable constants ───────────────────────────────────────────────────────

/// Wheel circumference in meters. 0.557 m diameter ≈ 22" solar-car wheel.
/// speed_mph = rpm × circumference × 60 / 1609.34   (direct-drive hub motor)
const WHEEL_DIAMETER_M: f32 = 0.557;
const GEAR_RATIO: f32 = 1.0; // motor revs per wheel rev; 1.0 for hub motor
const METERS_PER_MILE: f32 = 1609.34;

const RPM_MAX: f32 = 5000.0; // gauge full-scale — set to your KLS max
const SPEED_MAX_MPH: f32 = 80.0; // gauge full-scale

const PACK_V_MIN: f32 = 84.0; // 0% SoC voltage — tune to your pack
const PACK_V_MAX: f32 = 102.0; // 100% SoC voltage
const SOC_SEGMENTS: usize = 10;

const MOTOR_TEMP_MAX: f32 = 100.0; // °C, for bar scaling / coloring
const CTRL_TEMP_MAX: f32 = 80.0;
const FET_TEMP_MAX: f32 = 90.0;

const POLL_MS: u64 = 200;

// ─── Colors ───────────────────────────────────────────────────────────────────

const BG_ROOT: Color = Color { r: 0.04, g: 0.045, b: 0.055, a: 1.0 };
const BG_PANEL: Color = Color { r: 0.075, g: 0.08, b: 0.10, a: 1.0 };
const COLOR_BORDER: Color = Color { r: 0.14, g: 0.16, b: 0.21, a: 1.0 };
const COLOR_DIM: Color = Color { r: 0.35, g: 0.38, b: 0.44, a: 1.0 };
const COLOR_TEXT: Color = Color { r: 0.88, g: 0.91, b: 0.96, a: 1.0 };
const COLOR_TRACK: Color = Color { r: 0.14, g: 0.16, b: 0.21, a: 1.0 };
const CYAN: Color = Color { r: 0.0, g: 0.85, b: 0.95, a: 1.0 };
const GREEN: Color = Color { r: 0.18, g: 0.95, b: 0.48, a: 1.0 };
const AMBER: Color = Color { r: 1.0, g: 0.72, b: 0.0, a: 1.0 };
const RED: Color = Color { r: 1.0, g: 0.22, b: 0.22, a: 1.0 };

// ─── Redis fetch ──────────────────────────────────────────────────────────────

/// TS.GET returns (timestamp_ms, value). Missing/empty key → 0.0 so a single
/// absent key never aborts the whole fetch during startup.
async fn ts_get_f64(con: &mut MultiplexedConnection, key: &str) -> f64 {
    let result: redis::RedisResult<(i64, f64)> =
        redis::cmd("TS.GET").arg(key).query_async(con).await;
    match result {
        Ok((_ts, v)) => v,
        Err(_) => 0.0,
    }
}

async fn ts_get_bool(con: &mut MultiplexedConnection, key: &str) -> bool {
    ts_get_f64(con, key).await > 0.5
}

async fn connect_redis() -> anyhow::Result<Conn> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    let con = client.get_multiplexed_async_connection().await?;
    Ok(Conn(con))
}

async fn fetch_telemetry(mut con: MultiplexedConnection) -> anyhow::Result<Telemetry> {
    let con = &mut con;

    // A cheap liveness probe: PING fails fast if the server went away, so we
    // surface a link error instead of silently rendering a frame of zeros
    // (every ts_get_f64 below swallows errors by design).
    let _: String = redis::cmd("PING").query_async(con).await?;

    // ── Kelly — Message1 ─────────────────────────────────────────────────────
    let rpm = ts_get_f64(con, "Kelly:speed_rpm").await; // FIX: was "Kelly:rpm"
    let motor_current = ts_get_f64(con, "Kelly:motor_current").await; // NEW
    let battery_voltage = ts_get_f64(con, "Kelly:battery_voltage").await;

    let faults = KellyFaults {
        over_voltage: ts_get_bool(con, "Kelly:over_voltage").await,
        low_voltage: ts_get_bool(con, "Kelly:low_voltage").await,
        stall: ts_get_bool(con, "Kelly:stall").await,
        internal_volts: ts_get_bool(con, "Kelly:internal_volts_fault").await,
        controller_over_temp: ts_get_bool(con, "Kelly:over_temperature").await,
        motor_over_temp: ts_get_bool(con, "Kelly:motor_over_temperature").await,
        throttle_error: ts_get_bool(con, "Kelly:throttle_error").await,
        hall_throttle_open: ts_get_bool(con, "Kelly:hall_throttle_open").await,
        angle_sensor: ts_get_bool(con, "Kelly:angle_sensor_error").await,
        hall_galvanometer: ts_get_bool(con, "Kelly:hall_galvanometer_error").await,
        id_error: ts_get_bool(con, "Kelly:id_error").await,
    };

    // ── Kelly — Message2 ─────────────────────────────────────────────────────
    let throttle_raw = ts_get_f64(con, "Kelly:throttle_signal").await;
    let controller_temp = ts_get_f64(con, "Kelly:controller_temperature").await;
    let motor_temp = ts_get_f64(con, "Kelly:motor_temperature").await;
    let command_status = ts_get_f64(con, "Kelly:command_status").await;
    let feedback_status = ts_get_f64(con, "Kelly:feedback_status").await;
    let brake = ts_get_bool(con, "Kelly:brake_switch").await;
    let foot_sw = ts_get_bool(con, "Kelly:foot_switch").await;
    let boost_sw = ts_get_bool(con, "Kelly:boost_switch").await;

    // Encoding written by connect_bus_1: 0=Backward, 1=Forward, 2=Neutral
    let direction = match command_status as u8 {
        0 => MotorDirection::Reverse,
        1 => MotorDirection::Forward,
        2 => MotorDirection::Neutral,
        _ => MotorDirection::Unknown,
    };
    // 0=Stationary, 1=Forward, 2=Backward
    let feedback = match feedback_status as u8 {
        0 => MotorFeedback::Stationary,
        1 => MotorFeedback::Forward,
        2 => MotorFeedback::Backward,
        _ => MotorFeedback::Unknown,
    };

    // ── MPPTs ────────────────────────────────────────────────────────────────
    let mppt = [
        fetch_mppt(con, "mppt").await,
        fetch_mppt(con, "mppt_2").await,
        fetch_mppt(con, "mppt_3").await,
    ];

    Ok(Telemetry {
        rpm: rpm as f32,
        rpm_peak: 0.0, // peak-hold maintained in update(), not here
        battery_voltage: battery_voltage as f32,
        motor_current: motor_current as f32,
        throttle: (throttle_raw / 255.0).clamp(0.0, 1.0) as f32,
        controller_temp: controller_temp as f32,
        motor_temp: motor_temp as f32,
        direction,
        feedback,
        brake,
        foot_sw,
        boost_sw,
        kelly_faults: faults,
        mppt,
    })
}

async fn fetch_mppt(con: &mut MultiplexedConnection, prefix: &str) -> MpptData {
    let input_voltage = ts_get_f64(con, &format!("{prefix}:input_voltage")).await;
    let input_current = ts_get_f64(con, &format!("{prefix}:input_current")).await;
    let output_voltage = ts_get_f64(con, &format!("{prefix}:output_voltage")).await;
    let output_current = ts_get_f64(con, &format!("{prefix}:output_current")).await;
    let mosfet_temp = ts_get_f64(con, &format!("{prefix}:mosfet_temperature")).await;

    let fault_keys = [
        "error_mosfet_overheat",
        "error_low_arrow_power",
        "error_hw_over_voltage",
        "error_hw_over_current",
        "error_battery_low",
        "error_battery_full",
    ];
    let limit_keys = [
        "limit_output_voltage_max",
        "limit_mosfet_temperature",
        "limit_local_mppt",
        "limit_input_current_min",
        "limit_input_current_max",
        "limit_global_mppt",
        "limit_duty_cycle_max",
        "limit_duty_cycle_min",
    ];

    let mut fault = false;
    for key in fault_keys {
        fault |= ts_get_bool(con, &format!("{prefix}:{key}")).await;
    }
    let mut limiting = false;
    for key in limit_keys {
        limiting |= ts_get_bool(con, &format!("{prefix}:{key}")).await;
    }
    let aux_fault = ts_get_bool(con, &format!("{prefix}:error_12v_undervoltage")).await;

    MpptData {
        watts_in: (input_voltage * input_current) as f32,
        watts_out: (output_voltage * output_current) as f32,
        mosfet_temp: mosfet_temp as f32,
        fault,
        limiting,
        aux_fault,
    }
}

// ─── Telemetry model ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum MotorDirection {
    Forward,
    #[default]
    Neutral,
    Reverse,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum MotorFeedback {
    #[default]
    Stationary,
    Forward,
    Backward,
    Unknown,
}

#[derive(Debug, Clone, Copy, Default)]
struct KellyFaults {
    over_voltage: bool,
    low_voltage: bool,
    stall: bool,
    internal_volts: bool,
    controller_over_temp: bool,
    motor_over_temp: bool,
    throttle_error: bool,
    hall_throttle_open: bool,
    angle_sensor: bool,
    hall_galvanometer: bool,
    id_error: bool,
}

impl KellyFaults {
    fn rows(&self) -> [(&'static str, bool); 11] {
        [
            ("Over Voltage", self.over_voltage),
            ("Low Voltage", self.low_voltage),
            ("Stall", self.stall),
            ("Internal Volts", self.internal_volts),
            ("Ctrl Over Temp", self.controller_over_temp),
            ("Motor Over Temp", self.motor_over_temp),
            ("Throttle Error", self.throttle_error),
            ("Hall Throttle", self.hall_throttle_open),
            ("Angle Sensor", self.angle_sensor),
            ("Hall Galvo", self.hall_galvanometer),
            ("ID Error", self.id_error),
        ]
    }
    fn any(&self) -> bool {
        self.rows().iter().any(|(_, a)| *a)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct MpptData {
    watts_in: f32,
    watts_out: f32,
    mosfet_temp: f32,
    fault: bool,
    limiting: bool,
    aux_fault: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MpptStatus {
    Ok,
    Limiting,
    Fault,
}

impl MpptData {
    fn status(&self) -> MpptStatus {
        if self.fault || self.aux_fault {
            MpptStatus::Fault
        } else if self.limiting {
            MpptStatus::Limiting
        } else {
            MpptStatus::Ok
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Telemetry {
    rpm: f32,
    rpm_peak: f32,
    battery_voltage: f32,
    motor_current: f32,
    throttle: f32, // normalized 0–1
    controller_temp: f32,
    motor_temp: f32,
    direction: MotorDirection,
    feedback: MotorFeedback,
    brake: bool,
    foot_sw: bool,
    boost_sw: bool,
    kelly_faults: KellyFaults,
    mppt: [MpptData; 3],
}

impl Telemetry {
    fn speed_mph(&self) -> f32 {
        let wheel_rpm = self.rpm / GEAR_RATIO;
        wheel_rpm * WHEEL_DIAMETER_M * std::f32::consts::PI * 60.0 / METERS_PER_MILE
    }
    /// Power coming off the array (sum of MPPT inputs).
    fn solar_in(&self) -> f32 {
        self.mppt.iter().map(|m| m.watts_in).sum()
    }
    /// Power the MPPTs are pushing into the pack.
    fn charge_out(&self) -> f32 {
        self.mppt.iter().map(|m| m.watts_out).sum()
    }
    /// Power the motor controller is pulling from the pack.
    fn motor_draw(&self) -> f32 {
        self.battery_voltage * self.motor_current
    }
    /// Positive = pack charging, negative = pack discharging.
    fn net_pack(&self) -> f32 {
        self.charge_out() - self.motor_draw()
    }
    fn any_mppt_problem(&self) -> bool {
        self.mppt.iter().any(|m| m.status() != MpptStatus::Ok)
    }
    fn any_fault(&self) -> bool {
        self.kelly_faults.any() || self.mppt.iter().any(|m| m.fault || m.aux_fault)
    }
}

// ─── App ──────────────────────────────────────────────────────────────────────

/// Wrapper so Message can derive Debug (MultiplexedConnection isn't Debug).
#[derive(Clone)]
struct Conn(MultiplexedConnection);

impl fmt::Debug for Conn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RedisConn")
    }
}

#[derive(Debug, Default)]
struct Dashboard {
    con: Option<Conn>,
    connecting: bool,
    link_ok: bool,
    data: Telemetry,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Connected(Result<Conn, String>),
    TelemetryLoaded(Result<Telemetry, String>),
}

impl Dashboard {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Tick => {
                if let Some(conn) = &self.con {
                    // Reuse the persistent connection; clones of a
                    // MultiplexedConnection share the underlying socket.
                    let con = conn.0.clone();
                    Task::perform(fetch_telemetry(con), |r| {
                        Message::TelemetryLoaded(r.map_err(|e| e.to_string()))
                    })
                } else if !self.connecting {
                    self.connecting = true;
                    Task::perform(connect_redis(), |r| {
                        Message::Connected(r.map_err(|e| e.to_string()))
                    })
                } else {
                    Task::none()
                }
            }
            Message::Connected(Ok(conn)) => {
                self.con = Some(conn);
                self.connecting = false;
                Task::none()
            }
            Message::Connected(Err(e)) => {
                eprintln!("redis connect error: {e}");
                self.connecting = false;
                self.link_ok = false;
                Task::none()
            }
            Message::TelemetryLoaded(Ok(mut fresh)) => {
                // Peak-hold: carry the running RPM maximum across ticks.
                fresh.rpm_peak = self.data.rpm_peak.max(fresh.rpm);
                self.data = fresh;
                self.link_ok = true;
                Task::none()
            }
            Message::TelemetryLoaded(Err(e)) => {
                // Keep the last good frame on screen, flag the link, and drop
                // the connection so the next tick re-establishes it.
                eprintln!("telemetry fetch error: {e}");
                self.link_ok = false;
                self.con = None;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(POLL_MS)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Message> {
        let d = &self.data;
        let speed = d.speed_mph();

        // ── Row 1: big gauges ────────────────────────────────────────────────
        let gauges = row![
            gauge_panel(
                "MOTOR RPM",
                d.rpm,
                0.0,
                RPM_MAX,
                &format!("{:.0}", d.rpm),
                level_color(d.rpm / RPM_MAX),
                Some(d.rpm_peak / RPM_MAX),
            ),
            gauge_panel(
                "SPEED",
                speed,
                0.0,
                SPEED_MAX_MPH,
                &format!("{:.0} mph", speed),
                level_color(speed / SPEED_MAX_MPH),
                None,
            ),
            soc_panel(d.battery_voltage),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(5));

        // ── Row 2: power flow  array → pack → motor ─────────────────────────
        let net = d.net_pack();
        let power_strip = row![
            power_panel("SOLAR ARRAY", d.solar_in(), GREEN, false),
            power_panel("CHARGE → PACK", d.charge_out(), GREEN, false),
            power_panel("MOTOR DRAW", d.motor_draw(), CYAN, false),
            power_panel(
                "NET PACK",
                net.abs(),
                if net >= 0.0 { GREEN } else { RED },
                net < 0.0,
            ),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(2));

        // ── Row 3: per-MPPT mini panels ──────────────────────────────────────
        let mppt_strip = row![
            mppt_panel(1, &d.mppt[0]),
            mppt_panel(2, &d.mppt[1]),
            mppt_panel(3, &d.mppt[2]),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(2));

        // ── Row 4: drive status ──────────────────────────────────────────────
        let status = row![
            status_panel(
                "THROTTLE",
                &format!("{:.0}%", d.throttle * 100.0),
                d.throttle,
                CYAN,
            ),
            direction_panel(d.direction, d.feedback),
            switches_panel(d.brake, d.foot_sw, d.boost_sw),
            status_panel(
                "MOTOR TEMP",
                &format!("{:.0}°C", d.motor_temp),
                d.motor_temp / MOTOR_TEMP_MAX,
                level_color(d.motor_temp / MOTOR_TEMP_MAX),
            ),
            status_panel(
                "CTRL TEMP",
                &format!("{:.0}°C", d.controller_temp),
                d.controller_temp / CTRL_TEMP_MAX,
                level_color(d.controller_temp / CTRL_TEMP_MAX),
            ),
        ]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::FillPortion(2));

        let main_col = column![gauges, power_strip, mppt_strip, status]
            .spacing(8)
            .width(Length::FillPortion(4))
            .height(Length::Fill);

        // ── Right column: link state + fault list ────────────────────────────
        let any = d.any_fault();
        let kelly_any = d.kelly_faults.any();

        let mut fault_col = column![
            link_row(self.link_ok),
            fault_header(any),
            fault_section_label("KELLY"),
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

        for (label, active) in d.kelly_faults.rows() {
            fault_col = fault_col.push(fault_row(label, active));
        }
        if !kelly_any {
            // (all rows render dim when clear — list stays put so faults are
            // easy to spot at a glance while driving)
        }

        fault_col = fault_col.push(fault_section_label("MPPT"));
        for (i, m) in d.mppt.iter().enumerate() {
            fault_col = fault_col.push(mppt_status_row(i + 1, m.status(), m.aux_fault));
        }

        let fault_box = container(fault_col)
            .style(move |_: &Theme| container::Style {
                background: Some(iced::Background::Color(if any {
                    Color { r: 0.12, g: 0.03, b: 0.03, a: 1.0 }
                } else {
                    BG_PANEL
                })),
                border: iced::Border {
                    color: if any { RED } else { COLOR_BORDER },
                    width: if any { 1.5 } else { 1.0 },
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .width(Length::FillPortion(1))
            .height(Length::Fill);

        let root = row![main_col, fault_box]
            .spacing(8)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill);

        container(root)
            .style(|_: &Theme| container::Style {
                background: Some(iced::Background::Color(BG_ROOT)),
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// ─── Color helpers ────────────────────────────────────────────────────────────

fn level_color(frac: f32) -> Color {
    if frac >= 0.85 {
        RED
    } else if frac >= 0.65 {
        AMBER
    } else {
        CYAN
    }
}

// ─── Shared canvas helpers ────────────────────────────────────────────────────

fn draw_arc(
    frame: &mut Frame,
    cx: f32,
    cy: f32,
    r: f32,
    start: f32,
    end: f32,
    color: Color,
    width: f32,
    steps: usize,
) {
    let mut b = canvas::path::Builder::new();
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let a = start + (end - start) * t;
        let p = iced::Point::new(cx + a.cos() * r, cy + a.sin() * r);
        if i == 0 {
            b.move_to(p);
        } else {
            b.line_to(p);
        }
    }
    frame.stroke(
        &b.build(),
        Stroke::default().with_color(color).with_width(width),
    );
}

fn canvas_text(
    content: String,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    ax: iced::alignment::Horizontal,
    ay: iced::alignment::Vertical,
) -> canvas::Text {
    canvas::Text {
        content,
        position: iced::Point::new(x, y),
        color,
        size: iced::Pixels(size),
        font: Font::MONOSPACE,
        horizontal_alignment: ax,
        vertical_alignment: ay,
        ..canvas::Text::default()
    }
}

fn panel_container<'a>(
    c: impl canvas::Program<Message> + 'a,
    portion: u16,
    h: Length,
) -> Element<'a, Message> {
    container(canvas(c).width(Length::Fill).height(Length::Fill))
        .style(|_: &Theme| container::Style {
            background: Some(iced::Background::Color(BG_PANEL)),
            border: iced::Border {
                color: COLOR_BORDER,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .width(Length::FillPortion(portion))
        .height(h)
        .padding(8)
        .into()
}

// ─── Gauge Canvas ─────────────────────────────────────────────────────────────

struct GaugeCanvas {
    label: String,
    value: f32,
    min: f32,
    max: f32,
    display: String,
    color: Color,
    peak_frac: Option<f32>,
}

impl<Message> canvas::Program<Message> for GaugeCanvas {
    type State = ();
    fn draw(
        &self,
        _: &(),
        renderer: &iced::Renderer,
        _: &Theme,
        bounds: iced::Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let cx = bounds.width / 2.0;
            let cy = bounds.height * 0.50;
            let r = (bounds.width.min(bounds.height) * 0.38).min(cy - 12.0);
            let start: f32 = std::f32::consts::PI * (5.0 / 4.0);
            let sweep: f32 = std::f32::consts::PI * 1.5;
            let frac = ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0);

            draw_arc(
                frame,
                cx,
                cy,
                r,
                start,
                start + sweep,
                Color { r: 0.18, g: 0.20, b: 0.26, a: 1.0 },
                7.0,
                120,
            );
            if frac > 0.005 {
                draw_arc(frame, cx, cy, r, start, start + sweep * frac, self.color, 7.0, 120);
            }
            for i in 0..=10 {
                let a = start + sweep * (i as f32 / 10.0);
                let major = i % 5 == 0;
                let ir = if major { r - 13.0 } else { r - 7.0 };
                frame.stroke(
                    &Path::line(
                        iced::Point::new(cx + a.cos() * ir, cy + a.sin() * ir),
                        iced::Point::new(cx + a.cos() * (r + 1.0), cy + a.sin() * (r + 1.0)),
                    ),
                    Stroke::default()
                        .with_color(Color { r: 0.4, g: 0.42, b: 0.5, a: 0.6 })
                        .with_width(if major { 2.0 } else { 1.0 }),
                );
            }
            let na = start + sweep * frac;
            frame.stroke(
                &Path::line(
                    iced::Point::new(cx, cy),
                    iced::Point::new(cx + na.cos() * (r - 4.0), cy + na.sin() * (r - 4.0)),
                ),
                Stroke::default().with_color(Color::WHITE).with_width(2.0),
            );
            if let Some(pf) = self.peak_frac {
                let pa = start + sweep * pf.clamp(0.0, 1.0);
                frame.stroke(
                    &Path::line(
                        iced::Point::new(cx + pa.cos() * (r - 14.0), cy + pa.sin() * (r - 14.0)),
                        iced::Point::new(cx + pa.cos() * (r + 1.0), cy + pa.sin() * (r + 1.0)),
                    ),
                    Stroke::default().with_color(AMBER).with_width(2.5),
                );
            }
            frame.fill(
                &Path::circle(iced::Point::new(cx, cy), 5.0),
                canvas::Fill {
                    style: canvas::Style::Solid(self.color),
                    ..canvas::Fill::default()
                },
            );
            frame.fill_text(canvas_text(
                self.display.clone(),
                cx,
                cy + r * 0.42,
                20.0,
                COLOR_TEXT,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Center,
            ));
            frame.fill_text(canvas_text(
                self.label.clone(),
                cx,
                bounds.height - 5.0,
                10.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Bottom,
            ));
        });
        vec![geom]
    }
}

fn gauge_panel<'a>(
    label: &str,
    value: f32,
    min: f32,
    max: f32,
    display: &str,
    color: Color,
    peak_frac: Option<f32>,
) -> Element<'a, Message> {
    panel_container(
        GaugeCanvas {
            label: label.to_string(),
            value,
            min,
            max,
            display: display.to_string(),
            color,
            peak_frac,
        },
        1,
        Length::Fill,
    )
}

// ─── SoC Canvas ───────────────────────────────────────────────────────────────

struct SocCanvas {
    voltage: f32,
    soc: f32,
}

impl<Message> canvas::Program<Message> for SocCanvas {
    type State = ();
    fn draw(
        &self,
        _: &(),
        renderer: &iced::Renderer,
        _: &Theme,
        bounds: iced::Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let cx = w / 2.0;
            let body_w = w * 0.42;
            let body_h = h * 0.58;
            let body_x = cx - body_w / 2.0;
            let body_y = h * 0.14;
            let nub_w = body_w * 0.3;
            frame.fill_rectangle(
                iced::Point::new(cx - nub_w / 2.0, body_y - 6.0),
                iced::Size::new(nub_w, 6.0),
                Color { r: 0.25, g: 0.27, b: 0.32, a: 1.0 },
            );
            frame.stroke(
                &Path::rectangle(
                    iced::Point::new(body_x, body_y),
                    iced::Size::new(body_w, body_h),
                ),
                Stroke::default().with_color(COLOR_BORDER).with_width(1.5),
            );
            let seg_color = if self.soc > 0.5 {
                GREEN
            } else if self.soc > 0.2 {
                AMBER
            } else {
                RED
            };
            let seg_margin = 3.0;
            let seg_h = (body_h - seg_margin * (SOC_SEGMENTS as f32 + 1.0)) / SOC_SEGMENTS as f32;
            let filled = (self.soc * SOC_SEGMENTS as f32).round() as usize;
            for i in 0..SOC_SEGMENTS {
                let seg_y = body_y + body_h - seg_margin - (i as f32 + 1.0) * (seg_h + seg_margin)
                    + seg_margin;
                frame.fill_rectangle(
                    iced::Point::new(body_x + seg_margin, seg_y),
                    iced::Size::new(body_w - seg_margin * 2.0, seg_h),
                    if i < filled { seg_color } else { COLOR_TRACK },
                );
            }
            frame.fill_text(canvas_text(
                format!("{:.0}%", self.soc * 100.0),
                cx,
                body_y + body_h + 10.0,
                18.0,
                seg_color,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Top,
            ));
            frame.fill_text(canvas_text(
                format!("{:.1}V", self.voltage),
                cx,
                body_y + body_h + 28.0,
                10.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Top,
            ));
            frame.fill_text(canvas_text(
                "BATTERY SOC".to_string(),
                cx,
                h - 5.0,
                10.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Bottom,
            ));
        });
        vec![geom]
    }
}

fn soc_panel<'a>(voltage: f32) -> Element<'a, Message> {
    let soc = ((voltage - PACK_V_MIN) / (PACK_V_MAX - PACK_V_MIN)).clamp(0.0, 1.0);
    panel_container(SocCanvas { voltage, soc }, 1, Length::Fill)
}

// ─── Power Strip Canvas ───────────────────────────────────────────────────────

struct PowerCanvas {
    label: String,
    watts: f32,
    color: Color,
    negative: bool,
}

impl<Message> canvas::Program<Message> for PowerCanvas {
    type State = ();
    fn draw(
        &self,
        _: &(),
        renderer: &iced::Renderer,
        _: &Theme,
        bounds: iced::Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let prefix = if self.negative { "−" } else { "" };
            frame.fill_text(canvas_text(
                format!("{}{:.0} W", prefix, self.watts),
                w / 2.0,
                h / 2.0 + 2.0,
                21.0,
                self.color,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Center,
            ));
            frame.fill_text(canvas_text(
                self.label.clone(),
                0.0,
                0.0,
                10.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Left,
                iced::alignment::Vertical::Top,
            ));
        });
        vec![geom]
    }
}

fn power_panel<'a>(label: &str, watts: f32, color: Color, negative: bool) -> Element<'a, Message> {
    panel_container(
        PowerCanvas {
            label: label.to_string(),
            watts,
            color,
            negative,
        },
        1,
        Length::Fill,
    )
}

// ─── MPPT mini-panel Canvas ───────────────────────────────────────────────────

struct MpptCanvas {
    index: usize,
    data: MpptData,
}

impl<Message> canvas::Program<Message> for MpptCanvas {
    type State = ();
    fn draw(
        &self,
        _: &(),
        renderer: &iced::Renderer,
        _: &Theme,
        bounds: iced::Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let m = &self.data;

            frame.fill_text(canvas_text(
                format!("MPPT {}", self.index),
                0.0,
                0.0,
                10.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Left,
                iced::alignment::Vertical::Top,
            ));

            // status dot + word, top-right
            let (dot_color, word) = match m.status() {
                MpptStatus::Fault => (RED, "FAULT"),
                MpptStatus::Limiting => (AMBER, "LIMIT"),
                MpptStatus::Ok => (GREEN, "OK"),
            };
            frame.fill(
                &Path::circle(iced::Point::new(w - 8.0, 9.0), 4.0),
                canvas::Fill {
                    style: canvas::Style::Solid(dot_color),
                    ..canvas::Fill::default()
                },
            );
            frame.fill_text(canvas_text(
                word.to_string(),
                w - 16.0,
                9.0,
                9.0,
                dot_color,
                iced::alignment::Horizontal::Right,
                iced::alignment::Vertical::Center,
            ));

            // watts in (big) and watts out (small)
            frame.fill_text(canvas_text(
                format!("{:.0} W", m.watts_in),
                w / 2.0,
                h * 0.46,
                18.0,
                GREEN,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Center,
            ));
            frame.fill_text(canvas_text(
                format!("→ {:.0} W out", m.watts_out),
                w / 2.0,
                h * 0.46 + 16.0,
                10.0,
                CYAN,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Center,
            ));

            // FET temp bottom-left, 12V aux flag bottom-right
            frame.fill_text(canvas_text(
                format!("FET {:.0}°C", m.mosfet_temp),
                0.0,
                h,
                9.0,
                level_color(m.mosfet_temp / FET_TEMP_MAX),
                iced::alignment::Horizontal::Left,
                iced::alignment::Vertical::Bottom,
            ));
            if m.aux_fault {
                frame.fill_text(canvas_text(
                    "12V!".to_string(),
                    w,
                    h,
                    9.0,
                    RED,
                    iced::alignment::Horizontal::Right,
                    iced::alignment::Vertical::Bottom,
                ));
            }
        });
        vec![geom]
    }
}

fn mppt_panel<'a>(index: usize, data: &MpptData) -> Element<'a, Message> {
    panel_container(MpptCanvas { index, data: *data }, 1, Length::Fill)
}

// ─── Status Canvas (label + value + bar) ──────────────────────────────────────

struct StatusCanvas {
    label: String,
    val_str: String,
    fill: f32,
    color: Color,
}

impl<Message> canvas::Program<Message> for StatusCanvas {
    type State = ();
    fn draw(
        &self,
        _: &(),
        renderer: &iced::Renderer,
        _: &Theme,
        bounds: iced::Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let bar_h = 5.0;
            let bar_y = h - bar_h - 2.0;
            frame.fill_rectangle(
                iced::Point::new(0.0, bar_y),
                iced::Size::new(w, bar_h),
                COLOR_TRACK,
            );
            if self.fill > 0.01 {
                frame.fill_rectangle(
                    iced::Point::new(0.0, bar_y),
                    iced::Size::new(w * self.fill, bar_h),
                    self.color,
                );
            }
            frame.fill_text(canvas_text(
                self.label.clone(),
                0.0,
                0.0,
                10.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Left,
                iced::alignment::Vertical::Top,
            ));
            frame.fill_text(canvas_text(
                self.val_str.clone(),
                w / 2.0,
                (h - bar_h - 6.0) / 2.0 + 2.0,
                17.0,
                self.color,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Center,
            ));
        });
        vec![geom]
    }
}

fn status_panel<'a>(label: &str, val: &str, fill: f32, color: Color) -> Element<'a, Message> {
    panel_container(
        StatusCanvas {
            label: label.to_string(),
            val_str: val.to_string(),
            fill: fill.clamp(0.0, 1.0),
            color,
        },
        1,
        Length::Fill,
    )
}

// ─── Direction Canvas ─────────────────────────────────────────────────────────

struct DirectionCanvas {
    direction: MotorDirection,
    feedback: MotorFeedback,
}

impl<Message> canvas::Program<Message> for DirectionCanvas {
    type State = ();
    fn draw(
        &self,
        _: &(),
        renderer: &iced::Renderer,
        _: &Theme,
        bounds: iced::Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let cx = w / 2.0;
            let (label, color) = match self.direction {
                MotorDirection::Forward => ("▶ FWD", CYAN),
                MotorDirection::Neutral => ("■ NEUT", AMBER),
                MotorDirection::Reverse => ("◀ REV", RED),
                MotorDirection::Unknown => ("? ----", COLOR_DIM),
            };
            let pill_w = w * 0.82;
            let pill_h = h * 0.42;
            let pill_x = cx - pill_w / 2.0;
            let pill_y = h * 0.18;
            frame.fill_rectangle(
                iced::Point::new(pill_x, pill_y),
                iced::Size::new(pill_w, pill_h),
                Color {
                    r: color.r * 0.15,
                    g: color.g * 0.15,
                    b: color.b * 0.15,
                    a: 1.0,
                },
            );
            frame.stroke(
                &Path::rectangle(
                    iced::Point::new(pill_x, pill_y),
                    iced::Size::new(pill_w, pill_h),
                ),
                Stroke::default().with_color(color).with_width(1.5),
            );
            frame.fill_text(canvas_text(
                label.to_string(),
                cx,
                pill_y + pill_h / 2.0,
                14.0,
                color,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Center,
            ));

            // actual motion reported by the controller (feedback_status)
            let fb = match self.feedback {
                MotorFeedback::Stationary => "fb: STILL",
                MotorFeedback::Forward => "fb: FWD",
                MotorFeedback::Backward => "fb: REV",
                MotorFeedback::Unknown => "fb: ?",
            };
            frame.fill_text(canvas_text(
                fb.to_string(),
                cx,
                h - 2.0,
                9.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Center,
                iced::alignment::Vertical::Bottom,
            ));
            frame.fill_text(canvas_text(
                "DIRECTION".to_string(),
                0.0,
                0.0,
                10.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Left,
                iced::alignment::Vertical::Top,
            ));
        });
        vec![geom]
    }
}

fn direction_panel<'a>(direction: MotorDirection, feedback: MotorFeedback) -> Element<'a, Message> {
    panel_container(DirectionCanvas { direction, feedback }, 1, Length::Fill)
}

// ─── Switches Canvas (brake / foot / boost) ───────────────────────────────────

struct SwitchesCanvas {
    brake: bool,
    foot: bool,
    boost: bool,
}

impl<Message> canvas::Program<Message> for SwitchesCanvas {
    type State = ();
    fn draw(
        &self,
        _: &(),
        renderer: &iced::Renderer,
        _: &Theme,
        bounds: iced::Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let cache = Cache::default();
        let geom = cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let w = bounds.width;
            let h = bounds.height;
            let items: [(&str, bool, Color); 3] = [
                ("BRK", self.brake, RED),
                ("FOOT", self.foot, CYAN),
                ("BST", self.boost, AMBER),
            ];
            let gap = 5.0;
            let pill_w = (w - gap * (items.len() as f32 - 1.0)) / items.len() as f32;
            let pill_h = h * 0.42;
            let pill_y = h * 0.30;
            for (i, (label, on, color)) in items.iter().enumerate() {
                let x = i as f32 * (pill_w + gap);
                let c = if *on { *color } else { COLOR_DIM };
                if *on {
                    frame.fill_rectangle(
                        iced::Point::new(x, pill_y),
                        iced::Size::new(pill_w, pill_h),
                        Color {
                            r: color.r * 0.18,
                            g: color.g * 0.18,
                            b: color.b * 0.18,
                            a: 1.0,
                        },
                    );
                }
                frame.stroke(
                    &Path::rectangle(
                        iced::Point::new(x, pill_y),
                        iced::Size::new(pill_w, pill_h),
                    ),
                    Stroke::default()
                        .with_color(c)
                        .with_width(if *on { 1.5 } else { 1.0 }),
                );
                frame.fill_text(canvas_text(
                    label.to_string(),
                    x + pill_w / 2.0,
                    pill_y + pill_h / 2.0,
                    11.0,
                    c,
                    iced::alignment::Horizontal::Center,
                    iced::alignment::Vertical::Center,
                ));
            }
            frame.fill_text(canvas_text(
                "SWITCHES".to_string(),
                0.0,
                0.0,
                10.0,
                COLOR_DIM,
                iced::alignment::Horizontal::Left,
                iced::alignment::Vertical::Top,
            ));
        });
        vec![geom]
    }
}

fn switches_panel<'a>(brake: bool, foot: bool, boost: bool) -> Element<'a, Message> {
    panel_container(SwitchesCanvas { brake, foot, boost }, 1, Length::Fill)
}

// ─── Fault / link column widgets ──────────────────────────────────────────────

fn link_row<'a>(ok: bool) -> Element<'a, Message> {
    let (dot, label, col) = if ok {
        ("●", "LINK OK", GREEN)
    } else {
        ("●", "REDIS DOWN", RED)
    };
    container(
        row![
            text(dot).font(Font::MONOSPACE).size(10).color(col),
            text(label)
                .font(Font::MONOSPACE)
                .size(10)
                .color(col)
                .width(Length::Fill),
        ]
        .spacing(5),
    )
    .padding(iced::Padding {
        top: 8.0,
        right: 8.0,
        bottom: 2.0,
        left: 8.0,
    })
    .width(Length::Fill)
    .into()
}

fn fault_header<'a>(any: bool) -> Element<'a, Message> {
    container(
        text(if any { "⚡ FAULTS" } else { "FAULTS" })
            .font(Font::MONOSPACE)
            .size(11)
            .color(if any { RED } else { COLOR_DIM }),
    )
    .padding(iced::Padding {
        top: 4.0,
        right: 8.0,
        bottom: 2.0,
        left: 8.0,
    })
    .width(Length::Fill)
    .into()
}

fn fault_section_label<'a>(label: &'a str) -> Element<'a, Message> {
    container(text(label).font(Font::MONOSPACE).size(9).color(COLOR_DIM))
        .padding(iced::Padding {
            top: 5.0,
            right: 8.0,
            bottom: 1.0,
            left: 8.0,
        })
        .width(Length::Fill)
        .into()
}

fn fault_row<'a>(label: &'a str, active: bool) -> Element<'a, Message> {
    let dot = if active { "●" } else { "○" };
    let col = if active { RED } else { COLOR_DIM };
    container(
        row![
            text(dot).font(Font::MONOSPACE).size(9).color(col),
            text(label)
                .font(Font::MONOSPACE)
                .size(9)
                .color(col)
                .width(Length::Fill),
        ]
        .spacing(5),
    )
    .style(move |_: &Theme| container::Style {
        background: Some(iced::Background::Color(if active {
            Color { r: 0.35, g: 0.0, b: 0.0, a: 0.18 }
        } else {
            Color::TRANSPARENT
        })),
        ..Default::default()
    })
    .padding([3, 8])
    .width(Length::Fill)
    .into()
}

fn mppt_status_row<'a>(index: usize, status: MpptStatus, aux: bool) -> Element<'a, Message> {
    let (word, col) = match status {
        MpptStatus::Fault => {
            if aux {
                ("FAULT+12V", RED)
            } else {
                ("FAULT", RED)
            }
        }
        MpptStatus::Limiting => ("LIMIT", AMBER),
        MpptStatus::Ok => ("OK", COLOR_DIM),
    };
    let dot = if status == MpptStatus::Ok { "○" } else { "●" };
    container(
        row![
            text(dot).font(Font::MONOSPACE).size(9).color(col),
            text(format!("MPPT {index}"))
                .font(Font::MONOSPACE)
                .size(9)
                .color(col)
                .width(Length::Fill),
            text(word).font(Font::MONOSPACE).size(9).color(col),
        ]
        .spacing(5),
    )
    .style(move |_: &Theme| container::Style {
        background: Some(iced::Background::Color(match status {
            MpptStatus::Fault => Color { r: 0.35, g: 0.0, b: 0.0, a: 0.18 },
            MpptStatus::Limiting => Color { r: 0.35, g: 0.25, b: 0.0, a: 0.15 },
            MpptStatus::Ok => Color::TRANSPARENT,
        })),
        ..Default::default()
    })
    .padding([3, 8])
    .width(Length::Fill)
    .into()
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn dashboard_theme(_: &Dashboard) -> Theme {
    Theme::Dark
}

// Note: NO #[tokio::main] here. iced::application().run() manages the executor.
fn main() -> iced::Result {
    iced::application("Solar Car · Dashboard", Dashboard::update, Dashboard::view)
        .subscription(Dashboard::subscription)
        .theme(dashboard_theme)
        .window(iced::window::Settings {
            size: Size::new(800.0, 480.0),
            resizable: false,
            ..Default::default()
        })
        .run()
}