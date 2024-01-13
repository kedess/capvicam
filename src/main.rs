use clap::Parser;
use libc::memcpy;
use log::{debug, error, info};
use std::{
    ffi::CString,
    sync::{atomic::AtomicBool, Arc},
};

use crate::{
    capture::Capture,
    utils::ioctl::{
        check_support_video_streaming, close_device, format_info, init_fmt, open_device,
        read_device_capability, Buffer,
    },
};

mod capture;
mod error;
mod utils;

#[derive(Parser, Debug)]
#[clap(version, about = "Video Capture V4L2")]
struct Args {
    /// Path to device (default /dev/video0)
    #[clap(short, long, default_value_t=String::from("/dev/video0"))]
    path: String,
    /// Width picture (default 640)
    #[clap(long, default_value_t = 640)]
    width: i32,
    /// Height picture (default 480)
    #[clap(long, default_value_t = 480)]
    height: i32,
}

extern "C" fn handler(buffer: Buffer) {
    /* Данные из буфера обязательно нужно копировать, чтобы перенести в вектор например,
     * так как буферы освобождаются системой
     */
    let dest = unsafe {
        let dest = libc::malloc(buffer.len as usize);
        memcpy(dest, buffer.start, buffer.len as usize);
        dest
    };
    let data =
        unsafe { Vec::from_raw_parts(dest as *mut u8, buffer.len as usize, buffer.len as usize) };
    debug!("Picture size = {}", data.len());
}

fn main() {
    let args = Args::parse();
    env_logger::init();
    let interrupt = Arc::new(AtomicBool::new(false));
    let interrupt2 = interrupt.clone();
    ctrlc::set_handler(move || {
        info!("Received signal SIGINT. Stopping app");
        interrupt2.store(true, std::sync::atomic::Ordering::Relaxed);
    })
    .expect("Error settings SIGINT handler");
    let path = CString::new(args.path.as_bytes()).expect("Invalid path to device");
    match open_device(path.as_ptr()) {
        Ok(fd) => {
            info!("Device [{}] opened successfully", args.path);
            if let Some(caps) = read_device_capability(fd) {
                info!("{}", caps);
                if check_support_video_streaming(caps.get_device_caps()) {
                    info!("The device supports streaming. We're starting to stream");
                    format_info(fd);
                    match init_fmt(fd, args.width, args.height) {
                        Err(err) => error!("{}", err),
                        Ok(_) => {
                            let capture = Capture::new();
                            capture.run(fd, interrupt, handler);
                        }
                    }
                }
            }
            match close_device(fd) {
                Ok(_) => info!("Device closed successfully"),
                Err(err) => error!("Error closing device. {}", err),
            }
        }
        Err(err) => error!("Could not open device. {}: {}", err, args.path),
    }
}
