use super::*;
use crate::{AnnOpRef, AnnValRef, Dialect, Formatted, IR, OpId, OpMap, ValId, ValMap};
use std::ops::Deref;
use zhc_utils::{Dumpable, iter::MultiZip, small::SmallVec};

/// IR container with parallel annotation storage for operations and values.
#[derive(Debug, Clone)]
pub struct AnnIR<'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    pub(crate) ir: &'ir IR<D>,
    pub(crate) op_annotations: OpMap<OpAnn>,
    pub(crate) val_annotations: ValMap<ValAnn>,
}

impl<'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> AnnIR<'ir, D, OpAnn, ValAnn> {
    /// Creates a new annotated IR with the given operation and value annotations.
    ///
    /// # Panics
    ///
    /// Panics if the annotation maps are not completely filled for all active operations and
    /// values.
    pub fn new(
        ir: &'ir IR<D>,
        op_annotations: OpMap<OpAnn>,
        val_annotations: ValMap<ValAnn>,
    ) -> Self {
        assert!(
            op_annotations.is_filled(),
            "Operation annotations map must be filled for all active operations"
        );
        assert!(
            val_annotations.is_filled(),
            "Value annotations map must be filled for all active values"
        );
        Self {
            ir,
            op_annotations,
            val_annotations,
        }
    }

    /// Returns a reference to the operation annotations map.
    pub fn op_annotations(&self) -> &OpMap<OpAnn> {
        &self.op_annotations
    }

    /// Returns a reference to the value annotations map.
    pub fn val_annotations(&self) -> &ValMap<ValAnn> {
        &self.val_annotations
    }

    /// Returns a mutable reference to the operation annotations map.
    pub fn op_annotations_mut(&mut self) -> &mut OpMap<OpAnn> {
        &mut self.op_annotations
    }

    /// Returns a mutable reference to the value annotations map.
    pub fn val_annotations_mut(&mut self) -> &mut ValMap<ValAnn> {
        &mut self.val_annotations
    }

    /// Returns an annotated operation reference for the specified operation.
    ///
    /// # Panics
    ///
    /// Panics if the operation ID does not exist or refers to an inactive operation.
    pub fn get_op(&self, opid: OpId) -> AnnOpRef<'ir, '_, D, OpAnn, ValAnn> {
        let opref = self.ir.get_op(opid);
        let ann = &self.op_annotations[opid];
        AnnOpRef {
            ann_ir: self,
            opref,
            ann,
        }
    }

    /// Returns an annotated value reference for the specified value.
    ///
    /// # Panics
    ///
    /// Panics if the value ID does not exist, refers to an inactive value.
    pub fn get_val(&self, valid: ValId) -> AnnValRef<'ir, '_, D, OpAnn, ValAnn> {
        let valref = self.ir.get_val(valid);
        let ann = &self.val_annotations[valid];
        AnnValRef {
            ann_ir: self,
            valref,
            ann,
        }
    }

    /// Returns an iterator over all active operations with annotations in linear order.
    pub fn walk_ops_linear(
        &self,
    ) -> impl DoubleEndedIterator<Item = AnnOpRef<'ir, '_, D, OpAnn, ValAnn>> {
        self.ir.walk_ops_linear().map(|opref| {
            let ann = &self.op_annotations[*opref];
            AnnOpRef {
                ann_ir: self,
                opref,
                ann,
            }
        })
    }

    /// Returns an iterator over all active operations with annotations in topological order.
    pub fn walk_ops_topological(
        &self,
    ) -> impl DoubleEndedIterator<Item = AnnOpRef<'ir, '_, D, OpAnn, ValAnn>> {
        self.ir.walk_ops_topological().map(|opref| {
            let ann = &self.op_annotations[*opref];
            AnnOpRef {
                ann_ir: self,
                opref,
                ann,
            }
        })
    }

