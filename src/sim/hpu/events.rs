use std::fmt::Display;

use serde::Serialize;

use crate::sim::Event;

use super::{DOp, DOpId, TimeoutId};

pub type BatchSize = usize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Events {
    // Isc Events,
    IscPushDOps(Vec<DOp>),
    IscIssueDOp(DOp),
    IscUnlockRead(DOpId),
    IscUnlockWrite(DOpId),
    IscUnlockIssue(DOpId),
    IscUnlockSync(DOpId),
    IscRetireDOp(DOp),
    IscQuery,
    IscQueryUnlockRead,
    IscQueryUnlockWrite,
    IscQueryUnlockIssue,
    IscQueryRefill,
    IscQueryIssue,
    IscProcessOver,

    // PeAluAvents
    PeAluLaunchProcessing,
    PeAluLandProcessing,
    PeAluAvailable,
    PeAluUnavailable,

    // PeMemAvents
    PeMemLaunchProcessing,
    PeMemLandProcessing,
    PeMemAvailable,
    PeMemUnavailable,

    // PePbs Events
    PePbsLaunchLoadMemory,
    PePbsLandLoadMemory(DOp),
    PePbsLaunchUnloadMemory,
    PePbsLandUnloadMemory(DOpId),
    PePbsLaunchProcessing(BatchSize),
    PePbsLandProcessing(BatchSize),
    PePbsTimeout(TimeoutId),
    PePbsAvailable,
    PePbsUnavailable,
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Events::IscPushDOps(_) => write!(f, "IscPushDOps"),
            Events::IscIssueDOp(_) => write!(f, "IscIssueDOp"),
            Events::IscUnlockRead(_) => write!(f, "IscUnlockRead"),
            Events::IscUnlockWrite(_) => write!(f, "IscUnlockWrite"),
            Events::IscUnlockIssue(_) => write!(f, "IscUnlockIssue"),
            Events::IscUnlockSync(_) => write!(f, "IscUnlockSync"),
            Events::IscRetireDOp(_) => write!(f, "IscRetireDOp"),
            Events::IscQuery => write!(f, "IscQuery"),
            Events::IscQueryUnlockRead => write!(f, "IscQueryUnlockRead"),
            Events::IscQueryUnlockWrite => write!(f, "IscQueryUnlockWrite"),
            Events::IscQueryUnlockIssue => write!(f, "IscQueryUnlockIssue"),
            Events::IscQueryRefill => write!(f, "IscQueryRefill"),
            Events::IscQueryIssue => write!(f, "IscQueryIssue"),
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
            Events::PePbsTimeout(_) => write!(f, "PePbsTimeout"),
            Events::PePbsAvailable => write!(f, "PePbsAvailable"),
            Events::PePbsUnavailable => write!(f, "PePbsUnavailable"),
            Events::PeAluAvailable => write!(f, "PeAluAvailable"),
            Events::PeAluUnavailable => write!(f, "PeAluUnavailable"),
            Events::PeMemAvailable => write!(f, "PeMemAvailable"),
            Events::PeMemUnavailable => write!(f, "PeMemUnavailable"),
        }
    }
}

impl Event for Events {}
