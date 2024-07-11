use async_std::{sync, task::block_on};
use bitmap::{FrameBitmap, FrameBitmapYCbCr, VideoFrameBitmap};
use crabgrab::feature::*;
use crabgrab::{
    capturable_content::{CapturableContent, CapturableContentFilter},
    capture_stream::{CaptureConfig, CaptureStream, StreamEvent},
    platform::macos::MacosCaptureConfigExt,
};
use std::sync::mpsc;
use std::thread::spawn;
use std::time::Duration;
const BUFFER_SIZE: usize = 10000;
use image::{ImageBuffer, Rgb, RgbImage};

async fn token() -> crabgrab::prelude::CaptureAccessToken {
    match CaptureStream::test_access(true) {
        Some(token) => token,
        None => CaptureStream::request_access(true)
            .await
            .expect("Expected capture access"),
    }
}
async fn content() -> CapturableContent {
    let filter = CapturableContentFilter::NORMAL_WINDOWS;
    CapturableContent::new(filter).await.unwrap()
}

fn data_handle(receiver: mpsc::Receiver<Vec<u8>>) {
    spawn(move || loop {
        if let Ok(data) = receiver.try_recv() {
            println!("got new msg");
        }
    });
}

fn ycbcr_to_rgb(y: u8, cb: u8, cr: u8) -> Rgb<u8> {
    let y = y as f32;
    let cb = cb as f32 - 128.0;
    let cr = cr as f32 - 128.0;

    let r = (y + 1.402 * cr).clamp(0.0, 255.0) as u8;
    let g = (y - 0.344136 * cb - 0.714136 * cr).clamp(0.0, 255.0) as u8;
    let b = (y + 1.772 * cb).clamp(0.0, 255.0) as u8;

    Rgb([r, g, b])
}

fn windows() {
    let token = block_on(token());
    let content = block_on(content());
    let window = content
        .windows()
        .filter(|window| window.title().contains("抖音"))
        .last()
        .unwrap();
    let config =
        CaptureConfig::with_window(window, CaptureStream::supported_pixel_formats()[0]).unwrap();
    let (sender, mut receiver) = mpsc::channel();
    data_handle(receiver);
    let mut stream = CaptureStream::new(token, config, move |stream_event| match stream_event {
        Ok(event) => match event {
            StreamEvent::Video(frame) => {
                let data = frame.get_bitmap();
                if let Err(err) = data {
                    println!("err: {:?}", err);
                } else {
                    match data.unwrap() {
                        FrameBitmap::BgraUnorm8x4(data) => {
                            println!("hello1")
                        }
                        FrameBitmap::ArgbUnormPacked2101010(data) => {
                            println!("hello2")
                        }
                        FrameBitmap::RgbaF16x4(data) => {
                            println!("hello3")
                        }
                        FrameBitmap::YCbCr(data) => {
                            let a = data.luma_data;

                            println!("hello4")
                        }
                    };
                }
                // if let Ok(FrameBitmap::BgraUnorm8x4(image_bitmap_bgra8888)) = frame.get_bitmap() {
                //     let data: Vec<u8> = image_bitmap_bgra8888
                //         .data
                //         .iter()
                //         .flat_map(|&[b, g, r, a]| vec![b, g, r, a])
                //         .collect();
                //     sender.send(data).unwrap();
                // } else {
                //     println!("err");
                // }
            }
            _ => {}
        },
        Err(error) => {
            println!("Stream error: {:?}", error);
        }
    })
    .unwrap();
    std::thread::sleep(Duration::from_secs(40));
    stream.stop().unwrap();
}
fn main() {
    windows();
}
