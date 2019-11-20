pub const MAX_PRIORITY: i32 = 99;

use scheduler::Policy;
use core_affinity;

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

pub enum SequencerCommand {
    ProvideService,
    Exit
}