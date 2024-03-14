#![no_std]

use task::{current, TaskRef};
use crate::run_queue::{RUN_QUEUE, AxRunQueue};
use spinlock::SpinNoIrq;

#[macro_use]
extern crate log;
extern crate alloc;
mod run_queue;

pub fn init() {
    RUN_QUEUE.init_by(AxRunQueue::new());
}

pub fn task_rq(_task: &TaskRef) -> &SpinNoIrq<AxRunQueue> {
    &RUN_QUEUE
}

pub fn force_unlock() {
    unsafe { RUN_QUEUE.force_unlock() }
}