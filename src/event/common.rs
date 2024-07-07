#[allow(unused_imports)]
use crate::i;
#[allow(unused_imports)]
use log::{info, warn};

use super::{rhai::run, Data, Event};
use crate::common::{Float, Int, UInt};
use rdev::{
    get_current_mouse_location as _get_current_mouse_location, grab as _grab, listen as _listen,
    simulate as _simulate, stop_grab as _stop_grab, stop_listen as _stop_listen, Event as _Event,
    EventType as _EventType,
};
use rhai::EvalAltResult;
pub fn grab<T>(callback: T) -> Result<(), GrabError>
where
    T: Fn(_Event) -> Option<_Event> + 'static,
{
    match _grab(callback) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("grab error: {:?}", err);
            Err(err.into())
        }
    }
}
#[allow(dead_code)]
pub fn stop_grab() {
    _stop_grab();
}
#[allow(dead_code)]
pub fn listen<T>(callback: T) -> Result<(), ListenError>
where
    T: FnMut(_Event) + 'static,
{
    match _listen(callback) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("listen error: {:?}", err);
            Err(err.into())
        }
    }
}
#[allow(dead_code)]
pub fn stop_listen() {
    _stop_listen();
}

pub fn current_point() -> (Float, Float) {
    if let Some(point) = _get_current_mouse_location() {
        (point.x as Float, point.y as Float)
    } else {
        (0 as Float, 0 as Float)
    }
}

pub fn simulate_event(event: Event) -> Result<(), SimulateError> {
    let _event: _EventType = event.into();
    match _simulate(&_event) {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("simulate event error: {:?}", _event);
            Err(err.into())
        }
    }
}

pub fn simulate_event_data(event_data: &Data) -> Result<(), SimulateError> {
    let content = event_data.content.clone();
    match run(content) {
        Err(err) => {
            warn!("simulate file error: {:?}", err);
            Err(err)
        }
        Ok(_) => Ok(()),
    }
}

pub fn events_to_data(events: &Vec<Event>) -> Data {
    let mut content = String::new();
    events.iter().for_each(|event| {
        content += &event.to_string();
    });
    Data::new("undefined".to_string(), content)
}

pub fn virtual_path<T, F, I, G>(
    from_x: T,
    from_y: F,
    to_x: I,
    to_y: G,
    duration: UInt,
) -> Option<Vec<(Int, Int, UInt)>>
where
    T: Into<Float>,
    F: Into<Float>,
    I: Into<Float>,
    G: Into<Float>,
{
    let mut from_x = from_x.into() as Int;
    let mut from_y = from_y.into() as Int;
    let to_x = to_x.into() as Int;
    let to_y = to_y.into() as Int;
    let mut points = Vec::new();
    let caculate_callback = |from: Int, to: Int| {
        if from < to {
            return from + 1;
        }
        if from > to {
            return from - 1;
        }
        from
    };
    while from_x != to_x && from_y != to_y {
        from_x = caculate_callback(from_x, to_x);
        from_y = caculate_callback(from_y, to_y);
        points.push((from_x, from_y, 0));
    }
    let len = points.len() as UInt;
    if len > 0 {
        let each_duration = duration / len;
        points.iter_mut().for_each(|point| {
            point.2 = each_duration;
        });
        return Some(points);
    }
    None
}

pub enum GrabError {
    /// MacOS
    EventTapError,
    /// MacOS
    LoopSourceError,
    /// Linux
    MissingDisplayError,
    /// Linux
    KeyboardError,
    /// Windows
    KeyHookError(UInt),
    /// Windows
    MouseHookError(UInt),
    /// All
    SimulateError,
    IoError(std::io::Error),
}

pub enum ListenError {
    /// MacOS
    EventTapError,
    /// MacOS
    LoopSourceError,
    /// Linux
    MissingDisplayError,
    /// Linux
    KeyboardError,
    /// Linux
    RecordContextEnablingError,
    /// Linux
    RecordContextError,
    /// Linux
    XRecordExtensionError,
    /// Windows
    KeyHookError(UInt),
    /// Windows
    MouseHookError(UInt),
}

#[derive(Debug)]
pub enum SimulateError {
    Default,
    Rhai(Box<EvalAltResult>),
}
