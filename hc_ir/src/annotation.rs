//! Annotated intermediate representation providing type-safe access to IR elements with custom annotations.
//!
//! This module extends the base IR with annotation capabilities, allowing arbitrary metadata to be
//! attached to operations and values. The core types `AnnOpRef` and `AnnValRef` wrap their base
//! counterparts while providing access to associated annotations through a unified interface.
//!
//! The `AnnIR` container maintains parallel annotation maps alongside the base IR, ensuring
//! type safety and consistent access patterns. All navigation methods preserve annotation
//! context, returning annotated references that combine structural information with metadata.
//!
//! Key design principles:
//! - Annotations are stored in separate maps to avoid IR structure changes
//! - All public references carry both IR data and annotation context
//! - Deref implementations allow transparent access to underlying IR functionality

use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::Deref,
};

use hc_utils::{iter::MultiZip, small::SmallVec};

use crate::{
    AnnValOriginRef, AnnValUseRef, Dialect, IR, OpId, OpMap, OpRef, PrintWalker, Printer, ValId,
    ValMap, val_ref::ValRef,
};

/// Operation reference with attached annotation data.
#[derive(Debug, Clone)]
pub struct AnnOpRef<'ir, 'ann, D: Dialect, OpAnn, ValAnn>
where
    'ir: 'ann,
{
    ann_ir: &'ann AnnIR<'ir, D, OpAnn, ValAnn>,
    opref: OpRef<'ir, D>,
    ann: &'ann OpAnn,
}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn> AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn> {
    /// Returns the annotation for this operation.
    pub fn get_annotation(&self) -> &OpAnn {
        self.ann
    }

    /// Returns an iterator over the operation's argument values with annotations.
    pub fn get_args_iter(
        &self,
    ) -> impl Iterator<Item = AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>> + use<'ir, 'ann, D, OpAnn, ValAnn>
    {
        self.opref.get_args_iter().map(|valref| {
            let ann = &self.ann_ir.val_annotations[*valref];
            AnnValRef {
                ann_ir: self.ann_ir,
                valref,
                ann,
            }
        })
    }

    /// Returns an iterator over the operation's return values with annotations.
    pub fn get_returns_iter(
        &self,
    ) -> impl Iterator<Item = AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>> + use<'ir, 'ann, D, OpAnn, ValAnn>
    {
        self.opref.get_returns_iter().map(|valref| {
            let ann = &self.ann_ir.val_annotations[*valref];
            AnnValRef {
                ann_ir: self.ann_ir,
                valref,
                ann,
            }
        })
    }

    /// Returns an iterator over the direct users of this operation with annotations.
    pub fn get_users_iter(
        &self,
    ) -> impl Iterator<Item = AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>> + use<'ir, 'ann, D, OpAnn, ValAnn>
    {
        self.opref.get_users_iter().map(|opref| {
            let ann = &self.ann_ir.op_annotations[*opref];
            AnnOpRef {
                ann_ir: self.ann_ir,
                opref,
                ann,
            }
        })
    }

    /// Returns an iterator over the direct predecessors of this operation with annotations.
    pub fn get_predecessors_iter(
        &self,
    ) -> impl Iterator<Item = AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>> + use<'ir, 'ann, D, OpAnn, ValAnn>
    {
        self.opref.get_predecessors_iter().map(|opref| {
            let ann = &self.ann_ir.op_annotations[*opref];
            AnnOpRef {
                ann_ir: self.ann_ir,
                opref,
                ann,
            }
        })
    }

    /// Returns an iterator over all operations that can reach this operation with annotations.
    pub fn get_reaching_iter(
        &self,
    ) -> impl Iterator<Item = AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>> + use<'ir, 'ann, D, OpAnn, ValAnn>
    {
        self.opref.get_reaching_iter().map(|opref| {
            let ann = &self.ann_ir.op_annotations[*opref];
            AnnOpRef {
                ann_ir: self.ann_ir,
                opref,
                ann,
            }
        })
    }

    /// Returns an iterator over all operations that can be reached from this operation with annotations.
    pub fn get_reached_iter(
        &self,
    ) -> impl Iterator<Item = AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>> + use<'ir, 'ann, D, OpAnn, ValAnn>
    {
        self.opref.get_reached_iter().map(|opref| {
            let ann = &self.ann_ir.op_annotations[*opref];
            AnnOpRef {
                ann_ir: self.ann_ir,
                opref,
                ann,
            }
        })
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn> Deref for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn> {
    type Target = OpRef<'ir, D>;

    fn deref(&self) -> &Self::Target {
        &self.opref
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn> Display for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>
where
    OpAnn: Debug + Clone,
    ValAnn: Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let printer = Printer::from_ann_ir(
                self.ann_ir,
                crate::PrintWalker::Linear,
                true,
                true,
                true,
                true,
            );
            printer.format_ann_opref(f, self)
        } else {
            let printer = Printer::from_ir(self.ir, crate::PrintWalker::Topo, true, true);
            printer.format_ann_opref(f, self)
        }
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn: PartialEq, ValAnn> PartialEq
    for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    fn eq(&self, other: &Self) -> bool {
        self.opref == other.opref && *self.ann == *other.ann
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn: Eq, ValAnn> Eq for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn> {}

impl<'ir, 'ann, D: Dialect, OpAnn: Hash, ValAnn> Hash for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.opref.hash(state);
        self.ann.hash(state);
    }
}

/// Value reference with attached annotation data.
#[derive(Debug, Clone)]
pub struct AnnValRef<'ir, 'ann, D: Dialect, OpAnn, ValAnn>
where
    'ir: 'ann,
{
    ann_ir: &'ann AnnIR<'ir, D, OpAnn, ValAnn>,
    valref: ValRef<'ir, D>,
    ann: &'ann ValAnn,
}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn> AnnValRef<'ir, 'ann, D, OpAnn, ValAnn> {
    /// Returns the annotation for this value.
    pub fn get_annotation(&self) -> &ValAnn {
        self.ann
    }

    /// Returns the operation that produces this value with its annotation.
    pub fn get_origin(&self) -> AnnValOriginRef<'ir, 'ann, D, OpAnn, ValAnn> {
        let origin = self.valref.get_origin();
        let ann = &self.ann_ir.op_annotations[**origin];
        AnnValOriginRef {
            opref: AnnOpRef {
                ann_ir: self.ann_ir,
                opref: (*origin).clone(),
                ann,
            },
            position: origin.position,
        }
    }

    pub fn get_uses_iter(
        &self,
    ) -> impl Iterator<Item = AnnValUseRef<'ir, 'ann, D, OpAnn, ValAnn>> + use<'ir, 'ann, D, OpAnn, ValAnn>
    {
        self.valref.get_uses_iter().map(|user| {
            let ann = &self.ann_ir.op_annotations[**user];
            AnnValUseRef {
                opref: AnnOpRef {
                    ann_ir: self.ann_ir,
                    opref: user.opref,
                    ann,
                },
                position: user.position,
            }
        })
    }

    /// Returns an iterator over operations that use this value with their annotations.
    pub fn get_users_iter(
        &self,
    ) -> impl Iterator<Item = AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>> + use<'ir, 'ann, D, OpAnn, ValAnn>
    {
        self.valref.get_users_iter().map(|user| {
            let ann = &self.ann_ir.op_annotations[*user];
            AnnOpRef {
                ann_ir: self.ann_ir,
                opref: user,
                ann,
            }
        })
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn> Deref for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn> {
    type Target = ValRef<'ir, D>;

    fn deref(&self) -> &Self::Target {
        &self.valref
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn> Display for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>
where
    OpAnn: Debug + Clone,
    ValAnn: Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let printer = Printer::from_ir(self.ir, crate::PrintWalker::Linear, true, true);
            printer.format_ann_valref(f, self)
        } else {
            let printer = Printer::from_ir(self.ir, crate::PrintWalker::Topo, true, true);
            printer.format_ann_valref(f, self)
        }
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn: PartialEq> PartialEq
    for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    fn eq(&self, other: &Self) -> bool {
        self.valref == other.valref && *self.ann == *other.ann
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn: Eq> Eq for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn> {}

impl<'ir, 'ann, D: Dialect, OpAnn, ValAnn: Hash> Hash for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.valref.hash(state);
        self.ann.hash(state);
    }
}

/// IR container with parallel annotation storage for operations and values.
#[derive(Debug, Clone)]
pub struct AnnIR<'ir, D: Dialect, OpAnn, ValAnn> {
    ir: &'ir IR<D>,
    op_annotations: OpMap<OpAnn>,
    val_annotations: ValMap<ValAnn>,
}

impl<'ir, D: Dialect, OpAnn, ValAnn> AnnIR<'ir, D, OpAnn, ValAnn> {
    /// Creates a new annotated IR with the given operation and value annotations.
    ///
    /// # Panics
    ///
    /// Panics if the annotation maps are not completely filled for all active operations and values.
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

    pub fn check_ir(&self, expected: &str)
    where
        OpAnn: Debug,
        ValAnn: Debug,
    {
        self.check_ir_gen(PrintWalker::Topo, expected);
    }

    pub fn check_ir_linear(&self, expected: &str)
    where
        OpAnn: Debug,
        ValAnn: Debug,
    {
        self.check_ir_gen(PrintWalker::Linear, expected);
    }

    fn check_ir_gen(&self, walker: PrintWalker, expected: &str)
    where
        OpAnn: Debug,
        ValAnn: Debug,
    {
        let clean = |inp: &str| inp.replace(' ', "").replace('\n', "");
        let repr =
            Printer::from_ann_ir(self, walker, true, false, true, false).ann_ir_to_string(self);
        if clean(&repr) != clean(expected) {
            println!(
                "Failed to check ir.\nExpected:\n{}\nActual:\n{}",
                expected, repr
            );
            panic!("Failed to check ir");
        }
    }

    /// Performs backward dataflow analysis on the IR operations.
    pub fn backward_dataflow_analysis<OpAnnNew, ValAnnNew>(
        &self,
        mut f: impl FnMut(
            &OpMap<OpAnnNew>,
            &ValMap<ValAnnNew>,
            &AnnOpRef<D, OpAnn, ValAnn>,
        ) -> (OpAnnNew, SmallVec<ValAnnNew>),
    ) -> AnnIR<'ir, D, OpAnnNew, ValAnnNew> {
        let mut opmap = self.empty_opmap();
        let mut valmap = self.empty_valmap();
        for opref in self.walk_ops_topological().rev() {
            assert!(opref.get_users_iter().all(|k| opmap.contains_key(&k)));
            let (opann, valanns) = f(&opmap, &valmap, &opref);
            assert_eq!(valanns.len(), opref.get_return_valids().len());
            assert!(opmap.insert(**opref, opann).is_none());
            for (valann, valref) in (valanns.into_iter(), opref.get_return_valids().iter()).mzip() {
                assert!(valmap.insert(*valref, valann).is_none());
            }
        }
        AnnIR::new(self.ir, opmap, valmap)
    }

    /// Performs forward dataflow analysis on the IR operations.
    pub fn forward_dataflow_analysis<OpAnnNew, ValAnnNew>(
        &self,
        mut f: impl FnMut(
            &OpMap<OpAnnNew>,
            &ValMap<ValAnnNew>,
            &AnnOpRef<D, OpAnn, ValAnn>,
        ) -> (OpAnnNew, SmallVec<ValAnnNew>),
    ) -> AnnIR<'ir, D, OpAnnNew, ValAnnNew> {
        let mut opmap = self.empty_opmap();
        let mut valmap = self.empty_valmap();
        for opref in self.walk_ops_topological() {
            assert!(
                opref
                    .get_predecessors_iter()
                    .all(|k| opmap.contains_key(&k))
            );
            let (opann, valanns) = f(&opmap, &valmap, &opref);
            assert_eq!(valanns.len(), opref.get_return_valids().len());
            assert!(opmap.insert(**opref, opann).is_none());
            for (valann, valref) in (valanns.into_iter(), opref.get_return_valids().iter()).mzip() {
                assert!(valmap.insert(*valref, valann).is_none());
            }
        }
        AnnIR::new(self.ir, opmap, valmap)
    }
}

impl<'ir, D: Dialect, OpAnn, ValAnn> Deref for AnnIR<'ir, D, OpAnn, ValAnn> {
    type Target = IR<D>;

    fn deref(&self) -> &Self::Target {
        self.ir
    }
}
