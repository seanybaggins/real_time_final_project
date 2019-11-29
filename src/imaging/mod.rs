use crate::real_time;
use crate::real_time::RealTime;

use crate::ring_buffer::RingBuffer;

use std::sync::{Arc, Mutex};
use std::num::Wrapping;

use opencv::videoio;
use opencv::videoio::{CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};

const WINDOW: &str = "video capture";

pub struct Camera {
    pub hardware: opencv::videoio::VideoCapture,
    ring_buffer: Arc<RingBuffer>,
    ring_read_write_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>
}

impl Camera {

    pub fn new(ring_buffer: Arc<RingBuffer>, 
    ring_read_write_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>) -> Self {

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
            ring_read_write_count
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

    fn service(&mut self) {
        
        //opencv::highgui::named_window(WINDOW, 1)
        //    .expect("failed window init");

        // Determining what index to write to the ring buffer
        let mut ring_read_write_count = self.ring_read_write_count.lock().unwrap();
        let writer_count = &mut (*ring_read_write_count).1;
        let write_index = (*writer_count).0 % self.ring_buffer.buffer.len();

        let mut frame = self.ring_buffer.buffer.get(write_index).unwrap().lock().unwrap();

        self.hardware.read(&mut *frame)
            .expect("Error in writing frame");

        *writer_count += Wrapping(1);

        //opencv::highgui::imshow(WINDOW, &mut *frame)
        //    .expect("unable to show frame");

        //opencv::highgui::wait_key(5).unwrap();
    }
}

struct FileDiffer {
    ring_buffer: Arc<Mutex<RingBuffer>>,
}

impl FileDiffer {
    fn new(ring_buffer: Arc<Mutex<RingBuffer>>) -> Self {
        FileDiffer {
            ring_buffer
        }
    }

    
}

impl RealTime for FileDiffer {
    fn priority(&self) -> i32 {
        real_time::MAX_PRIORITY - 1
    }

    fn frequency(&self) -> u32 {
        20
    }

    fn service(&mut self) {
        
    }
}

#[cfg(test)]
mod tests;