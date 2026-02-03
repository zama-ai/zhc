use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{
    AnnIR, AnnOpRef, AnnValOriginRef, AnnValRefFormatter, AnnValUseRef, Annotation, Dialect, ValRef,
};

/// Value reference with attached annotation data.
#[derive(Debug, Clone)]
pub struct AnnValRef<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    pub(super) ann_ir: &'ann AnnIR<'ir, D, OpAnn, ValAnn>,
    pub(super) valref: ValRef<'ir, D>,
    pub(super) ann: &'ann ValAnn,
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation>
    AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    /// Returns the annotation for this value.
    pub fn get_annotation(&self) -> &ValAnn {
        self.ann
    }

    /// Returns the operation that produces this value with its annotation.
    pub fn get_origin(&self) -> AnnValOriginRef<'ir, 'ann, D, OpAnn, ValAnn> {
        let origin = self.valref.get_origin();
        let ann = &self.ann_ir.op_annotations[*origin.opref];
        AnnValOriginRef {
            opref: AnnOpRef {
                ann_ir: self.ann_ir,
                opref: origin.opref.clone(),
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
            let ann = &self.ann_ir.op_annotations[*user.opref];
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

    pub fn format(&self) -> AnnValRefFormatter<'_, 'ir, 'ann, D, OpAnn, ValAnn> {
        AnnValRefFormatter::new(self)
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Deref
    for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    type Target = ValRef<'ir, D>;

    fn deref(&self) -> &Self::Target {
        &self.valref
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> PartialEq
    for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    fn eq(&self, other: &Self) -> bool {
        self.valref == other.valref && *self.ann == *other.ann
    }
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Eq
    for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>
{
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Hash
    for AnnValRef<'ir, 'ann, D, OpAnn, ValAnn>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.valref.hash(state);
        self.ann.hash(state);
    }
}
