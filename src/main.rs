mod scheduling;
use scheduling::{Sequencer, RealTime};

mod imaging;
use imaging::{Camera, FrameDiffer};

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
    let (to_file_write, from_frame_selector) = channel::<Mat>();

    // Creating best effort task for file io
    let best_effort_thread = fileio::backround_write_files(from_frame_selector, Arc::clone(&universal_clock));

    let frame_differ: Box<dyn RealTime + Send> = Box::new(FrameDiffer::new(
        ring_buffer.clone(),
        ring_read_write_count.clone(),
        to_file_write
    ));

    let camera: Box<dyn RealTime + Send> = Box::new(Camera::new(
        ring_buffer.clone(),
        ring_read_write_count.clone()
    ));

    let services: Vec<Box<dyn RealTime + Send>> = vec![frame_differ, camera];
    let sequencer = Sequencer::new();
    
    let stop_time = Duration::from_secs(180);
    sequencer.sequence(services, stop_time, Arc::clone(&universal_clock));

    // Allow the best effort thread to finish writing frames
    best_effort_thread.join().expect("Failed to join best effort thread");
}
