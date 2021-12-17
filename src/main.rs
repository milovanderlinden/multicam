#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
compile_error!("multicam (v4l2) is for linux/freebsd only");
extern crate eye;
extern crate rscam;
use eye::prelude::*;
use rscam::{Camera, Config};
use std::{fs::{self}, io::{Write}, thread, time};


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

                    // Start the camera
                    camera
                        .start(&Config {
                            interval: (1, 30), // 30 fps.
                            resolution: (1280, 720),
                            format: b"MJPG",
                            ..Default::default()
                        })
                        .expect("not a valid camera interface");

                    // Create file to hold the frame
                    let mut _jpeg_file = fs::File::create(&_jpeg).unwrap();
                    //let mut _video_file = fs::File::create(&_mp4).unwrap();

                    // Grab the frames and write them to a file
                    loop {
                        let frame = camera.capture().unwrap();
                        //println!("{} - {} => {}", frame.get_timestamp(), _device, _jpeg);
                        _jpeg_file.write_all(&frame[..]).unwrap();
                        //_video_file.write_all(b"P5\n2448 2048\n255\n").unwrap();
                        //_video_file.write_all(&frame).unwrap();
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
