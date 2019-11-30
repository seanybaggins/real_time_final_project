use crate::real_time;
use crate::real_time::RealTime;

use crate::ring_buffer::RingBuffer;

use std::sync::{Arc, Mutex};
use std::num::Wrapping;

use std::f64;

use opencv::videoio;
use opencv::videoio::{CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};
use opencv::core;
use opencv::core::Mat;
use opencv::imgproc::COLOR_RGB2GRAY;

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

        // warm up the camera and take some frames frames
        let mut frame = core::Mat::default().unwrap();
        
        for _ in 0..100 {
            hardware.read(&mut frame)
            .expect("failed taking frames in initialization");
        }

        Camera {
            hardware,
            ring_buffer,
            ring_read_write_count
        }
        
    }
}

impl RealTime for Camera {

    fn name(&self) -> &str {
        "Camera"
    }

    fn priority(&self) -> i32 {
        real_time::MAX_PRIORITY - 1
    }

    fn frequency(&self) -> u32 {
        10
    }

    fn service(&mut self) {
        
        let mut ring_read_write_count = self.ring_read_write_count.lock().unwrap();
        let writer_count = &mut (*ring_read_write_count).1;
        let write_index = (*writer_count).0 % self.ring_buffer.buffer.len();

        let mut frame = self.ring_buffer.buffer.get(write_index).unwrap().lock().unwrap();

        self.hardware.read(&mut *frame)
            .expect("Error in writing frame");

        *writer_count += Wrapping(1);

    }
}

#[allow(dead_code)]
pub fn show_frame(frame: &mut Mat) {
    opencv::highgui::named_window(WINDOW, 1)
        .expect("failed window init");

    opencv::highgui::imshow(WINDOW, &mut *frame)
        .expect("unable to show frame");

    opencv::highgui::wait_key(10)
        .expect("Failure of wait key show frame");
}

#[allow(dead_code)]
pub fn convert_to_grayscale(frame: &Mat) -> Mat {
    let mut gray_frame = Mat::default().unwrap();
    
    opencv::imgproc::cvt_color(frame, &mut gray_frame, COLOR_RGB2GRAY,0).unwrap();
    
    gray_frame
}

pub struct FrameDiffer {
    ring_buffer: Arc<RingBuffer>,
    ring_read_write_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>,
    best_frame: Arc<Mutex<opencv::prelude::Mat>>,
    min_frame_diff: f64
}

impl FrameDiffer {
    pub fn new(ring_buffer: Arc<RingBuffer>, 
    ring_read_write_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>,
    best_frame: Arc<Mutex<core::Mat>>) -> Self {
        FrameDiffer {
            ring_buffer,
            ring_read_write_count,
            best_frame,
            min_frame_diff: f64::MAX
        }
    }

    fn diff_of_frames(frame0: &Mat, frame1: &Mat) {
        
        let mut diff_frame = Mat::default().unwrap();
        opencv::core::absdiff(frame0, frame1, &mut diff_frame).unwrap();
        let diff_rgb_data = opencv::core::sum(&diff_frame).unwrap();

        let mut sum = 0.0;
        for i in 0..diff_rgb_data.len() {
            sum += diff_rgb_data[i];
        }

        println!("{}", sum);
        
    }
}

impl RealTime for FrameDiffer {

    fn service(&mut self) {
        
        // Determining indexes in the ring buffer
        let mut ring_read_write_count = self.ring_read_write_count.lock().unwrap();
        let reader_count = &mut (*ring_read_write_count).0;
        let writer_count = &mut (*ring_read_write_count).1;

        if *writer_count <= *reader_count {
            // Then the buffer has not been populated yet
            return;
        }

        let reader_index = (*reader_count).0 % self.ring_buffer.buffer.len();
        let reader_index_1 = ((*reader_count).0 - 1) % self.ring_buffer.buffer.len();
        *reader_count += Wrapping(1);

        // dropping the counts after the indexes have been determined
        drop(ring_read_write_count);

        // Get the frames in the ring buffer
        let frame_n = self.ring_buffer.buffer.get(reader_index).unwrap().lock().unwrap();
        let frame_n_1 = self.ring_buffer.buffer.get(reader_index_1).unwrap().lock().unwrap();

        // Check if we need to reset our reference for the min frame diff

        let frame_diff = FrameDiffer::diff_of_frames(&(*frame_n), &(*frame_n_1));
        
        
        
    }

    fn name(&self) -> &str {
        "Frame Differ"
    }

    fn priority(&self) -> i32 {
        real_time::MAX_PRIORITY - 2
    }

    fn frequency(&self) -> u32 {
        5
    }

    
}

#[cfg(test)]
mod tests;