use std::{thread, time::Duration};

use glib;
use gst::prelude::*;
use gstreamer as gst;
use gstreamer_app::AppSink;

pub fn run<T, F: FnOnce() -> T + Send + 'static>(main: F) -> T
where
    T: Send + 'static,
{
    use cocoa::{
        appkit::{NSApplication, NSWindow},
        base::id,
        delegate,
    };
    use objc::{
        class, msg_send,
        runtime::{Object, Sel},
        sel, sel_impl,
    };
    use std::{
        ffi::c_void,
        sync::mpsc::{channel, Sender},
        thread,
    };
    unsafe {
        let app = cocoa::appkit::NSApp();
        let (send, recv) = channel::<()>();
        extern "C" fn on_finish_launching(this: &Object, _cmd: Sel, _notification: id) {
            let send = unsafe {
                let send_pointer = *this.get_ivar::<*const c_void>("send");
                let boxed = Box::from_raw(send_pointer as *mut Sender<()>);
                *boxed
            };
            send.send(()).unwrap();
        }
        let delegate = delegate!("AppDelegate", {
            app: id = app,
            send: *const c_void = Box::into_raw(Box::new(send)) as *const c_void,
            (applicationDidFinishLaunching:) => on_finish_launching as extern fn(&Object, Sel, id)
        });
        app.setDelegate_(delegate);
        let t = thread::spawn(move || {
            // Wait for the NSApp to launch to avoid possibly calling stop_() too early
            recv.recv().unwrap();
            let res = main();
            let app = cocoa::appkit::NSApp();
            app.stop_(cocoa::base::nil);
            // Stopping the event loop requires an actual event
            let event = cocoa::appkit::NSEvent::otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
                cocoa::base::nil,
                cocoa::appkit::NSEventType::NSApplicationDefined,
                cocoa::foundation::NSPoint { x: 0.0, y: 0.0 },
                cocoa::appkit::NSEventModifierFlags::empty(),
                0.0,
                0,
                cocoa::base::nil,
                cocoa::appkit::NSEventSubtype::NSApplicationActivatedEventType,
                0,
                0,
            );
            app.postEvent_atStart_(event, cocoa::base::YES);
            res
        });
        app.run();
        t.join().unwrap()
    }
}

fn example_main() {
    gst::init().unwrap();
    let pipeline = gst::Pipeline::new();
    let src = gst::ElementFactory::make("avfvideosrc").build().unwrap();
    src.set_property("capture-screen", true);
    src.set_property("device-index", 1);
    let video_scale = gst::ElementFactory::make("videoscale").build().unwrap();
    let autovideosink = gst::ElementFactory::make("autovideosink").build().unwrap();
    let appsink = gst::ElementFactory::make("appsink").build().unwrap();

    let framerate_caps = gst::Caps::builder("video/x-raw")
        .field("framerate", &gst::Fraction::new(30, 1))
        .build();
    let framerate_filter = gst::ElementFactory::make("capsfilter")
        .build()
        .expect("Failed to create capsfilter");
    framerate_filter.set_property("caps", &framerate_caps);

    let size_caps = gst::Caps::builder("video/x-raw")
        .field("width", &1920)
        .field("height", &1080)
        .build();

    let size_filter = gst::ElementFactory::make("capsfilter")
        .build()
        .expect("Failed to create capsfilter");
    size_filter.set_property("caps", &size_caps);
    pipeline
        .add_many(&[
            &src,
            &video_scale,
            &size_filter,
            &framerate_filter,
            &autovideosink,
        ])
        .unwrap();
    gst::Element::link_many(&[
        &src,
        &video_scale,
        &size_filter,
        &framerate_filter,
        &autovideosink,
    ])
    .unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();

    appsink.connect("new-sample", false, move |args| {
        let sample = args[1].get::<gst::Sample>().expect("Could not get sample");
        let buffer = sample.buffer().expect("Could not get buffer");
        // Process the buffer (TODO: Add your processing logic here)
        println!("Got frame: {:?}", buffer.pts());
        // Returning true means we handled the sample
        None
    });

    let bus = pipeline.bus().unwrap();

    glib::timeout_add_seconds(1, move || {
        // TODO: Add your processing logic here
        println!("Processing video stream...");
        glib::ControlFlow::Continue
    });

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Eos(..) => {
                println!("received eos");
                // An EndOfStream event was sent to the pipeline, so exit
                break;
            }
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            MessageView::Info(info) => {
                println!("Info: {:?}", info);
            }
            MessageView::InstantRateRequest(rate) => {
                println!("InstantRateRequest: {:?}", rate);
            }
            _ => {}
        };
    }

    pipeline.set_state(gst::State::Null).unwrap();
}

fn main() {
    println!("hehe");
    run(example_main);
}
