///global fns, types, etc.
#[allow(unused_imports)]
use crate::i;
#[allow(unused_imports)]
use log::{info, warn};

use crossbeam_channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;
use std::sync::RwLock;
use std::time::Instant;

pub type Int = i32;
pub type UInt = u32;
pub type Float = f64;

pub fn init_instant() {
    *NOW.write().unwrap() = Some(Instant::now());
}

pub fn clean_instant() {
    *NOW.write().unwrap() = None;
}

pub fn instant_elapse_millis() -> UInt {
    match *NOW.read().unwrap() {
        Some(lock) => lock.elapsed().as_millis() as u32,
        None => 0,
    }
}

pub fn simulate_state_send(msg: bool) {
    if let Err(err) = SIMULATE_STATE_CHANNEL.0.send(msg) {
        warn!("simulate_state_send error: {:?}", err);
    }
}

// pub fn capture_send(msg: Texture) {
//     if let Err(err) = CAPTURE_CHANNEL.0.send(msg) {
//         warn!("capture_send error: {:?}", err);
//     }
// }

lazy_static! {
    pub static ref NOW: RwLock<Option<Instant>> = RwLock::new(None);
    pub static ref SIMULATE_STATE_CHANNEL: (Sender<bool>, Receiver<bool>) = unbounded();
    // pub static ref CAPTURE_CHANNEL: (Sender<Texture>, Receiver<Texture>) = unbounded();
}
