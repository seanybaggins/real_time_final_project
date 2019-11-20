mod real_time;
mod camera;

use syslog::Facility;
use log::LevelFilter;

use real_time::{Sequencer, RealTime};
use std::sync::Arc;

use camera::Camera;

fn main() {
    
    syslog::init(
        Facility::LOG_USER, 
        LevelFilter::Info,
        Some("Sean's Code question_3")
    ).expect("Unable to connect to syslog");

    // Initializing resources
    let camera = Camera::new();
    let sequencer = Sequencer::new();

    // Setting CPU affinity and priority of sequencer
    Sequencer::real_time_setup();

    // Defining services of sequencer and other tasks
    let camera_service = Arc::new(
        move || {
            let frame = camera.capture();
        }
    );
    
    
    let services = vec![camera_service];

    sequencer.sequence(services);


}
