use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{AnnIR, AnnValRef, Annotation, Dialect, OpRef, Printer};

/// Operation reference with attached annotation data.
#[derive(Debug, Clone)]
pub struct AnnOpRef<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    pub(super) ann_ir: &'ann AnnIR<'ir, D, OpAnn, ValAnn>,
    pub(super) opref: OpRef<'ir, D>,
    pub(super) ann: &'ann OpAnn,
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation>
    AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>
{
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

    /// Returns an iterator over all operations that can be reached from this operation with
    /// annotations.
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

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Deref
    for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    type Target = OpRef<'ir, D>;

    fn deref(&self) -> &Self::Target {
        &self.opref
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Display
    for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>
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

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> PartialEq
    for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    fn eq(&self, other: &Self) -> bool {
        self.opref == other.opref && *self.ann == *other.ann
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Eq
    for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>
{
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Hash
    for AnnOpRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.opref.hash(state);
        self.ann.hash(state);
    }
}