    /// Returns an iterator over operations with annotations using a custom walker.
    pub fn walk_ops_with(
        &self,
        walker: impl Iterator<Item = OpId>,
    ) -> impl Iterator<Item = AnnOpRef<'ir, '_, D, OpAnn, ValAnn>> {
        self.ir.walk_ops_with(walker).map(|opref| {
            let ann = &self.op_annotations[*opref];
            AnnOpRef {
                ann_ir: self,
                opref,
                ann,
            }
        })
    }

    /// Returns an iterator over all active values with annotations in linear order.
    pub fn walk_vals_linear(
        &self,
    ) -> impl DoubleEndedIterator<Item = AnnValRef<'ir, '_, D, OpAnn, ValAnn>> {
        self.ir.walk_vals_linear().map(|valref| {
            let ann = &self.val_annotations[*valref];
            AnnValRef {
                ann_ir: self,
                valref,
                ann,
            }
        })
    }

    /// Returns an iterator over values with annotations using a custom walker.
    pub fn walk_vals_with(
        &self,
        walker: impl Iterator<Item = ValId>,
    ) -> impl Iterator<Item = AnnValRef<'ir, '_, D, OpAnn, ValAnn>> {
        self.ir.walk_vals_with(walker).map(|valref| {
            let ann = &self.val_annotations[*valref];
            AnnValRef {
                ann_ir: self,
                valref,
                ann,
            }
        })
    }

