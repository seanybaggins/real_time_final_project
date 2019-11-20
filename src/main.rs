mod real_time;
mod camera;

use syslog::Facility;
use log::{LevelFilter, error};

use std::thread;
use std::sync::mpsc::channel;

use real_time::{Sequencer, RealTime, 
    SequencerCommand::{Exit, ProvideService}};

use camera::Camera;

fn main() {
    
    syslog::init(
        Facility::LOG_USER, 
        LevelFilter::Info,
        Some("Sean's Code question_3")
    ).expect("Unable to connect to syslog");

    // Creating synchronization resources
    Sequencer::real_time_setup();
    let (tx_camera, rx_camera) = channel();

    let camera = Camera::new();

    let camera_service = move || {
        Camera::real_time_setup();

        loop {
            let sequencer_command = match rx_camera.recv() {
                Ok(sequencer_command) => sequencer_command,
                Err(error) =>  {
                    error!("camera failed to receive command: {}", error);
                    continue;
                }
            };

            match sequencer_command {
                ProvideService => {

                }
                Exit => break
            }
        }
    };
    
    thread::spawn(move || {
        
    });

}
