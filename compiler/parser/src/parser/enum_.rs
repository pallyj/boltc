use crate::{lexer::SyntaxKind, parser::file::ITEM_RECOVERY_SET};

use super::{Parser, marker::Marker};

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_enum(&mut self, marker: Marker) {
        debug_assert!(self.check(SyntaxKind::EnumKw));
        self.eat(SyntaxKind::EnumKw);

        self.name(ITEM_RECOVERY_SET);

        self.parse_enum_repr();

        self.parse_delim(SyntaxKind::EnumBody,
                         SyntaxKind::OpenBrace,
                         SyntaxKind::CloseBrace,
                         Self::parse_enum_item);

        marker.complete(self, SyntaxKind::EnumDef);
    }

	pub fn parse_enum_item(&mut self) {
        if self.eat(SyntaxKind::Semicolon) {
            return;
        }

        let marker = self.start();
        self.parse_comments();
        self.parse_attributes();
        self.parse_visibility();
        self.eat(SyntaxKind::StaticKw); // todo: no static case or init

        match self.peek() {
            Some(SyntaxKind::FuncKw) |
            Some(SyntaxKind::MutatingKw) => self.parse_func(marker, true),
            Some(SyntaxKind::OperatorKw) => self.parse_operator_func(marker),
            // Some(SyntaxKind::VarKw) => self.parse_var(marker),
            // Some(SyntaxKind::LetKw) => self.parse_let(marker),
            Some(SyntaxKind::InitKw) => self.parse_init(marker),
            Some(SyntaxKind::StructKw) => self.parse_struct(marker),
			Some(SyntaxKind::CaseKw) => self.parse_case(marker),
            Some(SyntaxKind::EnumKw) => self.parse_enum(marker),
            _ => {
                // Error
                self.error_recover("expected enum item", ITEM_RECOVERY_SET);
                marker.complete(self, SyntaxKind::Error);
            }
        }
    }

	pub fn parse_case(&mut self, marker: Marker) {
		debug_assert!(self.check(SyntaxKind::CaseKw));
        self.eat(SyntaxKind::CaseKw);

		// Loop through the case items
		while self.check(SyntaxKind::Ident) {
			self.parse_case_item();

			if !self.eat(SyntaxKind::Comma) {
				break;
			}
		}

		marker.complete(self, SyntaxKind::CaseDef);
	}

	pub fn parse_case_item(&mut self) {
		debug_assert!(self.check(SyntaxKind::Ident));

		let marker = self.start();
        if !self.eat(SyntaxKind::Ident) {
            self.error("Expected ident")
        }

        if self.check(SyntaxKind::OpenParen) {
            self.parse_paren_comma_seq(Self::parse_ty_tuple);
        }

        let marker2 = self.start();
        if self.eat(SyntaxKind::Equals) {
            self.parse_expr();
        }
        marker2.complete(self, SyntaxKind::AssignValue);

		marker.complete(self, SyntaxKind::CaseItem);
	}

    pub fn parse_enum_repr(&mut self) {
        self.parse_ty_return();
    }
}