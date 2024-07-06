use std::{fmt::Debug, thread::sleep, time::Duration};

use rdev::{simulate as _simulate, Button, EventType, Key, SimulateError};

use crate::{global::*, impls::KeyConvert};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coordinate {
    Abs,
    Rel,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Event {
    KeyPress {
        key: Key,
        elapse: u128,
        duration: u128,
    },
    KeyRelease {
        key: Key,
        elapse: u128,
        duration: u128,
    },
    ButtonPress {
        button: Button,
        x: f64,
        y: f64,
        coordinate: Coordinate,
        elapse: u128,
        duration: u128,
    },
    ButtonRelease {
        button: Button,
        x: f64,
        y: f64,
        coordinate: Coordinate,
        elapse: u128,
        duration: u128,
    },
    MouseMove {
        x: f64,
        y: f64,
        elapse: u128,
        duration: u128,
    },
    Drag {
        button: Button,
        x: f64,
        y: f64,
        elapse: u128,
        duration: u128,
    },
    Wheel {
        delta_x: i64,
        delta_y: i64,
        elapse: u128,
        duration: u128,
    },
}

#[allow(dead_code)]
impl Event {
    pub fn build(event: Self, previous_event: Option<&Self>) -> Event {
        let elapse: u128 = instant_elapse_millis();
        let mut actual_duration = 0;
        if let Some(previous_event) = previous_event {
            if elapse > previous_event.elapse() {
                actual_duration = elapse - previous_event.elapse();
            }
        }
        let mut event = event;
        match event {
            Event::KeyPress {
                ref mut duration, ..
            } => {
                *duration = actual_duration;
            }
            Event::KeyRelease {
                ref mut duration, ..
            } => {
                *duration = actual_duration;
            }
            Event::ButtonPress {
                ref mut duration, ..
            } => {
                *duration = actual_duration;
            }
            Event::ButtonRelease {
                ref mut duration, ..
            } => {
                *duration = actual_duration;
            }
            Event::MouseMove {
                ref mut duration, ..
            } => {
                *duration = actual_duration;
            }
            Event::Drag {
                ref mut duration, ..
            } => {
                *duration = actual_duration;
            }
            Event::Wheel {
                ref mut duration, ..
            } => {
                *duration = actual_duration;
            }
        }
        event
    }

    fn elapse(&self) -> u128 {
        match self {
            Event::KeyPress { elapse, .. } => *elapse,
            Event::KeyRelease { elapse, .. } => *elapse,
            Event::ButtonPress { elapse, .. } => *elapse,
            Event::ButtonRelease { elapse, .. } => *elapse,
            Event::MouseMove { elapse, .. } => *elapse,
            Event::Drag { elapse, .. } => *elapse,
            Event::Wheel { elapse, .. } => *elapse,
        }
    }

    fn duration(&self) -> u128 {
        match self {
            Event::KeyPress { duration, .. } => *duration,
            Event::KeyRelease { duration, .. } => *duration,
            Event::ButtonPress { duration, .. } => *duration,
            Event::ButtonRelease { duration, .. } => *duration,
            Event::MouseMove { duration, .. } => *duration,
            Event::Drag { duration, .. } => *duration,
            Event::Wheel { duration, .. } => *duration,
        }
    }

    pub fn to_rhai(&self) -> String {
        match self {
            Event::KeyPress { key, duration, .. } => {
                format!("key_press({});\ndelay({});", key.as_str(), duration)
            }
            Event::KeyRelease { key, duration, .. } => {
                format!("key_release({});\ndelay({});", key.as_str(), duration)
            }
            Event::ButtonPress {
                button,
                x,
                y,
                duration,
                ..
            } => match button {
                Button::Left => format!("button_left_press({},{});\ndelay({});", x, y, duration),
                Button::Right => {
                    format!("button_right_press({},{});\ndelay({});", x, y, duration)
                }
                Button::Middle => String::new(),
                Button::Unknown(_) => String::new(),
            },
            Event::ButtonRelease {
                button,
                x,
                y,
                duration,
                ..
            } => match button {
                Button::Left => {
                    format!(
                        "button_left_release({},{});\ndelay({});",
                        *x as i64, *y as i64, duration
                    )
                }
                Button::Right => {
                    format!(
                        "button_right_release({},{});\ndelay({});",
                        *x as i64, *y as i64, duration
                    )
                }
                Button::Middle => String::new(),
                Button::Unknown(_) => String::new(),
            },
            Event::MouseMove { x, y, duration, .. } => {
                format!("mouse_move({},{});\ndelay({});", x, y, duration)
            }
            Event::Drag {
                button,
                x,
                y,
                duration,
                ..
            } => match button {
                Button::Left => format!("drag_left_instant({},{});\ndelay({});", x, y, duration),
                Button::Right => {
                    format!("drag_right_instant({},{});\ndelay({});", x, y, duration)
                }
                Button::Middle => String::new(),
                Button::Unknown(_) => String::new(),
            },
            Event::Wheel {
                delta_y, duration, ..
            } => {
                if *delta_y > 0 {
                    return format!("wheel_down({});\ndelay({});", delta_y, duration);
                }
                if *delta_y < 0 {
                    return format!("wheel_up({});\ndelay({});", delta_y, duration);
                }
                String::new()
            }
        }
    }

    pub fn simulate(&self) {
        let event = self.clone();
        sleep(Duration::from_millis(event.duration() as u64));
        let event_type: EventType = event.into();
        println!("send: {:?}", &event_type);
        match _simulate(&event_type) {
            Ok(()) => (),
            Err(SimulateError) => {
                println!("We could not send {:?}", event_type);
            }
        }
    }
}

pub fn generate_vritual_path<T, F, I, G>(
    from_x: T,
    from_y: F,
    to_x: I,
    to_y: G,
    duration: u128,
) -> Option<Vec<(i32, i32, u128)>>
where
    T: Into<f64>,
    F: Into<f64>,
    I: Into<f64>,
    G: Into<f64>,
{
    let mut from_x = from_x.into() as i32;
    let mut from_y = from_y.into() as i32;
    let to_x = to_x.into() as i32;
    let to_y = to_y.into() as i32;
    let mut points = Vec::new();
    let caculate_callback = |from: i32, to: i32| {
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
    let len = points.len() as u128;
    if len > 0 {
        let each_duration = duration / len;
        points.iter_mut().for_each(|point| {
            point.2 = each_duration;
        });
        return Some(points);
    }
    None
}
