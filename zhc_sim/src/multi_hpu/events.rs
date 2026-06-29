use std::fmt::Display;

use serde::Serialize;

use crate::{Event, hpu::{HpuId, TransferId}};

use super::super::hpu::Events as HpuEvents;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Events {
    Hpu(HpuId, HpuEvents),
    TransferOut(TransferId),
    TransferIn(TransferId)
}

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Events::Hpu(id, hpu_event) => write!(f, "Hpu({}, {hpu_event})", id.0),
            Events::TransferIn(tid) => write!(f, "TransferIn({})", tid.0),
            Events::TransferOut(tid) => write!(f, "TransferOut({})", tid.0),
        }
    }
}


impl Event for Events {}
