use crate::real_time;
use crate::real_time::RealTime;

use crate::ring_buffer::RingBuffer;

use std::sync::{Arc, Mutex};

use std::num::Wrapping;

use opencv::videoio;
use opencv::videoio::{CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};

pub struct Camera {
    pub hardware: opencv::videoio::VideoCapture,
    pub ring_buffer: Arc<RingBuffer>,
    pub ring_reader_writer_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>
}

impl Camera {

    pub fn new(ring_buffer: Arc<RingBuffer>, 
    ring_reader_writer_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>) -> Self {

        #[cfg(feature = "opencv-32")]
        let mut hardware = videoio::VideoCapture::new(CAP_MODE_GRAY).unwrap();  // 0 is the default camera
        #[cfg(not(feature = "opencv-32"))]
        let mut hardware = videoio::VideoCapture::new_with_backend(0, videoio::CAP_ANY).unwrap();  // 0 is the default camera
        let opened = videoio::VideoCapture::is_opened(&hardware).unwrap();
        if !opened {
            panic!("Unable to open default camera!");
        }

        hardware.set(CAP_PROP_FRAME_HEIGHT, 480.0).unwrap();
        hardware.set(CAP_PROP_FRAME_WIDTH, 640.0).unwrap();

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
