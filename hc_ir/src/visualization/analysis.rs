use hc_utils::svec;

use crate::{AnnIR, Dialect, IR};

/// The height of an op is the largest distance between this op and an output/effect operation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Height(pub(super) u16);

impl Height {
    pub fn inc(self) -> Self {
        Height(self.0 + 1)
    }

    #[allow(unused)]
    pub fn dec(self) -> Self {
        Height(self.0 - 1)
    }

    pub fn to_layer(self, ir_depth: u16) -> Layer {
        Layer(ir_depth - self.0 - 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Layer(u16);

impl Layer {
    pub fn range_inclusive(from: Layer, to: Layer) -> impl Iterator<Item = Layer> {
        (from.0..=to.0).map(Into::into).map(Layer)
    }

    pub fn above(mut self) -> Self {
        self.0 -= 1;
        self
    }

    pub fn below(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

pub fn analyze<D: Dialect>(ir: &IR<D>) -> AnnIR<'_, D, Layer, ()> {
    let depth = ir.depth();
    ir.backward_dataflow_analysis::<Height, ()>(|opmap, _, opref| {
        let height = opref
            .get_users_iter()
            .map(|user| opmap.get(&user).unwrap().0)
            .max()
            .map(|a| Height(a).inc())
            .unwrap_or(Height(0));
        (height, svec![(); opref.get_return_valids().len()])
    })
    .map_opann(|opref| opref.get_annotation().to_layer(depth + 1))
    .map_opann(|opref| std::cmp::min(Layer(*opref.depth), *opref.get_annotation()))
    .backward_dataflow_analysis::<Layer, ()>(|opmap, _, opref| {
        let min = opref
            .get_users_iter()
            .map(|u| opmap.get(&u).unwrap())
            .min()
            .unwrap_or(opref.get_annotation());
        (
            std::cmp::max(min.above(), *opref.get_annotation()),
            svec![(); opref.get_return_valids().len()],
        )
    })
}
