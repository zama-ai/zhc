use std::fmt::Display;

use serde::Serialize;
use zhc_langs::hpulang::HpuId;

use crate::{Event, hpu::DOp};

use super::super::hpu::Events as HpuEvents;

/// Simulation events representing state changes and operations within a multi-HPU system.
///
/// Board-level events travel wrapped in `Hpu`, tagged with the [`HpuId`] of the board they
/// concern, whereas `PushDOps` and `ProcessOver` are system-wide and belong to no single board.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Events {
    /// Board-level event concerning the identified HPU.
    Hpu(HpuId, HpuEvents),
    /// Injects one DOp stream per HPU, in board order, starting the run.
    PushDOps(Vec<Vec<DOp>>),
    /// Every HPU has starved: the simulation reached completion.
    ProcessOver,
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Events::Hpu(id, hpu_event) => write!(f, "Hpu({}, {hpu_event})", id.0),
            Events::PushDOps(_) => write!(f, "PushDOps"),
            Events::ProcessOver => write!(f, "ProcessOver"),
        }
    }
}

impl Event for Events {}