    /// Performs backward dataflow analysis with access to existing annotations.
    ///
    /// Operations are visited in reverse topological order. The callback
    /// receives the in-progress annotated ref (with [`Analysing`] wrappers)
    /// and the existing annotated ref from `self`, and must return an
    /// operation annotation and one value annotation per return value.
    ///
    /// # Panics
    ///
    /// Panics if the callback returns a number of value annotations that
    /// differs from the operation's return arity.
    pub fn backward_dataflow_analysis<OpAnnNew: Annotation, ValAnnNew: Annotation>(
        &self,
        mut f: impl FnMut(
            AnnOpRef<D, Analysing<OpAnnNew>, Analysing<ValAnnNew>>,
            &AnnOpRef<D, OpAnn, ValAnn>,
        ) -> (OpAnnNew, SmallVec<ValAnnNew>),
    ) -> AnnIR<'ir, D, OpAnnNew, ValAnnNew> {
        let mut ann_ir = AnnIR {
            op_annotations: self.filled_opmap(Analysing::Pending),
            val_annotations: self.filled_valmap(Analysing::Pending),
            ir: self.ir,
        };
        for opref in self.walk_ops_topological().rev() {
            let (opann, valanns) = f(ann_ir.get_op(**opref), &opref);
            assert_eq!(valanns.len(), opref.get_return_valids().len());
            assert!(matches!(
                ann_ir
                    .op_annotations
                    .insert(**opref, Analysing::Analyzed(opann)),
                Some(Analysing::Pending)
            ));
            for (valann, valref) in (valanns.into_iter(), opref.get_return_valids().iter()).mzip() {
                assert!(matches!(
                    ann_ir
                        .val_annotations
                        .insert(*valref, Analysing::Analyzed(valann)),
                    Some(Analysing::Pending)
                ));
            }
        }
        AnnIR {
            ir: self.ir,
            op_annotations: ann_ir.op_annotations.map(Analysing::unwrap_analyzed),
            val_annotations: ann_ir.val_annotations.map(Analysing::unwrap_analyzed),
        }
    }

    /// Performs forward dataflow analysis with access to existing annotations.
    ///
    /// Operations are visited in topological order. The callback receives the
    /// in-progress annotated ref (with [`Analysing`] wrappers) and the
    /// existing annotated ref from `self`, and must return an operation
    /// annotation and one value annotation per return value.
    ///
    /// # Panics
    ///
    /// Panics if the callback returns a number of value annotations that
    /// differs from the operation's return arity.
    pub fn forward_dataflow_analysis<OpAnnNew: Annotation, ValAnnNew: Annotation>(
        &self,
        mut f: impl FnMut(
            AnnOpRef<D, Analysing<OpAnnNew>, Analysing<ValAnnNew>>,
            &AnnOpRef<D, OpAnn, ValAnn>,
        ) -> (OpAnnNew, SmallVec<ValAnnNew>),
    ) -> AnnIR<'ir, D, OpAnnNew, ValAnnNew> {
        let mut ann_ir = AnnIR {
            op_annotations: self.filled_opmap(Analysing::Pending),
            val_annotations: self.filled_valmap(Analysing::Pending),
            ir: self.ir,
        };
        for opref in self.walk_ops_topological() {
            let (opann, valanns) = f(ann_ir.get_op(**opref), &opref);
            assert_eq!(valanns.len(), opref.get_return_valids().len());
            assert!(matches!(
                ann_ir
                    .op_annotations
                    .insert(**opref, Analysing::Analyzed(opann)),
                Some(Analysing::Pending)
            ));
            for (valann, valref) in (valanns.into_iter(), opref.get_return_valids().iter()).mzip() {
                assert!(matches!(
                    ann_ir
                        .val_annotations
                        .insert(*valref, Analysing::Analyzed(valann)),
                    Some(Analysing::Pending)
                ));
            }
        }
        AnnIR {
            ir: self.ir,
            op_annotations: ann_ir.op_annotations.map(Analysing::unwrap_analyzed),
            val_annotations: ann_ir.val_annotations.map(Analysing::unwrap_analyzed),
        }
    }

    /// Transforms operation annotations using the provided function.
    pub fn map_opann<OpAnnNew: Annotation>(
        &self,
        mut f: impl FnMut(&AnnOpRef<D, OpAnn, ValAnn>) -> OpAnnNew,
    ) -> AnnIR<'ir, D, OpAnnNew, ValAnn> {
        let mut opmap = self.empty_opmap();
        for opref in self.walk_ops_linear() {
            let new_opann = f(&opref);
            assert!(opmap.insert(**opref, new_opann).is_none());
        }
        AnnIR::new(self.ir, opmap, self.val_annotations.clone())
    }

    /// Transforms value annotations using the provided function.
    pub fn map_valann<ValAnnNew: Annotation>(
        &self,
        mut f: impl FnMut(&AnnValRef<D, OpAnn, ValAnn>) -> ValAnnNew,
    ) -> AnnIR<'ir, D, OpAnn, ValAnnNew> {
        let mut valmap = self.empty_valmap();
        for valref in self.walk_vals_linear() {
            let new_valann = f(&valref);
            assert!(valmap.insert(**valref, new_valann).is_none());
        }
        AnnIR::new(self.ir, self.op_annotations.clone(), valmap)
    }

    /// Creates a configurable formatter for the annotated IR.
    pub fn format(&self) -> Formatted<'_, Self> {
        Formatted::new(self)
    }

    /// Consumes the annotated IR and returns the operation annotations map.
    pub fn into_opmap(self) -> OpMap<OpAnn> {
        self.op_annotations
    }

    /// Consumes the annotated IR and returns the value annotations map.
    pub fn into_valmap(self) -> ValMap<ValAnn> {
        self.val_annotations
    }

    /// Consumes the annotated IR and returns both annotation maps.
    pub fn into_maps(self) -> (OpMap<OpAnn>, ValMap<ValAnn>) {
        (self.op_annotations, self.val_annotations)
    }
}

impl<'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Deref
    for AnnIR<'ir, D, OpAnn, ValAnn>
{
    type Target = IR<D>;

    fn deref(&self) -> &Self::Target {
        self.ir
    }
}

impl<'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Dumpable
    for AnnIR<'ir, D, OpAnn, ValAnn>
{
    fn dump_to_string(&self) -> String {
        format!(
            "{}",
            self.format()
                .with_walker(crate::PrintWalker::Linear)
                .show_types(false)
                .show_opid(true)
                .show_comments(true)
        )
    }
}
