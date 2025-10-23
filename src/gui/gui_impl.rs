//! Implement Gui trait for common types

use std::sync::Arc;

use super::{Format, GraphFmt, GraphShow};
use eframe::egui::Color32;

use crate::gir::{IR, OpId, ValId};
use crate::ioplang;

type Context = Arc<IR<ioplang::Ioplang>>;
impl GraphFmt for (OpId, Context) {
    fn fmt_short(&self) -> String {
        let (id, ctx) = self;
        let operation = ctx.get_op(*id).get_operation();
        match operation {
            ioplang::Operations::Input { pos, .. } => format!("In<{pos}>"),
            ioplang::Operations::Output { pos, .. } => format!("Out<{pos}>"),
            ioplang::Operations::Variable { .. } => "Var".to_string(),
            ioplang::Operations::Constant { value } => match value {
                ioplang::Litteral::PlaintextBlock(val) => format!("Pt<{val}>"),
                ioplang::Litteral::Index(val) => format!("Idx<{val}>"),
            },
            ioplang::Operations::GenerateLut { .. } => "GenLut".to_string(),
            ioplang::Operations::ExtractCtBlock | ioplang::Operations::ExtractPtBlock => {
                "LD".to_string()
            }
            ioplang::Operations::StoreCtBlock => "ST".to_string(),
            dflt => format!("{dflt}"),
        }
    }

    fn fmt_long(&self) -> String {
        let (id, ctx) = self;
        let op_ref = ctx.get_op(*id);
        format!("{:?}::{}", op_ref.get_id(), op_ref.get_operation())
    }
}

impl GraphShow for (OpId, Context) {
    fn format(&self) -> super::Format {
        let (id, ctx) = self;
        let operation = ctx.get_op(*id).get_operation();
        match operation {
            // IO kind
            ioplang::Operations::Input { .. }
            | ioplang::Operations::Output { .. }
            | ioplang::Operations::Variable { .. }
            | ioplang::Operations::Constant { .. }
            | ioplang::Operations::GenerateLut { .. }
            | ioplang::Operations::ExtractCtBlock
            | ioplang::Operations::ExtractPtBlock
            | ioplang::Operations::StoreCtBlock => Format::Rectangle,
            // Arith kind
            ioplang::Operations::AddCt | ioplang::Operations::SubCt | ioplang::Operations::Mac => {
                Format::Ellipse
            }
            // ArithMsg kind
            ioplang::Operations::AddPt
            | ioplang::Operations::SubPt
            | ioplang::Operations::PtSub
            | ioplang::Operations::MulPt => Format::Ellipse,
            // Pbs kind
            ioplang::Operations::Pbs
            | ioplang::Operations::Pbs2
            | ioplang::Operations::Pbs4
            | ioplang::Operations::Pbs8 => Format::Circle,
        }
    }

    fn color(&self) -> Color32 {
        let (id, ctx) = self;
        let operation = ctx.get_op(*id).get_operation();
        match operation {
            // IO kind
            ioplang::Operations::Input { .. }
            | ioplang::Operations::Output { .. }
            | ioplang::Operations::Variable { .. }
            | ioplang::Operations::Constant { .. }
            | ioplang::Operations::GenerateLut { .. } => Color32::WHITE,

            ioplang::Operations::ExtractCtBlock
            | ioplang::Operations::ExtractPtBlock
            | ioplang::Operations::StoreCtBlock => Color32::GREEN,
            // Arith kind
            ioplang::Operations::AddCt | ioplang::Operations::SubCt | ioplang::Operations::Mac => {
                Color32::ORANGE
            }
            // ArithMsg kind
            ioplang::Operations::AddPt
            | ioplang::Operations::SubPt
            | ioplang::Operations::PtSub
            | ioplang::Operations::MulPt => Color32::YELLOW,
            // Pbs kind
            ioplang::Operations::Pbs
            | ioplang::Operations::Pbs2
            | ioplang::Operations::Pbs4
            | ioplang::Operations::Pbs8 => Color32::RED,
        }
    }
}

impl GraphFmt for (ValId, Context) {
    fn fmt_short(&self) -> String {
        let (id, ctx) = self;
        let typ = ctx.get_val(*id).get_type();
        format!("{}", typ)
    }

    fn fmt_long(&self) -> String {
        let (id, ctx) = self;
        format!("{:?}", ctx.get_val(*id))
    }
}
