use opencv;
use std::sync::Mutex;


pub struct RingBuffer {
    pub buffer: Vec<Mutex<opencv::prelude::Mat>>,
}

impl RingBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut ring_buffer = Vec::with_capacity(capacity);
        for _ in 0..ring_buffer.capacity(){
            let frame_matrix = Mutex::new(opencv::core::Mat::default()
                .expect("unable to initialize ring buffer frames")
            );
            ring_buffer.push(frame_matrix);
        };

        RingBuffer {
            buffer: ring_buffer,
        }
    }
}
