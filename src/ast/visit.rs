use pest::{
    iterators::{Pair, Pairs},
    Parser, Span,
};

use crate::{
    ast::{RonParser, Rule},
    error::Result,
};

pub struct Ast;

impl Ast {
    pub fn visit_file<'a, V: Visitor<'a> + ?Sized>(src: &'a str, visitor: &mut V) -> Result<()> {
        let mut ron_file: Pairs<Rule> = <RonParser as Parser<Rule>>::parse(Rule::ron_file, src)?;
        let ron_file: Pairs<Rule> = ron_file.next().unwrap().into_inner();

        for pair in ron_file {
            match pair.as_rule() {
                Rule::COMMENT => Self::handle_comment(pair, visitor),
                Rule::extension => visitor.visit_extension(pair.into()),
                Rule::value => visitor.visit_value(pair.into()),
                Rule::EOI => {}
                _ => unreachable!(
                    "`ron_file` rule does not emit anything else (found: {:?})",
                    pair
                ),
            }
        }

        Ok(())
    }

    pub fn visit_extension<'a, V: Visitor<'a> + ?Sized>(extension: Extension<'a>, visitor: &mut V) {
        for pair in extension.pair.into_inner() {
            match pair.as_rule() {
                Rule::COMMENT => Self::handle_comment(pair, visitor),
                Rule::ident => visitor.visit_extension_ident(pair.into()),
                _ => unreachable!(
                    "`extension` rule does not emit anything else (found: {:?})",
                    pair
                ),
            }
        }
    }

    pub fn visit_value<'a, V: Visitor<'a> + ?Sized>(value: Value<'a>, visitor: &mut V) {
        for pair in value.pair.into_inner() {
            match pair.as_rule() {
                Rule::COMMENT => Self::handle_comment(pair, visitor),
                Rule::bool => visitor.visit_bool(pair.into()),
                Rule::float => visitor.visit_float(pair.into()),
                _ => unreachable!(
                    "`value` rule does not emit anything else (found: {:?})",
                    pair
                ),
            }
        }
    }

    fn handle_comment<'a, V: Visitor<'a> + ?Sized>(pair: Pair<'a, Rule>, visitor: &mut V) {
        visitor.visit_comment(pair.into());
    }
}

pub trait Visitor<'a> {
    /// Visits one extension statement, which may contain multiple idents.
    /// Defaults to delegating the call to `Ast::visit_extension`.
    fn visit_extension(&mut self, extension: Extension<'a>) {
        Ast::visit_extension(extension, self);
    }

    fn visit_extension_ident(&mut self, ident: Ident<'a>);

    fn visit_comment(&mut self, comment: Comment<'a>);

    fn visit_value(&mut self, value: Value<'a>) {
        Ast::visit_value(value, self);
    }

    fn visit_bool(&mut self, value: Bool<'a>);
    fn visit_float(&mut self, value: Float<'a>);
}

/// Creates an AST element that simply wraps a span.
macro_rules! ast_element {
    ($name:ident, $content:ident) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            pub span: Span<'a>,
        }

        impl<'a> $name<'a> {
            pub fn as_span(&self) -> Span<'a> {
                self.span.clone()
            }

            pub fn $content(&self) -> &str {
                self.as_span().as_str()
            }
        }

        impl<'a> From<Pair<'a, Rule>> for $name<'a> {
            fn from(pair: Pair<'a, Rule>) -> $name<'a> {
                $name {
                    span: pair.as_span(),
                }
            }
        }
    };
}

/// Creates an AST element that simply wraps a pair.
macro_rules! ast_element_pair {
    ($name:ident, $content:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name<'a> {
            pub pair: Pair<'a, Rule>,
        }

        impl<'a> $name<'a> {
            pub fn as_span(&self) -> Span<'a> {
                self.pair.as_span()
            }

            pub fn $content(&self) -> &str {
                self.as_span().as_str()
            }
        }

        impl<'a> From<Pair<'a, Rule>> for $name<'a> {
            fn from(pair: Pair<'a, Rule>) -> $name<'a> {
                $name { pair }
            }
        }
    };
}

ast_element!(Bool, as_str);

impl<'a> Bool<'a> {
    pub fn as_bool(&self) -> bool {
        self.as_str().as_bytes()[0] == b't'
    }
}

ast_element_pair!(Comment, content);
ast_element_pair!(Extension, as_str);

ast_element_pair!(Float, as_str);

impl<'a> Float<'a> {
    pub fn as_float(&self) -> f64 {
        self.as_str()
            .parse()
            .expect("Unreachable: float parsed already")
    }
}

ast_element!(Ident, as_str);
ast_element_pair!(Value, source_str);
