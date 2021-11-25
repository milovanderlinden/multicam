extern crate eye;
extern crate rscam;
use eye::prelude::*;
use rscam::{Camera, Config};
use std::{fs, io::Write, thread, time};

fn main() {
    let ctx = Context::new();

    // Detect video devices
    let devices = ctx.query_devices().expect("Failed to query devices");

    if devices.is_empty() {
        panic!("No devices available");
    }

    for _device_uri in devices {
        println!("Found camera at {}", _device_uri);
        // Each camera needs to be activated in it's own thread
        thread::spawn(move || -> ! {
            // Initialize the camera reader
            let mut _device = _device_uri.replace("v4l://", "");
            let mut _file_name = format!("{}.jpeg", _device.replace("/", "_"));
            let mut camera = Camera::new(&_device_uri.replace("v4l://", ""))
                .expect("Failed to open video device");

            // TODO: Initialize the video writer
            

            // Start the camera
            camera
                .start(&Config {
                    interval: (1, 30), // 30 fps.
                    resolution: (1280, 720),
                    format: b"MJPG",
                    ..Default::default()
                })
                .unwrap();
            // Grab the frames and write them to a file
            loop {
                let frame = camera.capture().unwrap();
                println!("{} - {} => {}",frame.get_timestamp(), _device, _file_name);

                // Writing to a jpeg file for now, needs to be replaced with writing to video format.
                let mut _file = fs::File::create(&_file_name).unwrap();
                _file.write_all(&frame[..]).unwrap();
            }
        });
    }
    let ten_millis = time::Duration::from_millis(10000);
    loop {
        thread::sleep(ten_millis);
        println!("Multicam is running")
    }
}
