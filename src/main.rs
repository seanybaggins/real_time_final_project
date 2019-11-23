mod real_time;
mod imaging;

use syslog::Facility;
use log::LevelFilter;

use real_time::{Sequencer, RealTime};
use std::sync::mpsc::{Sender, Receiver, channel};

use std::time::Duration;

use imaging::Camera;
use rscam::Frame;

fn main() {
    
    syslog::init(
        Facility::LOG_USER, 
        LevelFilter::Info,
        Some("Sean's Code question_3")
    ).expect("Unable to connect to syslog");

    // Initializing resources
    let (to_file_differ, from_camera): (Sender<Frame>, Receiver<Frame>) = channel();
    let camera: Box<RealTime + Send> = Box::new(Camera::new(to_file_differ));
    let services: Vec<Box<RealTime + Send>> = vec![camera];
    let sequencer = Sequencer::new();

    let stop_time = Duration::from_millis(100);
    sequencer.sequence(services, stop_time);
}
