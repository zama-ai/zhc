use std::fmt::Display;

use serde::Serialize;
use zhc_langs::hpulang::HpuId;

use crate::{Event, hpu::DOp};

use super::super::hpu::Events as HpuEvents;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Events {
    Hpu(HpuId, HpuEvents),
    PushDOps(Vec<Vec<DOp>>),
    ProcessOver,
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Events::Hpu(id, hpu_event) => write!(f, "Hpu({}, {hpu_event})", id.0),
            Events::PushDOps(_) => write!(f, "PushDOps"),
            Events::ProcessOver => write!(f, "ProcessOver")
        }
    }
}


impl Event for Events {}
