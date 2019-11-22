pub const MAX_PRIORITY: i32 = 99;

use scheduler::Policy;
use core_affinity;

use std::thread;
use std::sync::{mpsc::channel, Arc};
use std::time::{Instant, Duration};

use log::{error, info};

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

pub struct Sequencer {}

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

    pub fn sequence(&self, services: Vec<Arc<RealTime + Send + Sync>>) {
        let mut tx_channels = Vec::with_capacity(services.len());

        for service in services.iter() {
            // Setting up communication channels
            let (tx, rx) = channel();
            tx_channels.push(tx);

            // Giving threads their own reference to objects
            let service = service.clone();

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

        let start_time = Instant::now();
        let stop_time = Duration::from_secs(1800);
        let mut sequence_count = 0;

        while start_time.elapsed() < stop_time {
            for (service_number, service) in services.iter().enumerate() {
                if sequence_count % service.frequency() == 0 {
                    tx_channels[service_number].send(
                        SequencerCommand::ProvideService
                    )
                    .unwrap();
                } 
            }
        }
    }

    pub fn new() -> Self {
        let sequencer = Sequencer {};
        sequencer.real_time_setup();

        return sequencer;
    }
    
}

pub enum SequencerCommand {
    ProvideService,
    Exit
}