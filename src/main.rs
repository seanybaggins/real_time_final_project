mod real_time;
use real_time::{Sequencer, RealTime};

mod imaging;
use imaging::Camera;

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
        Some("Sean's Code question_3")
    ).expect("Unable to connect to syslog");
    
    let ring_buffer = Arc::new(RingBuffer::with_capacity(10));
    let ring_reader_writer_indexs = Arc::new(Mutex::new(
        (Wrapping(1), Wrapping(0))
    ));

    let camera: Box<RealTime + Send> = Box::new(Camera::new(
        ring_buffer.clone(),
        ring_reader_writer_indexs.clone()
    ));

    let services: Vec<Box<RealTime + Send>> = vec![camera];
    let sequencer = Sequencer::new();
    
    let stop_time = Duration::from_millis(100);
    sequencer.sequence(services, stop_time);
}
