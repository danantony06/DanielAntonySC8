// Test data replayer — same shape as the old script, now dual-bus.
// Loops through a single candump log file and routes each frame:
//   extended ID (Kelly motorcontroller) → vcan0
//   standard ID 0x600–0x62F (MPPTs)     → vcan1
//
// Run: cargo run --bin test_feed
// (with test_data_combined.log in the working directory)

use anyhow::{Context, Result};
use embedded_can::Frame;
use socketcan::{dump::Reader, CanAnyFrame, CanSocket, Socket};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let kelly_sock = CanSocket::open("vcan0") // bus 1: motorcontroller
        .with_context(|| "Failed to open socket on vcan0")?;

    let mppt_sock = CanSocket::open("vcan1") // bus 2: mppts
        .with_context(|| "Failed to open socket on vcan1")?;

    let path = Path::new("src/test_data_combined.txt"); // Path to test data

    loop {
        // re-open the reader each pass so the file replays forever
        let mut reader =
            Reader::from_file(&path).with_context(|| "Error opening txt file")?;

        for rec in reader.records() {
            let (_ts, frame) = rec?;

            match frame {
                CanAnyFrame::Normal(f) => {
                    // Kelly uses extended 29-bit IDs (0CF11E05 / 0CF11F05);
                    // the MPPTs use standard 11-bit IDs at 0x600/0x610/0x620.
                    if f.is_extended() {
                        kelly_sock.write_frame(&f)?;
                    } else {
                        mppt_sock.write_frame(&f)?;
                    }
                }
                CanAnyFrame::Remote(_) => {}
                CanAnyFrame::Fd(_) => {} // readers expect classic frames only
                _ => {}
            }

            // pace the replay so it streams continuously instead of
            // dumping the whole file at once (~50 frames/sec)
            sleep(Duration::from_millis(20));
        }
    }
}