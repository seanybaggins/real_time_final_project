
#[cfg(test)]
use crate::imaging;
use crate::RealTime;
use crate::ring_buffer::RingBuffer;
use std::num::Wrapping;
use std::sync::{Mutex, Arc};
use std::time::{Duration, Instant};

pub(self) fn set_up() -> Box<RealTime + Send> {
    let ring_buffer = Arc::new(RingBuffer::with_capacity(10));
    let ring_read_write_count = Arc::new(Mutex::new((Wrapping(1), Wrapping(0))));

    let camera: Box<RealTime + Send> = Box::new(imaging::Camera::new(
        ring_buffer.clone(),
        ring_read_write_count.clone()
    ));

    camera.real_time_setup();

    return camera;
}

#[test]
fn camera_wcet() {
    let mut camera = set_up();

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

}