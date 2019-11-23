use crate::real_time;
use crate::real_time::RealTime;

use rscam::{Config, Frame};

use std::sync::{Arc, mpsc::Sender};

pub struct Camera {
    to_file_differ: Sender<Frame>,
    hardware: rscam::Camera,
}

impl Camera {
    pub fn capture(&self) -> Frame {
        self.hardware.capture().unwrap()
    }

    pub fn new (to_file_differ: Sender<Frame>) -> Self {
        let mut camera = Camera {
            to_file_differ,
            hardware: rscam::Camera::new("/dev/video0").unwrap(),
        };

        camera.hardware.start(
            &Config {
                interval: (1, 30),      // 30 fps.
                resolution: (640, 480),
                format: b"RGB3",
                ..Default::default()
            }
        )
        .expect("Could not configure camera");

        // Capture the first 10 frames to make sure we don't get a stagnant
        for _ in 0..10 {
            camera.hardware.capture().unwrap();
        }

        camera
    }
}

impl RealTime for Camera {

    fn priority(&self) -> i32 {
        real_time::MAX_PRIORITY - 2
    }

    fn frequency(&self) -> u32 {
        10
    }

    fn service(&self) {
        
    }
}
