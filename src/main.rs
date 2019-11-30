mod scheduling;
use scheduling::{Sequencer, RealTime};

mod imaging;
use imaging::{Camera, FrameDiffer, FrameSelector};
use opencv::core;
use opencv::core::Mat;

mod ring_buffer;
use ring_buffer::RingBuffer;

mod fileio;

use syslog::Facility;
use log::LevelFilter;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use std::num::Wrapping;

fn main() {
    
    syslog::init(
        Facility::LOG_USER, 
        LevelFilter::Info,
        Some("Sean's Code Final Project")
    ).expect("Unable to connect to syslog");
    
    let universal_clock = Arc::new(Instant::now());
    let ring_buffer = Arc::new(RingBuffer::with_capacity(10));
    let ring_read_write_count = Arc::new(Mutex::new((Wrapping(1), Wrapping(0))));
    let best_frame = Arc::new(Mutex::new(None));
    let (to_file_write, from_frame_selector) = channel::<Mat>();

    // Creating best effort task for file io
    fileio::backround_write_files(from_frame_selector, Arc::clone(&universal_clock));

    let camera: Box<RealTime + Send> = Box::new(Camera::new(
        ring_buffer.clone(),
        ring_read_write_count.clone()
    ));

    let frame_differ: Box<RealTime + Send> = Box::new(FrameDiffer::new(
        ring_buffer.clone(),
        ring_read_write_count.clone(),
        best_frame.clone()
    ));

    let frame_selector: Box<RealTime + Send> = Box::new(FrameSelector::new(
        to_file_write,
        best_frame.clone()
    ));

    let services: Vec<Box<RealTime + Send>> = vec![camera, frame_differ];
    let sequencer = Sequencer::new();
    
    let stop_time = Duration::from_secs(1);
    sequencer.sequence(services, stop_time, Arc::clone(&universal_clock));
}
