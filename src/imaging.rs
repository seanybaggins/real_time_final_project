use crate::real_time;
use crate::real_time::RealTime;

use rscam::{Config, Frame};

use std::sync::{Arc, Mutex};

use std::num::Wrapping;

use std::{mem, ptr};

pub struct Camera {
    hardware: rscam::Camera,
    pub ring_buffer: Arc<[Mutex<Box<Frame>>; 10]>,
    pub ring_reader_writer_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>
}

impl Camera {
    pub fn capture(&self) -> Frame {
        self.hardware.capture().unwrap()
    }

    pub fn new () -> Self {
        let mut hardware = rscam::Camera::new("/dev/video0").unwrap();
        
        hardware.start(
            &Config {
                interval: (1, 30),      // 30 fps.
                resolution: (640, 480),
                format: b"RGB3",
                ..Default::default()
            }
        )
        .expect("Could not configure camera");

        let mut fake: Vec<Box<_>> = Vec::new();
        
        let frame = hardware.capture().unwrap();

        fake.push(Box::new(frame));

        let ring_buffer = unsafe {
            let mut ring_buffer: [Mutex<Box<Frame>>; 10] = mem::uninitialized();
            for element in ring_buffer.iter_mut() {
                let frame = Mutex::new(Box::new(hardware.capture().unwrap()));
                ptr::write(element, frame);
            }
            ring_buffer
        };

        
        let ring_buffer = Arc::new(ring_buffer);
        let ring_reader_writer_count = Arc::new(Mutex::new(
            (Wrapping(1), Wrapping(0))
        ));

        Camera {
            hardware,
            ring_buffer,
            ring_reader_writer_count
        }
        
    }
}

impl RealTime for Camera {

    fn priority(&self) -> i32 {
        real_time::MAX_PRIORITY - 1
    }

    fn frequency(&self) -> u32 {
        20
    }

    fn service(&self) {
        /*
        let frame = self.capture();

        // Acquiring locks to ring buffer
        let (reader_count, writer_count) = *self.ring_reader_writer_count
            .lock().unwrap();
        let mut buffer_resource = self.ring_buffer[writer_count.0]
            .lock().unwrap();
        
        *buffer_resource = frame;
        */
    }
}
