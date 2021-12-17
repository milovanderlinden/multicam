#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
compile_error!("multicam (v4l2) is for linux/freebsd only");
extern crate eye;
use eye::prelude::*;
use std::{thread, time};
use std::process::{Command};

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
                    let _command = Command::new("ffmpeg")
                        .args([
                            "-y",
                            "-loglevel", 
                            "quiet",
                            "-f",
                            "v4l2",
                            "-video_size",
                            "1280x720",
                            "-i",
                            &_device.to_owned(),
                            &_mp4.to_owned()
                        ])
                        .spawn()
                        .expect("failed to execute process");
                    loop {
                    } // keep running.
                    
                }),
        );
    });

    let ten_millis = time::Duration::from_millis(10000);

    loop {
        thread::sleep(ten_millis);
        println!("Multicam is running with {:?} cameras", threads.len());
    }

}
