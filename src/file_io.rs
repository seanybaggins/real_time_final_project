use std::sync::mpsc::Receiver;
use rscam::Frame;

mod real_time;
use real_time::RealTime;

struct FileDiffer {
    from_camera: Receiver<Frame>
}

impl RealTime for FileDiffer {
    fn service(&self) {
        println!("", )
    }
}