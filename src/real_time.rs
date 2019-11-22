pub const MAX_PRIORITY: i32 = 99;

use scheduler::Policy;
use core_affinity;

use std::thread;
use std::sync::mpsc::channel;

use log::error;

pub trait RealTime {

    fn priority(&self) -> i32;

    fn frequency(&self) -> u32;

    fn service(&self);

    fn set_cpu_affinity(&self) {
        let core_ids = core_affinity::get_core_ids().expect("Failed to get available cores");
        core_affinity::set_for_current(core_ids[0]);
    }

    fn set_policy(&self) {
        scheduler::set_self_policy(Policy::Fifo, self.priority()).unwrap();
    }

    fn real_time_setup(&self) {
        self.set_cpu_affinity();
        self.set_policy();
    }
}

pub struct Sequencer {
    services: Vec<Box<RealTime + Send>>
}

impl RealTime for Sequencer {

    fn priority(&self) -> i32 {
        MAX_PRIORITY
    }

    fn frequency(&self) -> u32 {
        100
    }

    fn service(&self) {
        println!("hello");
    }

    
}

impl Sequencer {
    pub fn sequence(&self, mut services: Vec<Box<RealTime + Send>>)  {

        for service in services {
            let (tx, rx) = channel();
            thread::spawn(move || {
                
                service.real_time_setup();
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
                            service.service();
                        }
                        SequencerCommand::Exit => break
                    }
                }
            });
        }
    }

    pub fn new(services: Vec<Box<RealTime + Send>>) -> Self {
        Sequencer {
            services
        }
    }
}

pub enum SequencerCommand {
    ProvideService,
    Exit
}