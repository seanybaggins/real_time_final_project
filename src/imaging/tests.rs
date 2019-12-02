
#[cfg(test)]
use crate::imaging::{Camera, FrameDiffer};
use crate::imaging;

use crate::RealTime;
use crate::ring_buffer::RingBuffer;
use std::num::Wrapping;
use std::sync::{Mutex, Arc};
use std::time::{Duration, Instant};
use opencv::core;
use opencv::core::Mat;

pub(self) fn set_up() -> (Camera, FrameDiffer) {
    let ring_buffer = Arc::new(RingBuffer::with_capacity(10));
    let ring_read_write_count = Arc::new(Mutex::new((Wrapping(1), Wrapping(0))));
    let best_frame = Arc::new(Mutex::new(None));

    let camera = Camera::new(
        ring_buffer.clone(),
        ring_read_write_count.clone()
    );

    let file_differ = FrameDiffer::new(
        ring_buffer.clone(), 
        ring_read_write_count.clone(), 
        best_frame.clone()
    );

    return (camera, file_differ);
}

#[test]
fn camera_wcet() {
    let (mut camera, _) = set_up();

    // WCET = Worst Case Execution Time
    let mut wcet = Duration::new(0, 0);
    for _ in 0..500 {
        let start = Instant::now();
        camera.service();
        let end = start.elapsed();

        if end > wcet {
            wcet = end;
        }
    }

    println!("WCET: {:?}", wcet);
    assert!(wcet < camera.period(), "Requested period is not within WCET");
}

#[test]
fn frame_convert_gray_scale() {
    let (mut camera, _) = set_up();
    let mut frame = opencv::core::Mat::default().unwrap();
    camera.hardware.read(&mut frame).unwrap();

    let mut gray_frame = imaging::convert_to_grayscale(&frame);

    imaging::show_frame(&mut gray_frame);

    print!("");

}

#[test]
fn diff_of_frames() {
    let (mut camera, _) = set_up();
    let mut frame0 = opencv::core::Mat::default().unwrap();
    let mut frame1 = opencv::core::Mat::default().unwrap();
    camera.hardware.read(&mut frame0).unwrap();
    camera.hardware.read(&mut frame1).unwrap();
    
    imaging::FrameDiffer::diff_of_frames(&mut frame0, &mut frame1);
    
}