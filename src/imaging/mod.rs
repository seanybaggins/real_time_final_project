use crate::scheduling;
use crate::scheduling::RealTime;

use crate::ring_buffer::RingBuffer;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

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

        
        let mut hardware = init_camera_hardware();

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

fn init_camera_hardware() -> videoio::VideoCapture {
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

    hardware
}

impl RealTime for Camera {

    fn name(&self) -> &str {
        "Camera"
    }

    fn priority(&self) -> i32 {
        scheduling::MAX_PRIORITY - 2
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
    
    opencv::imgproc::cvt_color(frame, &mut gray_frame, COLOR_RGB2GRAY, 0).unwrap();
    
    gray_frame
}

pub struct FrameDiffer {
    ring_buffer: Arc<RingBuffer>,
    ring_read_write_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>,
    to_file_write: Sender<Mat>,
    still_frame_threshold: f64,
    blurred_frame_threshold: f64,
    new_best_frame_needed: bool,
}

impl FrameDiffer {
    pub fn new(ring_buffer: Arc<RingBuffer>, 
    ring_read_write_count: Arc<Mutex<(Wrapping<usize>, Wrapping<usize>)>>,
    to_file_write: Sender<Mat>) -> Self {

        let (still_frame_threshold, blurred_frame_threshold) 
            = Self::still_and_blurred_frame_threshold();

        FrameDiffer {
            ring_buffer,
            ring_read_write_count,
            to_file_write,
            still_frame_threshold,
            blurred_frame_threshold,
            new_best_frame_needed: true
        }
    }

    fn still_and_blurred_frame_threshold() -> (f64, f64) {
        let mut hardware = init_camera_hardware();
        let mut frame0 = opencv::core::Mat::default().unwrap();
        let mut frame1 = opencv::core::Mat::default().unwrap();
        let start_time = Instant::now();

        let mut blurred_frame_diff_value = 0.0;
        let mut still_frame_diff_value = f64::MAX;
        while start_time.elapsed() < Duration::from_secs(4) {
            hardware.read(&mut frame0).unwrap();
            hardware.read(&mut frame1).unwrap();
            
            let diff_frame_value = Self::diff_of_frames(&mut frame0, &mut frame1);

            if diff_frame_value > blurred_frame_diff_value {
                blurred_frame_diff_value = diff_frame_value;
            }
            if diff_frame_value < still_frame_diff_value {
                still_frame_diff_value = diff_frame_value;
            }
        }

        (still_frame_diff_value, blurred_frame_diff_value)
    }

    fn image_did_blur(&self,frame_diff_value: f64) -> bool {
        let distance_from_still_frame_threshold 
            = (frame_diff_value - self.still_frame_threshold).abs();

        let distance_from_blurred_frame_threshold
            = (frame_diff_value - self.blurred_frame_threshold).abs();
            
        if distance_from_blurred_frame_threshold < distance_from_still_frame_threshold {
            return true;
        }
        else {
            return false;
        }
    }

    fn diff_of_frames(frame0: &Mat, frame1: &Mat) -> f64 {

        let mut gray0 = convert_to_grayscale(frame0);
        let mut gray1 = convert_to_grayscale(frame1);
        
        let mut diff_frame = Mat::default().unwrap();
        opencv::core::absdiff(&mut gray0, &mut gray1, &mut diff_frame).unwrap();
        //show_frame(&mut diff_frame);
        let diff_rgb_data = opencv::core::sum(&diff_frame).unwrap();

        let mut sum = 0.0;
        for i in 0..diff_rgb_data.len() {
            sum += diff_rgb_data[i];
        }

        sum
        
    }
}

impl RealTime for FrameDiffer {

    fn service(&mut self) {

        // Determining indexes in the ring buffer
        let mut ring_read_write_count = self.ring_read_write_count.lock().unwrap();
        let (ref mut reader_count, ref mut writer_count) = *ring_read_write_count;
        
        if writer_count <= reader_count {
            // Then the buffer has not been populated yet
            return;
        }

        let reader_index = (*reader_count).0 % self.ring_buffer.buffer.len();
        let reader_index_1 = ((*reader_count) - Wrapping(1)).0 % self.ring_buffer.buffer.len();
        *reader_count += Wrapping(1);

        // dropping the counts after the indexes have been determined
        drop(ring_read_write_count);

        // Get the frames in the ring buffer
        let frame_n = self.ring_buffer.buffer.get(reader_index).unwrap().lock().unwrap();
        let frame_n_1 = self.ring_buffer.buffer.get(reader_index_1).unwrap().lock().unwrap();
        
        let frame_diff = FrameDiffer::diff_of_frames(&(*frame_n), &(*frame_n_1));

        let image_did_blur = self.image_did_blur(frame_diff);
        if image_did_blur {
            self.new_best_frame_needed = true;
        }
        else if self.new_best_frame_needed && !image_did_blur {

            self.new_best_frame_needed = false;

            self.to_file_write.send(
                Mat::clone(&*frame_n).unwrap()
            ).unwrap();
        }
    }

    fn name(&self) -> &str {
        "Frame Differ"
    }

    fn priority(&self) -> i32 {
        scheduling::MAX_PRIORITY - 1
    }

    fn frequency(&self) -> u32 {
        20
    }
}

#[cfg(test)]
mod tests;