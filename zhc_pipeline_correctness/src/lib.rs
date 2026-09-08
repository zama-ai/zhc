//! Correctness and snapshot tests for compilation passes using builder-generated circuits.
#![cfg(test)]

pub mod equivalence_check;

mod hpu_allocate_register;
mod hpu_batch_legacy;
mod hpu_generate_translation_table;
mod hpu_schedule;
mod hpu_schedule_legacy;
mod lower_iop_to_hpu;
