mod real_time;
mod imaging;

use syslog::Facility;
use log::LevelFilter;

use real_time::{Sequencer, RealTime};
use std::sync::Arc;

use imaging::Camera;

fn main() {
    
    syslog::init(
        Facility::LOG_USER, 
        LevelFilter::Info,
        Some("Sean's Code question_3")
    ).expect("Unable to connect to syslog");

    // Initializing resources
    let camera: Arc<RealTime + Send + Sync> = Arc::new(Camera::new());

    let services: Vec<Arc<RealTime + Send + Sync>> = vec![camera];

    let sequencer = Sequencer::new();

    sequencer.sequence(services);


}
