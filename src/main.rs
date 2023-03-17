// fusée gelée
//
// Launcher for the {re}switched coldboot/bootrom hacks--
// launches payloads above the Horizon

use clap::Parser;
use rusb::{Device, DeviceHandle, Result, UsbContext};
use std::env;
use std::fs;
use std::process;
use std::time::Duration;

const SWITCH_RCM_VID: u16 = 0x0955;
const SWITCH_RCM_PID: u16 = 0x7321;
const RCM_PAYLOAD_EP: u8 = 0x01;

fn find_switch_rcm<T: UsbContext>(context: &T) -> Result<Option<(Device<T>, DeviceHandle<T>)>> {
    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;

        if device_desc.vendor_id() == SWITCH_RCM_VID && device_desc.product_id() == SWITCH_RCM_PID {
            let handle = device.open()?;
            return Ok(Some((device, handle)));
        }
    }

    Ok(None)
}

fn send_payload<T: UsbContext>(handle: &mut DeviceHandle<T>, payload: &[u8]) -> Result<()> {
    handle.claim_interface(0)?;

    let timeout = Duration::from_secs(15);
    let response = handle.write_bulk(RCM_PAYLOAD_EP, payload, timeout);

    println!("{:?}", response);

    handle.release_interface(0)?;

    Ok(())
}

/// Program to inject a payload into a Nintendo Switch in RCM mode
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// How long to wait for the device
    #[arg(short, long)]
    wait_for_device: String,

    /// Which OS is currently being used
    #[arg(short, long)]
    os_override: String,

    /// VID
    #[arg(short, long)]
    vid: String,

    /// PID
    #[arg(short, long)]
    pid: String,

    /// Override checks
    #[arg(short, long)]
    override_checks: String,
}

// Implement the HaxBackend and DeviceHandle traits or structs for your platform,
// and replace unimplemented!() with the actual implementation.

// Here's the main function where you parse the command-line arguments and use the RCMHax struct.
// You can use the clap crate or the std::env::args() function to parse arguments.
// fn main() {
//     let args = Args::parse();

//     // Parse command-line arguments...
//     let wait_for_device = args.wait_for_device;
//     let os_override = args.os_override;
//     let vid = args.vid;
//     let pid = args.pid;
//     let override_checks = args.override_checks;
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <payload.bin>", args[0]);
        process::exit(1);
    }

    let payload = fs::read(&args[1]).expect("Failed to read payload file");

    let context = rusb::Context::new().expect("Failed to create USB context");

    match find_switch_rcm(&context).expect("Failed to enumerate USB devices") {
        Some((_, mut handle)) => {
            println!("Switch found in RCM mode");
            send_payload(&mut handle, &payload).expect("Failed to send payload");
            println!("Payload sent successfully");
        }
        None => {
            eprintln!("No Switch found in RCM mode");
            process::exit(1);
        }
    }
}
