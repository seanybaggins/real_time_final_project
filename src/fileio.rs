use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;

use crate::imaging;

use opencv;
use opencv::core::{Mat, Point, FONT_HERSHEY_PLAIN, Scalar, LINE_8};
use opencv::imgcodecs::IMWRITE_PXM_BINARY;
use opencv::types::VectorOfint;
use opencv::prelude::*;

use crate::scheduling::best_effort;



pub fn backround_write_files(from_frame_select: Receiver<Mat>, universal_clock: Arc<Instant>) {
    thread::spawn(move || {
        best_effort::set_cpu_affinity();
        let mut frame_count = 0;
        
        loop {
            let best_frame = from_frame_select.recv();
            
            match best_frame {
                Ok(mut best_frame) => {
                    // write clock time on image
                    opencv::imgproc::put_text(
                        &mut best_frame,
                        format!("frame{:04}, Time: {:?}", frame_count, universal_clock.elapsed()).as_str(),
                        Point::new(15,15), // Starting location of string
                        FONT_HERSHEY_PLAIN, // Font type
                        12.0, // Font Scale
                        Scalar::new(0.0,0.0,0.0,0.0),
                        1, // Thickness
                        LINE_8,
                        false, // Bottom left origin
                    ).unwrap();

                    imaging::show_frame(&mut best_frame);
                    
                    opencv::imgcodecs::imwrite(
                        format!("frame{:04}", frame_count).as_str(),
                        &mut best_frame,
                        &VectorOfint::from_iter(vec!(IMWRITE_PXM_BINARY, 1))
                    ).unwrap();
                    
                    frame_count += 1;
                }
                Err(_) => {
                    break;
                }
            }

        }
    });
}
