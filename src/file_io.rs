use std::sync::mpsc::Receiver;
use rscam::Frame;

mod real_time;
use real_time::RealTime;

struct FileDiffer {
    from_camera: Receiver<Frame>
}

impl RealTime for FileDiffer {
    fn priority(&self) -> i32 {
        real_time::MAX_PRIORITY - 1
    }

    fn frequency(&self) -> u32 {
        20
    }

    fn service(&self) {
        
    }
}