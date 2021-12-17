#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
compile_error!("multicam (v4l2) is for linux/freebsd only");
extern crate eye;
extern crate log;
extern crate log4rs;
use eye::prelude::*;
use std::process::Command;
use std::{thread, time};
use log::{info, warn, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
/**
 * Main function
 */
fn main() {
    let ctx = Context::new();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {l} -- {m}\n")))
        .build("multicam.log").expect("could not create log file");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).expect("log configuration invalid");

    log4rs::init_config(config).expect("could not initialize logging");

    // Detect video devices
    let devices = ctx.query_devices().expect("Failed to query devices");

    if devices.is_empty() {
        panic!("No devices available");
    }

    devices.into_iter().for_each(|_device_uri| {
        let mut _device = _device_uri.replace("v4l://", "");
        info!("Probing for camera at {}", _device);
        let mut _name = _device.replace("/dev/", "");
        let mut _mp4 = format!("%Y%m%d%H%M%S-{}.mp4", _name);

        // Each camera needs to be activated in it's own thread
        thread::Builder::new()
            .name(format!("multicam{}", _name))
            .spawn(move || {
                let _command = Command::new("ffmpeg")
                    .args([
                        "-y", // overwrite
                        "-i",
                        &_device.to_owned(), // set video at index as input
                        "-loglevel",
                        "quiet", //suppress ffmpeg log
                        "-f",
                        "segment", // use segment muxer
                        "-strftime",
                        "1", // use time as string in filename
                        "-reset_timestamps", 
                        "1", // reset every segment to 0
                        "-segment_time",
                        "360", // segment should be 360 seconds
                        &_mp4.to_owned(),
                    ])
                    .status().unwrap();
                    warn!("{} is not a valid camera: {}", _device, _command);
            }).unwrap();
    });

    let ten_millis = time::Duration::from_millis(10000);

    loop {
        thread::sleep(ten_millis);
        info!("multicam running")
    }
}
