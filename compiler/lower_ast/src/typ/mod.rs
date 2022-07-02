mod struct_;
mod enum_;

use blir::typ::{Type, TypeKind};
use parser::ast::typ::Type as AstType;

use crate::AstLowerer;

impl<'a, 'b> AstLowerer<'a, 'b> {
    pub(crate) fn lower_type(&mut self, typ: AstType) -> Type {
        let range = typ.range();
        let span = self.span(range);

        match typ {
            AstType::UnitType(..) => TypeKind::Void,

            AstType::NamedType(named_type) => TypeKind::Named(named_type.name()),
            AstType::MemberType(member_type) => TypeKind::Member { parent: Box::new(self.lower_type(member_type.parent())),
                                                                   member: member_type.child().unwrap(), },

            AstType::FuncType(func_type) => {
                let return_type = func_type.return_type()
                                           .map(|ty| self.lower_type(ty))
                                           .unwrap_or_else(|| TypeKind::Void.anon());

                let params = func_type.params()
                                      .into_iter()
                                      .map(|ty| self.lower_type(ty))
                                      .collect();

                TypeKind::Function { return_type: Box::new(return_type),
                                     params,
                                     labels: vec![] }
            }

            AstType::ParenthesizedType(paren_type) => {
                let typ = self.lower_type(paren_type.typ());

                if let Some(label) = paren_type.tuple_label() {
                    TypeKind::Tuple(vec![typ], vec![Some(label)])
                } else {
                    return typ
                }
            },

            AstType::TupleType(tuple_type) => {
                let (tuple_types, labels) =
                    tuple_type.types()
                              .map(|ty| (self.lower_type(ty.typ()), ty.label()))
                              .unzip();

                TypeKind::Tuple(tuple_types, labels)
            }

            AstType::InferType(_) => Type::infer().kind,
            AstType::GenericType(generic) => {
                let hk = self.lower_type(generic.polymorphic_type());
                let parameters = generic.type_arguments().into_iter().map(|ty| self.lower_type(ty)).collect::<Vec<_>>();

                TypeKind::GenericOf { higher_kind: Box::new(hk), params: parameters }
            }
            AstType::ArrayType(array_type) => {
                let item = self.lower_type(array_type.item_type());
                let lowered_expr = self.lower_expr(array_type.length(), None);
                let (is_negative, len) = self.lower_integer(&lowered_expr);

                if is_negative {
                    // throw an error
                    println!("error: negative array length")
                }

                TypeKind::Array { item: Box::new(item), len: len as usize }
            }

            AstType::Error => panic!(),
        }.spanned(span)
    }
}
