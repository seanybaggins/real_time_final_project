mod imaging;

use imaging::Camera;

struct RingBuffer {
    mut frames: Vec<Option<Arc<Mutex<Frame>>>>>
}

impl RingBuffer {
    pub fn new() -> Self {

        let buffer_size = 10;
        let ring_buffer: Vec<Option<Arc<Mutex<Frame>>>> = Vec::with_capacity(
            mem::size_of::<Option<Arc<Mutex<Frame>>>>() * buffer_size as usize
        );

        // Capture same frames 
        for index in (0..buffer_size) {

        }
    }

    pub fn length(&self) -> u32 {
        
    } 
}