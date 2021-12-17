#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
compile_error!("multicam (v4l2) is for linux/freebsd only");
extern crate eye;
extern crate rscam;
extern crate mp4;
use eye::prelude::*;
use rscam::{Camera, Config, CtrlData, FLAG_DISABLED, ResolutionInfo};
use std::{fs::{self}, io::{Write}, thread, time};

/**
 * Prints information about the camera resolutions
 */
fn resolution_info(camera: &Camera) {
    for wformat in camera.formats() {
        let format = wformat.unwrap();
        println!("{:?}", format);
        let resolutions = camera.resolutions(&format.format).unwrap();
        if let ResolutionInfo::Discretes(d) = resolutions {
            for resol in &d {
                println!(
                    "  {}x{}  {:?}",
                    resol.0,
                    resol.1,
                    camera.intervals(&format.format, *resol).unwrap()
                );
            }
        } else {
            println!("  {:?}", resolutions);
        }
    }
}

/**
 * Prints information about the camera control
 */
fn control_info(camera: &Camera) {
    for wctrl in camera.controls() {
        let ctrl = wctrl.unwrap();

        if let CtrlData::CtrlClass = ctrl.data {
            println!("\n[{}]\n", ctrl.name);
            continue;
        }

        print!("{:>32} ", ctrl.name);

        if ctrl.flags & FLAG_DISABLED != 0 {
            println!("(disabled)");
            continue;
        }

        match ctrl.data {
            CtrlData::Integer {
                value,
                default,
                minimum,
                maximum,
                step,
            } => println!(
                "(int)     min={} max={} step={} default={} value={}",
                minimum, maximum, step, default, value
            ),
            CtrlData::Boolean { value, default } => {
                println!("(bool)    default={} value={}", default, value)
            }
            CtrlData::Menu {
                value,
                default,
                ref items,
                ..
            } => {
                println!("(menu)    default={} value={}", default, value);
                for item in items {
                    println!("{:42} {}: {}", "", item.index, item.name);
                }
            }
            CtrlData::IntegerMenu {
                value,
                default,
                ref items,
                ..
            } => {
                println!("(intmenu) default={} value={}", default, value);
                for item in items {
                    println!(
                        "{:42} {}: {} ({:#x})",
                        "", item.index, item.value, item.value
                    );
                }
            }
            CtrlData::Bitmask {
                value,
                default,
                maximum,
            } => println!(
                "(bitmask) max={:x} default={:x} value={:x}",
                maximum, default, value
            ),
            CtrlData::Integer64 {
                value,
                default,
                minimum,
                maximum,
                step,
            } => println!(
                "(int64)   min={} max={} step={} default={} value={}",
                minimum, maximum, step, default, value
            ),
            CtrlData::String {
                ref value,
                minimum,
                maximum,
                step,
            } => println!(
                "(str)     min={} max={} step={} value={}",
                minimum, maximum, step, value
            ),
            CtrlData::Button => println!("(button)"),
            _ => {}
        }
    }
}

/**
 * Main function
 */
fn main() {
    let ctx = Context::new();
    // Placeholder to gather threads
    let mut threads = vec![];
    // Detect video devices
    let devices = ctx.query_devices().expect("Failed to query devices");

    if devices.is_empty() {
        panic!("No devices available");
    }

    devices.into_iter().for_each(|_device_uri| {
        println!("Found camera at {}", _device_uri);
        let mut _device = _device_uri.replace("v4l://", "");
        let mut _name = _device.replace("/", "_");
        let mut _jpeg = format!("{}.jpeg", _name);
        let mut _mp4 = format!("{}.mp4", _name);

        // Each camera needs to be activated in it's own thread
        threads.push(
            thread::Builder::new()
                .name(format!("multicam{}", _name))
                .spawn(move || -> ! {
                    // Initialize the camera reader
                    let mut camera = Camera::new(&_device).expect("Failed to open video device");
                    resolution_info(&camera);
                    control_info(&camera);
                    
                    // Start the camera
                    camera
                        .start(&Config {
                            interval: (1, 30), // 30 fps.
                            resolution: (1280, 720),
                            format: b"MJPG",
                            ..Default::default()
                        })
                        .expect("not a valid camera interface");

                    // Create files
                    let mut _jpeg_file = fs::File::create(&_jpeg).unwrap();

                    // Grab the frames and write them to a file
                    loop {
                        let frame = camera.capture().unwrap();
                        println!("{} - {} => {}", frame.get_timestamp(), _device, _jpeg);
                        _jpeg_file.write_all(&frame[..]).unwrap();
                    }
                }),
        );
    });
    let ten_millis = time::Duration::from_millis(10000);
    loop {
        thread::sleep(ten_millis);
        println!("Multicam is running");
    }
}
