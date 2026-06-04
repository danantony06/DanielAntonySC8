use crate::messages_drivercontroller::CanError;
use crate::messages_kelly::Message2FeedbackStatus;
use crate::messages_mppt;
use crate::messages_mppt::CanError as OtherCanError;
use crate::messages_mppt_2;
use crate::messages_mppt_3;
use anyhow::{Context, Result};
use embedded_can::Frame;
use iovec::IoVec;
use nix::cmsg_space;
use nix::sys::socket::RecvMsg;
use nix::sys::socket::{ControlMessageOwned, MsgFlags, recvmsg, setsockopt, sockopt};
use nix::sys::time::TimeSpec;
use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;
use redis::cmd;
use socketcan::{
    CanAnyFrame, CanFdSocket, CanFrame, CanSocket, ExtendedId, Frame as OtherFrame, Id, Socket,
    StandardId, dump::Reader,};
use std::fmt;
use std::io::IoSliceMut;
use std::os::fd::{AsRawFd, RawFd};
use tokio::io::Interest;
use tokio::io::unix::AsyncFd;
use tracing::{error, info};
use tracing_subscriber;

pub async fn connect_bus_2() -> anyhow::Result<()> {
    let connection = "redis://127.0.0.1";
    let client = redis::Client::open(connection)?;
    let mut con: redis::aio::MultiplexedConnection =
        client.get_multiplexed_async_connection().await?;
    let sock_rx = match socketcan::CanSocket::open("can1") {// TODO- Make socket a runtime Flag. bus 2 in can1
        Ok(s) => {
            info!("Succesfully connected to mppt bus");
            s
        }
        Err(e) => {
            error!("Failed to make socket connection:{}", e);
            return Err(e).context("Failed to connect to the mppt bus");
        }
    };

    sock_rx.set_nonblocking(true)?;

    let timestamp = true;

    if let Err(e) = setsockopt(
        &sock_rx,
        nix::sys::socket::sockopt::ReceiveTimestamp,
        &timestamp,
    ) {
        error!("Nix error during setsock, {}", e);
    }

    let async_fd = AsyncFd::new(sock_rx)?;

    let flags = MsgFlags::empty();

    let keys = [
        "mppt:input_voltage",
        "mppt:input_current", // PowerInput
        "mppt:output_voltage",
        "mppt:output_current", //PowerOutput
        "mppt:mosfet_temperature",
        "mppt:controller_temperature", // Temperature
        "mppt:twelve_volt",
        "mppt:three_volt", // AuxillaryPowerSupply
        "mppt:max_output_voltage",
        "mppt:max_input_current", //Limits
        "mppt:error_mosfet_overheat",
        "mppt:error_low_arrow_power",
        "mppt:error_hw_over_voltage",
        "mppt:error_hw_over_current",
        "mppt:error_battery_low",
        "mppt:error_battery_full", //Status
        "mppt:error_12v_undervoltage",
        "mppt:limit_output_voltage_max",
        "mppt:limit_mosfet_temperature",
        "mppt:limit_local_mppt",
        "mppt:limit_input_current_min",
        "mppt:limit_input_current_max",
        "mppt:limit_global_mppt",
        "mppt:limit_duty_cycle_max",
        "mppt:limit_duty_cycle_min",
        "mppt:can_rx_error_counter",
        "mppt:can_tx_error_counter",
        "mppt:can_tx_overflow_counter",
        "mppt:mode",
        "mppt:test_counter",
        "mppt:output_voltage_battery_side",
        "mppt:power_connector_temp", // PowerConnector
        //MPPT NUMBER 2//
        "mppt_2:input_voltage",
        "mppt_2:input_current", // PowerInput
        "mppt_2:output_voltage",
        "mppt_2:output_current", //PowerOutput
        "mppt_2:mosfet_temperature",
        "mppt_2:controller_temperature", // Temperature
        "mppt_2:twelve_volt",
        "mppt_2:three_volt", // AuxillaryPowerSupply
        "mppt_2:max_output_voltage",
        "mppt_2:max_input_current", //Limits
        "mppt_2:error_mosfet_overheat",
        "mppt_2:error_low_arrow_power",
        "mppt_2:error_hw_over_voltage",
        "mppt_2:error_hw_over_current",
        "mppt_2:error_battery_low",
        "mppt_2:error_battery_full", //Status
        "mppt_2:error_12v_undervoltage",
        "mppt_2:limit_output_voltage_max",
        "mppt_2:limit_mosfet_temperature",
        "mppt_2:limit_local_mppt",
        "mppt_2:limit_input_current_min",
        "mppt_2:limit_input_current_max",
        "mppt_2:limit_global_mppt",
        "mppt_2:limit_duty_cycle_max",
        "mppt_2:limit_duty_cycle_min",
        "mppt_2:can_rx_error_counter",
        "mppt_2:can_tx_error_counter",
        "mppt_2:can_tx_overflow_counter",
        "mppt_2:mode",
        "mppt_2:test_counter",
        "mppt_2:output_voltage_battery_side",
        "mppt_2:power_connector_temp", // PowerConnector
        //MPPT NUMBER 3//
        "mppt_3:input_voltage",
        "mppt_3:input_current", // PowerInput
        "mppt_3:output_voltage",
        "mppt_3:output_current", //PowerOutput
        "mppt_3:mosfet_temperature",
        "mppt_3:controller_temperature", // Temperature
        "mppt_3:twelve_volt",
        "mppt_3:three_volt", // AuxillaryPowerSupply
        "mppt_3:max_output_voltage",
        "mppt_3:max_input_current", //Limits
        "mppt_3:error_mosfet_overheat",
        "mppt_3:error_low_arrow_power",
        "mppt_3:error_hw_over_voltage",
        "mppt_3:error_hw_over_current",
        "mppt_3:error_battery_low",
        "mppt_3:error_battery_full", //Status
        "mppt_3:error_12v_undervoltage",
        "mppt_3:limit_output_voltage_max",
        "mppt_3:limit_mosfet_temperature",
        "mppt_3:limit_local_mppt",
        "mppt_3:limit_input_current_min",
        "mppt_3:limit_input_current_max",
        "mppt_3:limit_global_mppt",
        "mppt_3:limit_duty_cycle_max",
        "mppt_3:limit_duty_cycle_min",
        "mppt_3:can_rx_error_counter",
        "mppt_3:can_tx_error_counter",
        "mppt_3:can_tx_overflow_counter",
        "mppt_3:mode",
        "mppt_3:test_counter",
        "mppt_3:output_voltage_battery_side",
        "mppt_3:power_connector_temp", // PowerConnector
    ];
    for key in keys {
        let _: Result<(), redis::RedisError> = cmd("TS.CREATE")
            .arg(key)
            .arg("RETENTION")
            .arg(60_000)
            .arg("DUPLICATE_POLICY")
            .arg("last")
            .query_async(&mut con)
            .await;
    }

    loop {
        let (iov, time) = async_fd
            .async_io(Interest::READABLE, |inner| {
                let mut data = [0u8; 16];
                let time = {
                    let mut iov = [IoSliceMut::new(&mut data)];

                    let mut cmsg_buffer = cmsg_space!(nix::sys::time::TimeVal);

                    let r =
                        recvmsg::<()>(inner.as_raw_fd(), &mut iov, Some(&mut cmsg_buffer), flags)?;

                    let time = match r.cmsgs()?.next() {
                        Some(nix::sys::socket::ControlMessageOwned::ScmTimestamp(rtime)) => {
                            Some(rtime)
                        }
                        _ => None,
                    };
                    time
                };

                let mut data_clone: [u8; 16] = [0u8; 16];
                data_clone.clone_from_slice(&data);

                Ok((data_clone, time))
            })
            .await?;

        let time_seconds = match time {
            Some(t) => t.tv_sec().to_string(),
            None => {
                error!("Timestamp not written into cmsg buffer, using redis default value, *");
                "*".to_string()
            }
        };

        let can_id: u32 =
            iov[0] as u32 | (iov[1] as u32) << 8 | (iov[2] as u32) << 16 | (iov[3] as u32) << 24;

        let eff_flag = (iov[3] as u32 & 0x80 as u32) >> 7;

        let clean_id = if eff_flag == 1 {
            match socketcan::ExtendedId::new(can_id & 0x1FFFFFFF) {
                Some(id) => socketcan::Id::Extended(id),
                None => {
                    error!(eff_flag = true, "Invalid extended CAN Id");
                    continue;
                }
            }
        } else {
            match socketcan::StandardId::new((can_id & 0x000007FF) as u16) {
                Some(id) => socketcan::Id::Standard(id),
                None => {
                    error!(eff_flag = false, "Invalid Standard CAN Id");
                    continue;
                }
            }
        };

        let frame_data: &[u8] = &iov[8..=15];

        let can_frame = match socketcan::frame::CanFrame::new(clean_id, frame_data) {
            Some(frame) => frame,
            None => {
                error!(id = ?clean_id, data = frame_data, "Failed to cast Id and data into CAN Frame");
                continue;
            }
        };

        let raw_id = can_frame.raw_id();

        match raw_id & 0xFF0 {
            0x600 => {
                match messages_mppt::Messages::from_can_message(can_frame.id(), can_frame.data()) {
                    Ok(messages_mppt::Messages::PowerInput(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt:input_voltage")
                            .arg(&time_seconds)
                            .arg(frame.input_voltage())
                            .arg("mppt:input_current")
                            .arg(&time_seconds)
                            .arg(frame.input_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerInput Frame");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerInput Frame");
                    }
                    Ok(messages_mppt::Messages::PowerOutput(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt:output_voltage")
                            .arg(&time_seconds)
                            .arg(frame.output_voltage())
                            .arg("mppt:output_current")
                            .arg(&time_seconds)
                            .arg(frame.output_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerOutput Frame");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerOutput Frame");
                    }
                    Ok(messages_mppt::Messages::Temperature(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt:mosfet_temperature")
                            .arg(&time_seconds)
                            .arg(frame.mosfet_temperature())
                            .arg("mppt:controller_temperature")
                            .arg(&time_seconds)
                            .arg(frame.controller_temperature())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Temperature Frame");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Temperature Frame");
                    }
                    Ok(messages_mppt::Messages::AuxillaryPowerSupply(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt:twelve_volt")
                            .arg(&time_seconds)
                            .arg(frame.twelve_volt())
                            .arg("mppt:three_volt")
                            .arg(&time_seconds)
                            .arg(frame.three_volt())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on AuxillaryPowerSupply Frame");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote AuxillaryPowerSupply Frame");
                    }
                    Ok(messages_mppt::Messages::Limits(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt:max_output_voltage")
                            .arg(&time_seconds)
                            .arg(frame.max_output_voltage())
                            .arg("mppt:max_input_current")
                            .arg(&time_seconds)
                            .arg(frame.max_input_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Limits Frame");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Limits Frame");
                    }
                    Ok(messages_mppt::Messages::Status(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt:error_mosfet_overheat")
                            .arg(&time_seconds)
                            .arg(frame.error_mosfet_overheat())
                            .arg("mppt:error_low_arrow_power")
                            .arg(&time_seconds)
                            .arg(frame.error_low_arrow_power())
                            .arg("mppt:error_hw_over_voltage")
                            .arg(&time_seconds)
                            .arg(frame.error_hw_over_voltage())
                            .arg("mppt:error_hw_over_current")
                            .arg(&time_seconds)
                            .arg(frame.error_hw_over_current())
                            .arg("mppt:error_battery_low")
                            .arg(&time_seconds)
                            .arg(frame.error_battery_low())
                            .arg("mppt:error_battery_full")
                            .arg(&time_seconds)
                            .arg(frame.error_battery_full())
                            .arg("mppt:error_12v_undervoltage")
                            .arg(&time_seconds)
                            .arg(frame.error12v_undervoltage())
                            .arg("mppt:limit_output_voltage_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_output_voltage_max())
                            .arg("mppt:limit_mosfet_temperature")
                            .arg(&time_seconds)
                            .arg(frame.limit_mosfet_temperature())
                            .arg("mppt:limit_local_mppt")
                            .arg(&time_seconds)
                            .arg(frame.limit_local_mppt())
                            .arg("mppt:limit_input_current_min")
                            .arg(&time_seconds)
                            .arg(frame.limit_input_current_min())
                            .arg("mppt:limit_input_current_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_input_current_max())
                            .arg("mppt:limit_global_mppt")
                            .arg(&time_seconds)
                            .arg(frame.limit_global_mppt())
                            .arg("mppt:limit_duty_cycle_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_duty_cycle_max())
                            .arg("mppt:limit_duty_cycle_min")
                            .arg(&time_seconds)
                            .arg(frame.limit_dury_cycle_min())
                            .arg("mppt:can_rx_error_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_rx_error_counter())
                            .arg("mppt:can_tx_error_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_tx_error_counter())
                            .arg("mppt:can_tx_overflow_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_tx_overflow_counter())
                            .arg("mppt:mode")
                            .arg(&time_seconds)
                            .arg(frame.mode())
                            .arg("mppt:test_counter")
                            .arg(&time_seconds)
                            .arg(frame.test_counter())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Status Frame");
                        }
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Status Frame");
                    }
                    Ok(messages_mppt::Messages::PowerConnector(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt:output_voltage_battery_side")
                            .arg(&time_seconds)
                            .arg(frame.output_voltage_battery_side())
                            .arg("mppt:power_connector_temp")
                            .arg(&time_seconds)
                            .arg(frame.power_connector_temp())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerConnector Frame");
                        }
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerConnector Frame");
                    }
                    Err(e) => {
                        error!(error = %e, id = raw_id, "Failed to decode MPPT 1 frame");
                    }
                }
            }
            0x610 => {
                match messages_mppt_2::Messages::from_can_message(can_frame.id(), can_frame.data())
                {
                    Ok(messages_mppt_2::Messages::PowerInput(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_2:input_voltage")
                            .arg(&time_seconds)
                            .arg(frame.input_voltage())
                            .arg("mppt_2:input_current")
                            .arg(&time_seconds)
                            .arg(frame.input_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerInput Frame For MPPT #2");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerInput Frame For MPPT #2");
                    }
                    Ok(messages_mppt_2::Messages::PowerOutput(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_2:output_voltage")
                            .arg(&time_seconds)
                            .arg(frame.output_voltage())
                            .arg("mppt_2:output_current")
                            .arg(&time_seconds)
                            .arg(frame.output_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerOutput Frame for MPPT #2");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerOutput Frame for MPPT #2");
                    }
                    Ok(messages_mppt_2::Messages::Temperature(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_2:mosfet_temperature")
                            .arg(&time_seconds)
                            .arg(frame.mosfet_temperature())
                            .arg("mppt_2:controller_temperature")
                            .arg(&time_seconds)
                            .arg(frame.controller_temperature())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Temperature Frame for MPPT #2");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Temperature Frame for MPPT #2");
                    }
                    Ok(messages_mppt_2::Messages::AuxillaryPowerSupply(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_2:twelve_volt")
                            .arg(&time_seconds)
                            .arg(frame.twelve_volt())
                            .arg("mppt_2:three_volt")
                            .arg(&time_seconds)
                            .arg(frame.three_volt())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on AuxillaryPowerSupply Frame for MPPT #2");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote AuxillaryPowerSupply Frame for MPPT #2");
                    }
                    Ok(messages_mppt_2::Messages::Limits(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_2:max_output_voltage")
                            .arg(&time_seconds)
                            .arg(frame.max_output_voltage())
                            .arg("mppt_2:max_input_current")
                            .arg(&time_seconds)
                            .arg(frame.max_input_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Limits Frame for MPPT #2");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Limits Frame for MPPT #2");
                    }
                    Ok(messages_mppt_2::Messages::Status(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_2:error_mosfet_overheat")
                            .arg(&time_seconds)
                            .arg(frame.error_mosfet_overheat())
                            .arg("mppt_2:error_low_arrow_power")
                            .arg(&time_seconds)
                            .arg(frame.error_low_arrow_power())
                            .arg("mppt_2:error_hw_over_voltage")
                            .arg(&time_seconds)
                            .arg(frame.error_hw_over_voltage())
                            .arg("mppt_2:error_hw_over_current")
                            .arg(&time_seconds)
                            .arg(frame.error_hw_over_current())
                            .arg("mppt_2:error_battery_low")
                            .arg(&time_seconds)
                            .arg(frame.error_battery_low())
                            .arg("mppt_2:error_battery_full")
                            .arg(&time_seconds)
                            .arg(frame.error_battery_full())
                            .arg("mppt_2:error_12v_undervoltage")
                            .arg(&time_seconds)
                            .arg(frame.error12v_undervoltage())
                            .arg("mppt_2:limit_output_voltage_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_output_voltage_max())
                            .arg("mppt_2:limit_mosfet_temperature")
                            .arg(&time_seconds)
                            .arg(frame.limit_mosfet_temperature())
                            .arg("mppt_2:limit_local_mppt")
                            .arg(&time_seconds)
                            .arg(frame.limit_local_mppt())
                            .arg("mppt_2:limit_input_current_min")
                            .arg(&time_seconds)
                            .arg(frame.limit_input_current_min())
                            .arg("mppt_2:limit_input_current_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_input_current_max())
                            .arg("mppt_2:limit_global_mppt")
                            .arg(&time_seconds)
                            .arg(frame.limit_global_mppt())
                            .arg("mppt_2:limit_duty_cycle_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_duty_cycle_max())
                            .arg("mppt_2:limit_duty_cycle_min")
                            .arg(&time_seconds)
                            .arg(frame.limit_dury_cycle_min())
                            .arg("mppt_2:can_rx_error_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_rx_error_counter())
                            .arg("mppt_2:can_tx_error_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_tx_error_counter())
                            .arg("mppt_2:can_tx_overflow_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_tx_overflow_counter())
                            .arg("mppt_2:mode")
                            .arg(&time_seconds)
                            .arg(frame.mode())
                            .arg("mppt_2:test_counter")
                            .arg(&time_seconds)
                            .arg(frame.test_counter())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Status Frame for MPPT #2");
                        }
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Status Frame for MPPT #2");
                    }
                    Ok(messages_mppt_2::Messages::PowerConnector(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_2:output_voltage_battery_side")
                            .arg(&time_seconds)
                            .arg(frame.output_voltage_battery_side())
                            .arg("mppt_2:power_connector_temp")
                            .arg(&time_seconds)
                            .arg(frame.power_connector_temp())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerConnector Frame for MPPT #2");
                        }
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerConnector Frame for MPPT #2");
                    }
                    Err(e) => {
                        error!(error = %e, id = raw_id, "Failed to decode MPPT 2 frame");
                    }
                }
            }
            0x620 => {
                match messages_mppt_3::Messages::from_can_message(can_frame.id(), can_frame.data())
                {
                    Ok(messages_mppt_3::Messages::PowerInput(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_3:input_voltage")
                            .arg(&time_seconds)
                            .arg(frame.input_voltage())
                            .arg("mppt_3:input_current")
                            .arg(&time_seconds)
                            .arg(frame.input_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerInput Frame For MPPT #3");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerInput Frame For MPPT #3");
                    }
                    Ok(messages_mppt_3::Messages::PowerOutput(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_3:output_voltage")
                            .arg(&time_seconds)
                            .arg(frame.output_voltage())
                            .arg("mppt_3:output_current")
                            .arg(&time_seconds)
                            .arg(frame.output_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerOutput Frame for MPPT #3");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerOutput Frame for MPPT #3");
                    }
                    Ok(messages_mppt_3::Messages::Temperature(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_3:mosfet_temperature")
                            .arg(&time_seconds)
                            .arg(frame.mosfet_temperature())
                            .arg("mppt_3:controller_temperature")
                            .arg(&time_seconds)
                            .arg(frame.controller_temperature())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Temperature Frame for MPPT #3");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Temperature Frame for MPPT #3");
                    }
                    Ok(messages_mppt_3::Messages::AuxillaryPowerSupply(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_3:twelve_volt")
                            .arg(&time_seconds)
                            .arg(frame.twelve_volt())
                            .arg("mppt_3:three_volt")
                            .arg(&time_seconds)
                            .arg(frame.three_volt())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on AuxillaryPowerSupply Frame for MPPT #3");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote AuxillaryPowerSupply Frame for MPPT #3");
                    }
                    Ok(messages_mppt_3::Messages::Limits(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_3:max_output_voltage")
                            .arg(&time_seconds)
                            .arg(frame.max_output_voltage())
                            .arg("mppt_3:max_input_current")
                            .arg(&time_seconds)
                            .arg(frame.max_input_current())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Limits Frame for MPPT #3");
                        };
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Limits Frame for MPPT #3");
                    }
                    Ok(messages_mppt_3::Messages::Status(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_3:error_mosfet_overheat")
                            .arg(&time_seconds)
                            .arg(frame.error_mosfet_overheat())
                            .arg("mppt_3:error_low_arrow_power")
                            .arg(&time_seconds)
                            .arg(frame.error_low_arrow_power())
                            .arg("mppt_3:error_hw_over_voltage")
                            .arg(&time_seconds)
                            .arg(frame.error_hw_over_voltage())
                            .arg("mppt_3:error_hw_over_current")
                            .arg(&time_seconds)
                            .arg(frame.error_hw_over_current())
                            .arg("mppt_3:error_battery_low")
                            .arg(&time_seconds)
                            .arg(frame.error_battery_low())
                            .arg("mppt_3:error_battery_full")
                            .arg(&time_seconds)
                            .arg(frame.error_battery_full())
                            .arg("mppt_3:error_12v_undervoltage")
                            .arg(&time_seconds)
                            .arg(frame.error12v_undervoltage())
                            .arg("mppt_3:limit_output_voltage_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_output_voltage_max())
                            .arg("mppt_3:limit_mosfet_temperature")
                            .arg(&time_seconds)
                            .arg(frame.limit_mosfet_temperature())
                            .arg("mppt_3:limit_local_mppt")
                            .arg(&time_seconds)
                            .arg(frame.limit_local_mppt())
                            .arg("mppt_3:limit_input_current_min")
                            .arg(&time_seconds)
                            .arg(frame.limit_input_current_min())
                            .arg("mppt_3:limit_input_current_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_input_current_max())
                            .arg("mppt_3:limit_global_mppt")
                            .arg(&time_seconds)
                            .arg(frame.limit_global_mppt())
                            .arg("mppt_3:limit_duty_cycle_max")
                            .arg(&time_seconds)
                            .arg(frame.limit_duty_cycle_max())
                            .arg("mppt_3:limit_duty_cycle_min")
                            .arg(&time_seconds)
                            .arg(frame.limit_dury_cycle_min())
                            .arg("mppt_3:can_rx_error_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_rx_error_counter())
                            .arg("mppt_3:can_tx_error_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_tx_error_counter())
                            .arg("mppt_3:can_tx_overflow_counter")
                            .arg(&time_seconds)
                            .arg(frame.can_tx_overflow_counter())
                            .arg("mppt_3:mode")
                            .arg(&time_seconds)
                            .arg(frame.mode())
                            .arg("mppt_3:test_counter")
                            .arg(&time_seconds)
                            .arg(frame.test_counter())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on Status Frame for MPPT #3");
                        }
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Status Frame for MPPT #3");
                    }
                    Ok(messages_mppt_3::Messages::PowerConnector(frame)) => {
                        if let Err(e) = redis::cmd("TS.MADD")
                            .arg("mppt_3:output_voltage_battery_side")
                            .arg(&time_seconds)
                            .arg(frame.output_voltage_battery_side())
                            .arg("mppt_3:power_connector_temp")
                            .arg(&time_seconds)
                            .arg(frame.power_connector_temp())
                            .query_async::<_, Vec<u64>>(&mut con)
                            .await
                        {
                            error!(frame = ?frame, error = %e, "Failed Write to Redis on PowerConnector Frame for MPPT #3");
                        }
                        info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerConnector Frame for MPPT #3");
                    }
                    Err(e) => {
                        error!(error = %e, id = raw_id, "Failed to decode MPPT 3 frame");
                    }
                }
            }
            _ => {
                error!(id = raw_id, "Unknown CAN base address, not an MPPT frame"); // TODO: If not an MPPT Frame, decode for BMS frame
            }
        }
    }
}
