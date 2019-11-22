mod real_time;
mod imaging;

use syslog::Facility;
use log::LevelFilter;

use real_time::{Sequencer, RealTime};

use std::mem;

use imaging::Camera;
use rscam::Frame;

fn main() {
    
    syslog::init(
        Facility::LOG_USER, 
        LevelFilter::Info,
        Some("Sean's Code question_3")
    ).expect("Unable to connect to syslog");

    // Initializing resources
    

    let camera: Box<RealTime + Send> = Box::new(Camera::new());

    let services: Vec<Box<RealTime + Send>> = vec![camera];

    let sequencer = Sequencer::new(services);


}
