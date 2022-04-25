use std::iter;

use blir::typ::{Type, TypeKind};
use blirssa::typ::Type as SsaType;

use crate::BlirLowerer;

impl BlirLowerer {
    pub(super) fn lower_type(&self, typ: &Type) -> SsaType {
        match &typ.kind() {
            TypeKind::Void => SsaType::Void,
            TypeKind::Divergent => SsaType::Void,
            TypeKind::StrSlice => SsaType::StrSlice,

            TypeKind::Integer { bits } => SsaType::Integer { bits: *bits as u32 },
            TypeKind::Float { bits } => SsaType::Float { bits: *bits as u32 },

            TypeKind::Struct(sref) => SsaType::Struct { container: self.ssa_library()
                                                                        .get_struct(&sref.link_name())
                                                                        .cloned()
                                                                        .unwrap(), },
            TypeKind::Function { return_type,
                                 params,
                                 labels: _, } => SsaType::Function { return_type: Box::new(self.lower_type(return_type)),
                                                                      pars:        params.iter().map(|par| self.lower_type(par)).collect(), },
            TypeKind::Method { reciever,
                               return_type,
                               params, } => SsaType::Function { return_type: Box::new(self.lower_type(return_type)),
                                                                 pars:        iter::once(reciever.as_ref()).chain(params.iter())
                                                                                                           .map(|par| self.lower_type(par))
                                                                                                           .collect(), },

            TypeKind::Tuple(tuple_items) => SsaType::Tuple(tuple_items.iter().map(|item| self.lower_type(item)).collect()),

            _ => panic!("{typ:?}"),
        }
    }
}
