pub const MAX_PRIORITY: i32 = 99;

use scheduler::Policy;
use core_affinity;

use std::thread;
use std::sync::{mpsc::channel, Arc};

use log::error;

pub(super) trait RealTime {

    fn priority() -> i32;

    fn frequency() -> u32;

    fn set_cpu_affinity() {
        let core_ids = core_affinity::get_core_ids().expect("Failed to get available cores");
        core_affinity::set_for_current(core_ids[0]);
    }

    fn set_policy() {
        scheduler::set_self_policy(Policy::Fifo, Self::priority()).unwrap();
    }

    fn real_time_setup() {
        Self::set_cpu_affinity();
        Self::set_policy();
    }
}

pub struct Sequencer;

impl RealTime for Sequencer {

    fn priority() -> i32 {
        MAX_PRIORITY
    }

    fn frequency() -> u32 {
        100
    }
    
}

impl Sequencer {
    pub fn sequence<F>(&self, mut services: Vec<Arc<F>>) 
    where F: Fn() + Send + Sync + 'static {
        
        for service in services.iter_mut() {
            let (tx, rx) = channel();
            let service = service.clone();
            thread::spawn(move || {

                loop {
                    let sequencer_command = match rx.recv() {
                        Ok(sequencer_command) => sequencer_command,
                        Err(error) =>  {
                            error!("camera failed to receive command: {}", error);
                            continue;
                        }
                    };
        
                    match sequencer_command {
                        SequencerCommand::ProvideService => {
                            service();
                        }
                        SequencerCommand::Exit => break
                    }
                }
            });
        }
    }

    pub fn new() -> Self {
        Sequencer{}
    }
}

pub enum SequencerCommand {
    ProvideService,
    Exit
}

/*
pub fn provide<F>(service: F) 
    where F: FnMut() + Send + Sync + 'static {
        service()
}
*/