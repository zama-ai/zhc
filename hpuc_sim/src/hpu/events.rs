use std::fmt::Display;

use serde::Serialize;

use crate::Event;

use super::{DOp, DOpId, IscCommand};

/// Number of operations processed together in a batch.
pub type BatchSize = usize;

/// Simulation events representing state changes and operations within HPU components.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Events {
    /// Instruction scheduler receives new operations to schedule.
    IscPushDOps(Vec<DOp>),
    /// Instruction scheduler issues an operation to a processing element.
    IscIssueDOp(DOp),
    /// Unlocks read access for the specified operation.
    IscUnlockRead(DOpId),
    /// Unlocks write access for the specified operation.
    IscUnlockWrite(DOpId),
    /// Unlocks issue capability for the specified operation.
    IscUnlockIssue(DOpId),
    /// Retires a completed operation from the scheduler.
    IscRetireDOp(DOp),
    /// Periodic scheduler query for available operations.
    IscQuery,
    /// Refills an operation back into the scheduler queue.
    IscRefillDOp(DOp),
    /// Signals completion of all scheduled operations.
    IscProcessOver,

    /// ALU processing element begins operation execution.
    PeAluLaunchProcessing,
    /// ALU processing element completes operation execution.
    PeAluLandProcessing,
    /// ALU processing element becomes available for new operations.
    PeAluAvailable,
    /// ALU processing element becomes unavailable for new operations.
    PeAluUnavailable,

    /// Memory processing element begins operation execution.
    PeMemLaunchProcessing,
    /// Memory processing element completes operation execution.
    PeMemLandProcessing,
    /// Memory processing element becomes available for new operations.
    PeMemAvailable,
    /// Memory processing element becomes unavailable for new operations.
    PeMemUnavailable,

    /// PBS processing element begins loading data into memory.
    PePbsLaunchLoadMemory,
    /// PBS processing element completes loading operation data.
    PePbsLandLoadMemory(DOp),
    /// PBS processing element begins unloading data from memory.
    PePbsLaunchUnloadMemory,
    /// PBS processing element completes unloading operation data.
    PePbsLandUnloadMemory(DOpId),
    /// PBS processing element begins batch processing with specified size.
    PePbsLaunchProcessing(BatchSize),
    /// PBS processing element completes batch processing.
    PePbsLandProcessing(BatchSize),
    /// PBS processing element timeout.
    PePbsTimeout,
    /// PBS processing element becomes available for new operations.
    PePbsAvailable,
    /// PBS processing element becomes unavailable for new operations.
    PePbsUnavailable,


    /// External notification of scheduler command execution.
    NotifyIsc(DOpId, IscCommand),
    /// External notification
    NotifyStartOnTimeout{last_in: DOp},
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Events::IscPushDOps(_) => write!(f, "IscPushDOps"),
            Events::IscIssueDOp(_) => write!(f, "IscIssueDOp"),
            Events::IscUnlockRead(_) => write!(f, "IscUnlockRead"),
            Events::IscUnlockWrite(_) => write!(f, "IscUnlockWrite"),
            Events::IscUnlockIssue(_) => write!(f, "IscUnlockIssue"),
            Events::IscRetireDOp(_) => write!(f, "IscRetireDOp"),
            Events::IscQuery => write!(f, "IscQuery"),
            Events::IscRefillDOp(_) => write!(f, "IscQueryRefill"),
            Events::IscProcessOver => write!(f, "IscProcessOver"),
            Events::PeAluLandProcessing => write!(f, "PeAluLandProcessing"),
            Events::PeAluLaunchProcessing => write!(f, "PeAluLaunchProcessing"),
            Events::PeMemLandProcessing => write!(f, "PeMemLandProcessing"),
            Events::PeMemLaunchProcessing => write!(f, "PeMemLaunchProcessing"),
            Events::PePbsLaunchLoadMemory => write!(f, "PePbsLaunchLoadMemory"),
            Events::PePbsLandLoadMemory(_) => write!(f, "PePbsLandLoadMemory"),
            Events::PePbsLaunchProcessing(_) => write!(f, "PePbsLaunchProcessing"),
            Events::PePbsLandProcessing(_) => write!(f, "PePbsLandProcessing"),
            Events::PePbsLaunchUnloadMemory => write!(f, "PePbsLaunchUnloadMemory"),
            Events::PePbsLandUnloadMemory(_) => write!(f, "PePbsLandUnloadMemory"),
            Events::PePbsTimeout => write!(f, "PePbsTimeout"),
            Events::PePbsAvailable => write!(f, "PePbsAvailable"),
            Events::PePbsUnavailable => write!(f, "PePbsUnavailable"),
            Events::PeAluAvailable => write!(f, "PeAluAvailable"),
            Events::PeAluUnavailable => write!(f, "PeAluUnavailable"),
            Events::PeMemAvailable => write!(f, "PeMemAvailable"),
            Events::PeMemUnavailable => write!(f, "PeMemUnavailable"),
            Events::NotifyIsc(_, _) => write!(f, "NotifyIsc"),
            Events::NotifyStartOnTimeout { .. } => write!(f, "NotifyStartOnTimeout"),
        }
    }
}

impl Event for Events {}
