pub const MAX_PRIORITY: i32 = 99;

use scheduler::Policy;
use core_affinity;

use std::thread;
use std::sync::{mpsc::channel, Arc};
use std::time::{Instant, Duration};

use log::{error, info};

pub trait RealTime {

    fn name(&self) -> &str;

    fn service(&mut self);

    fn priority(&self) -> i32;

    /// Should return the frequency that the service should run in hertz
    fn frequency(&self) -> u32;

    fn period(&self) -> Duration {
        let mut period = 1.0/(self.frequency() as f64);

        // converting to nano seconds
        period *= 1e9;

        Duration::from_nanos(period as u64)
    }

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

pub struct Sequencer;

impl RealTime for Sequencer {

    fn name (&self) -> &str {
        "sequencer"
    }

    fn priority(&self) -> i32 {
        MAX_PRIORITY
    }

    fn frequency(&self) -> u32 {
        100
    }

    fn service(&mut self) {
        
    }
}

impl Sequencer {

    pub fn sequence(&self, services: Vec<Box<RealTime + Send>>, stop_time: Duration) {
        let mut tx_channels = Vec::with_capacity(services.len());
        let universal_clock = Arc::new(Instant::now());

        let service_frequencies: Vec<u32> = services.iter()
            .map(|real_time_object| {
                real_time_object.frequency()
            })
            .collect();
        
        for mut service in services {
            // Setting up communication channels
            let (tx, rx) = channel();
            tx_channels.push(tx);

            // Giving threads their own reference to objects and universal clock
            let universal_clock = universal_clock.clone();

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
                            info!("{},T,{:?}, time_elapsed, {:?}", service.name(), 
                                service.period(),    
                                universal_clock.elapsed()
                            );
                                service.service();
                        }
                        SequencerCommand::Exit => break
                    }
                }
            });
        }

        let time_capturing = Instant::now();
        let mut sequence_count = 0;
        let start_time = universal_clock.elapsed();

        while time_capturing.elapsed() < stop_time {

            for (service_number, service_frequency) in service_frequencies.iter().enumerate() {
                if sequence_count % (self.frequency()/service_frequency) == 0 {
                    tx_channels[service_number].send(
                        SequencerCommand::ProvideService
                    )
                    .unwrap();
                }
            }

            info!("{},time_elapsed,{:?}", 
                self.name(),universal_clock.elapsed());

            let time_error = start_time + time_capturing.elapsed() - 
                self.period() * sequence_count;

            thread::sleep(self.period() - time_error);
            
            sequence_count += 1;
            
        }
        
        // We are done! Sending message to threads to stop working
        for tx in tx_channels {
            tx.send(SequencerCommand::Exit).unwrap();
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