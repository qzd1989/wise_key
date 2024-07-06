use crate::{
    event::{Coordinate, Event},
    global::instant_elapse_millis,
};
use rdev::{Event as _Event, EventType};

impl Into<Event> for _Event {
    fn into(self) -> Event {
        let elapse = instant_elapse_millis();
        match self.event_type {
            EventType::KeyPress(key) => Event::KeyPress {
                key,
                elapse,
                duration: 0,
            },
            EventType::KeyRelease(key) => Event::KeyRelease {
                key,
                elapse,
                duration: 0,
            },
            EventType::ButtonPress { button, x, y } => Event::ButtonPress {
                button,
                x,
                y,
                coordinate: Coordinate::Abs,
                elapse,
                duration: 0,
            },
            EventType::ButtonRelease { button, x, y } => Event::ButtonRelease {
                button,
                x,
                y,
                coordinate: Coordinate::Abs,
                elapse,
                duration: 0,
            },
            EventType::Drag { button, x, y } => Event::Drag {
                button,
                x,
                y,
                elapse,
                duration: 0,
            },
            EventType::MouseMove { x, y } => Event::MouseMove {
                x,
                y,
                elapse,
                duration: 0,
            },
            EventType::Wheel { delta_x, delta_y } => Event::Wheel {
                delta_x,
                delta_y,
                elapse,
                duration: 0,
            },
        }
    }
}

impl Into<EventType> for Event {
    fn into(self) -> EventType {
        match self {
            Event::KeyPress { key, .. } => EventType::KeyPress(key),
            Event::KeyRelease { key, .. } => EventType::KeyRelease(key),
            Event::ButtonPress { button, x, y, .. } => EventType::ButtonPress { button, x, y },
            Event::ButtonRelease { button, x, y, .. } => EventType::ButtonRelease { button, x, y },
            Event::MouseMove { x, y, .. } => EventType::MouseMove { x, y },
            Event::Drag { button, x, y, .. } => EventType::Drag { button, x, y },
            Event::Wheel {
                delta_x, delta_y, ..
            } => EventType::Wheel { delta_x, delta_y },
        }
    }
}
