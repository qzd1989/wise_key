use crabgrab::{feature::bitmap::VideoFrameBitmap as _, prelude::*};
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgb, RgbImage, Rgba};
use std::{fs::File, io::BufWriter, ops::Deref, thread::spawn, time::Duration};
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    let future = runtime.spawn(async {
        let token = match CaptureStream::test_access(false) {
            Some(token) => token,
            None => CaptureStream::request_access(false).await.expect("Expected capture access")
        };
        let filter = CapturableContentFilter::NORMAL_WINDOWS;
        let content = CapturableContent::new(filter).await.unwrap();
        let window = content.windows().filter(|window| {
            let app_identifier = window.application().identifier();
            window.title().len() != 0 && app_identifier.to_lowercase().contains("chrome")
        }).next();
        match window {
            Some(window) => {
                println!("capturing window: {}", window.title()); 
                let config = CaptureConfig::with_window(window, CapturePixelFormat::Bgra8888).unwrap();
                let mut number = 0;
                let mut stream = CaptureStream::new(token, config, move |stream_event| {
                    match stream_event {
                        Ok(event) => {
                            match event {
                                StreamEvent::Video(frame) => {
                                    println!("Got frame: {}", frame.frame_id());
                                    match frame.get_bitmap() {
                                        Ok(bitmap) => {
                                            match bitmap {
                                                crabgrab::feature::bitmap::FrameBitmap::BgraUnorm8x4(data) => {
                                                    println!("format: BgraUnorm8x4");
                                                   
                                                },
                                                crabgrab::feature::bitmap::FrameBitmap::ArgbUnormPacked2101010(_) => println!("format: ArgbUnormPacked2101010"),
                                                crabgrab::feature::bitmap::FrameBitmap::RgbaF16x4(_) => println!("format: RgbaF16x4"),
                                                crabgrab::feature::bitmap::FrameBitmap::YCbCr(data) => {
                                                    println!("format: YCbCr");
                                                    let (chroma_data, luma_data) = (data.chroma_data.deref(), data.luma_data.deref());
                                                    println!("chroma.width is {}, chroma.height is {}", data.chroma_width, data.chroma_height);
                                                    println!("luma.width is {}, luma.height is {}", data.luma_width, data.luma_height);
                                                    let (width, height) = (800, 437);
                                                    println!("chroma.len is {}, luma.len is {}", chroma_data.len(), luma_data.len());
                                                    let mut img: RgbImage = ImageBuffer::new(width, height);
                                                    for (x, y, pixel) in img.enumerate_pixels_mut() {
                                                        let index = (y * width + x) as usize;
                                                        let luma = luma_data[index];
                                                        let chroma = chroma_data[index];
                                                        *pixel = Rgb([luma, chroma[0], chroma[1]]);
                                                    }
                                                    img.save("output.png").unwrap();
                                                    println!("Image saved as output.png");
                                                },
                                            }
                                        },
                                        Err(e) => {
                                            println!("Bitmap error: {:?}", e);
                                        }
                                    }
                                    //valid on macos
                                    //MetalVideoFramePlaneTexture::Chroma
                                    //MetalVideoFramePlaneTexture::Luminance
                                    match frame.get_metal_texture(MetalVideoFramePlaneTexture::Luminance){
                                        Ok(_) => {
                                            println!("got metal texture");
                                        },
                                        Err(err) => {
                                            println!("got error of get_mental_texture: error is {}", err);
                                        },
                                    }
                                    //invalid on macos
                                    match frame.get_wgpu_texture(WgpuVideoFramePlaneTexture::Chroma, None){
                                        Ok(_) => println!("get_wgpu_texture: Chroma"),
                                        Err(err) => println!("get_wgpu_texture: Chroma, err is {err}"),
                                    }
                                    //invalid on macos
                                    match frame.get_wgpu_texture(WgpuVideoFramePlaneTexture::Luminance, None){
                                        Ok(_) => println!("get_wgpu_texture: Luminance"),
                                        Err(err) => println!("get_wgpu_texture: Luminance, err is {err}"),
                                    }
                                    //invalid on macos
                                    match frame.get_wgpu_texture(WgpuVideoFramePlaneTexture::Rgba, None){
                                        Ok(_) => println!("get_wgpu_texture: Rgba"),
                                        Err(err) => println!("get_wgpu_texture: Rgba, err is {err}"),
                                    }
                                },
                                _ => {}
                            }
                        },
                        Err(error) => {
                            println!("Stream error: {:?}", error);
                        }
                    }
                }).unwrap();
                println!("stream created!"); 
                tokio::task::block_in_place(|| std::thread::sleep(Duration::from_millis(3000)));
                stream.stop().unwrap();
            },
            None => { println!("Failed to find window"); }
        }
    });
    runtime.block_on(future).unwrap();
    runtime.shutdown_timeout(Duration::from_millis(10000000));
}

