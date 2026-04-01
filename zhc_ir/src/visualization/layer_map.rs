use zhc_utils::small::SmallVec;
use zhc_utils::{Dumpable, SafeAs};

use crate::visualization::LayoutDialect;
use crate::{IR, OpId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerWalker(Vec<OpId>);

impl LayerWalker {
    pub fn walker(&self) -> impl Iterator<Item = OpId> {
        self.0.iter().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayersMap(SmallVec<LayerWalker>);

impl Dumpable for LayersMap {
    fn dump_to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("LayersMap {\n");
        for (depth, layer) in self.0.iter().enumerate() {
            result.push_str(&format!("  Layer {}: [", depth));
            let ops: Vec<String> = layer.0.iter().map(|id| format!("{:?}", id)).collect();
            result.push_str(&ops.join(", "));
            result.push_str("]\n");
        }
        result.push_str("}");
        result
    }
}

impl LayersMap {
    pub fn extract_from_ir(ir: &IR<LayoutDialect>) -> Self {
        let mut output = LayersMap(SmallVec::new());
        for op in ir.walk_ops_linear() {
            let depth = op.get_depth();
            while output.0.len() < depth.sas::<usize>() + 1 {
                output.0.push(LayerWalker(Vec::new()));
            }
            output.0[depth.sas::<usize>() - 1].0.push(op.get_id());
        }
        output
    }

    pub fn iter_layers(&self) -> impl DoubleEndedIterator<Item = &LayerWalker> {
        self.0.iter()
    }
}
