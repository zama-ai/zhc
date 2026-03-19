use super::*;
use crate::{AnnOpRef, AnnValRef, Dialect, IR, OpId, OpMap, ValId, ValMap};

#[derive(Debug, Clone)]
pub struct AnnIRView<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> {
    pub(crate) ir: &'ir IR<D>,
    pub(crate) op_annotations: &'ann OpMap<OpAnn>,
    pub(crate) val_annotations: &'ann ValMap<ValAnn>,
}

impl<'ir, 'ann, D: Dialect, OpAnn: Annotation, ValAnn: Annotation>
    AnnIRView<'ir, 'ann, D, OpAnn, ValAnn>
{
    pub fn new(
        ir: &'ir IR<D>,
        op_annotations: &'ann OpMap<OpAnn>,
        val_annotations: &'ann ValMap<ValAnn>,
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

    /// Returns an annotated operation reference for the specified operation.
    ///
    /// # Panics
    ///
    /// Panics if the operation ID does not exist or refers to an inactive operation.
    pub fn get_op(&self, opid: OpId) -> AnnOpRef<'ir, '_, D, OpAnn, ValAnn> {
        let opref = self.ir.get_op(opid);
        let ann = &self.op_annotations[opid];
        AnnOpRef {
            ir: self.clone(),
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
            ir: self.clone(),
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
                ir: self.clone(),
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
                ir: self.clone(),
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
                ir: self.clone(),
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
                ir: self.clone(),
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
                ir: self.clone(),
                valref,
                ann,
            }
        })
    }

    // /// Creates a configurable formatter for the annotated IR.
    // pub fn format(&self) -> Formatted<'_, Self> {
    //     Formatted::new(self)
    // }
}

// impl<'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Deref
//     for AnnIR<'ir, D, OpAnn, ValAnn>
// {
//     type Target = IR<D>;

//     fn deref(&self) -> &Self::Target {
//         self.ir
//     }
// }

// impl<'ir, D: Dialect, OpAnn: Annotation, ValAnn: Annotation> Dumpable
//     for AnnIR<'ir, D, OpAnn, ValAnn>
// {
//     fn dump_to_string(&self) -> String {
//         format!(
//             "{}",
//             self.format()
//                 .with_walker(crate::PrintWalker::Linear)
//                 .show_types(false)
//                 .show_opid(true)
//                 .show_comments(true)
//         )
//     }
// }
