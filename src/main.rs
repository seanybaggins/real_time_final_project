mod real_time;
use real_time::{Sequencer, RealTime};

mod imaging;
use imaging::{Camera, FrameDiffer};
use opencv::core;

mod ring_buffer;
use ring_buffer::RingBuffer;

use syslog::Facility;
use log::LevelFilter;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use std::num::Wrapping;

fn main() {
    
    syslog::init(
        Facility::LOG_USER, 
        LevelFilter::Info,
        Some("Sean's Code Final Project")
    ).expect("Unable to connect to syslog");
    
    let ring_buffer = Arc::new(RingBuffer::with_capacity(10));
    let ring_read_write_count = Arc::new(Mutex::new((Wrapping(1), Wrapping(0))));
    let best_frame = Arc::new(Mutex::new(core::Mat::default().unwrap()));

    let camera: Box<RealTime + Send> = Box::new(Camera::new(
        ring_buffer.clone(),
        ring_read_write_count.clone()
    ));

    let frame_differ: Box<RealTime + Send> = Box::new(FrameDiffer::new(
        ring_buffer.clone(),
        ring_read_write_count.clone(),
        best_frame.clone()
    ));

    let services: Vec<Box<RealTime + Send>> = vec![camera, frame_differ];
    let sequencer = Sequencer::new();
    
    let stop_time = Duration::from_secs(1);
    sequencer.sequence(services, stop_time);
}
