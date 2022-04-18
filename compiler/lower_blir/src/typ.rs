use std::iter;

use blir::typ::{Type, TypeKind};
use blirssa::typ::Type as BlirType;

use crate::BlirLowerer;

impl BlirLowerer {
    pub(super) fn lower_type(&self, typ: &Type) -> BlirType {
        match &typ.kind() {
            TypeKind::Void => BlirType::Void,
            TypeKind::Divergent => BlirType::Void,

            TypeKind::Integer { bits } => BlirType::Integer { bits: *bits as u32 },
            TypeKind::Float { bits } => BlirType::Float { bits: *bits as u32 },

            TypeKind::Struct(sref) => BlirType::Struct { container: self.ssa_library()
                                                                        .get_struct(&sref.link_name())
                                                                        .cloned()
                                                                        .unwrap(), },
            TypeKind::Function { return_type,
                                 params,
                                 labels: _, } => BlirType::Function { return_type: Box::new(self.lower_type(return_type)),
                                                                      pars:        params.iter().map(|par| self.lower_type(par)).collect(), },
            TypeKind::Method { reciever,
                               return_type,
                               params, } => BlirType::Function { return_type: Box::new(self.lower_type(return_type)),
                                                                 pars:        iter::once(reciever.as_ref()).chain(params.iter())
                                                                                                           .map(|par| self.lower_type(par))
                                                                                                           .collect(), },

            _ => panic!("{typ:?}"),
        }
    }
}
