//! Implement Gui trait for common types

use eframe::egui::Color32;

use super::{Format, GraphFmt, GraphShow};
use crate::ir::{IrCell, IrOperation, OpKind};

impl GraphFmt for IrOperation {
    fn fmt_short(&self) -> String {
        match self {
            IrOperation::Load { .. } => "LD".to_string(),
            IrOperation::Store { .. } => "ST".to_string(),
            IrOperation::Sync {} => "SYNC".to_string(),
            IrOperation::Add { .. } => "ADD".to_string(),
            IrOperation::Sub { .. } => "SUB".to_string(),
            IrOperation::Mac { .. } => "MAC".to_string(),
            IrOperation::Adds { .. } => "ADDS".to_string(),
            IrOperation::Subs { .. } => "SUBS".to_string(),
            IrOperation::Ssub { .. } => "SSUB".to_string(),
            IrOperation::Muls { .. } => "MULS".to_string(),
            IrOperation::Pbs { flush, .. } => {
                format!("PBS{}", if *flush { "_F" } else { "" })
            }
        }
    }

    fn fmt_long(&self) -> String {
        format!("{}", self)
    }
}

impl GraphShow for IrOperation {
    fn format(&self) -> super::Format {
        match OpKind::from(self) {
            OpKind::Mem => Format::Rectangle,
            OpKind::Arith => Format::Ellipse,
            OpKind::ArithMsg => Format::Ellipse,
            OpKind::Pbs => Format::Circle,
            OpKind::Ucore => Format::Rectangle,
        }
    }

    fn color(&self) -> Color32 {
        match OpKind::from(self) {
            OpKind::Mem => Color32::GREEN,
            OpKind::Arith => Color32::ORANGE,
            OpKind::ArithMsg => Color32::YELLOW,
            OpKind::Pbs => Color32::RED,
            OpKind::Ucore => Color32::WHITE,
        }
    }
}

impl GraphFmt for IrCell {
    fn fmt_short(&self) -> String {
        format!("{}", self)
    }

    fn fmt_long(&self) -> String {
        format!("{:?}", self)
    }
}
