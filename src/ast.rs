#[cfg(ide)]
pub use crate::grammar::{RonParser, Rule};

use pest::{Parser, iterators::{Pair, Pairs}, Span};

use crate::error::Result;

pub struct Ast;

impl Ast {
    pub fn visit_file<'a, V: Visitor<'a>>(src: &'a str, visitor: &mut V) -> Result<()> {
        let mut ron_file: Pairs<Rule> = <RonParser as Parser<Rule>>::parse(Rule::ron_file, src)?;
        let ron_file: Pairs<Rule> = ron_file.next().unwrap().into_inner();

        for pair in ron_file {
            match pair.as_rule() {
                Rule::COMMENT => Self::handle_comment(pair, visitor),
                Rule::extension => visitor.visit_extension(pair.into()),
                Rule::value => visitor.visit_value(pair.into()),
                Rule::EOI => {}
                _ => unreachable!("`ron_file` rule does not emit anything else (found: {:?})", pair),
            }
        }

        Ok(())
    }

    pub fn visit_extension<'a, V: Visitor<'a>>(extension: Extension<'a>, visitor: &mut V) {
        for pair in extension.pair.into_inner() {
            match pair.as_rule() {
                Rule::COMMENT => Self::handle_comment(pair, visitor),
                Rule::ident => visitor.visit_ident(pair.into()),
                _ => unreachable!("`extension` rule does not emit anything else"),
            }
        }
    }

    fn handle_comment<'a, V: Visitor<'a>>(pair: Pair<'a, Rule>, visitor: &mut V) {
        visitor.visit_comment(pair.into());
    }
}

pub trait Visitor<'a> {
    /// Visits one extension statement, which may contain multiple idents.
    fn visit_extension(&mut self, extension: Extension<'a>) { panic!("Missing implementation") }
    fn visit_value(&mut self, value: Value<'a>) { panic!("Missing implementation") }
    fn visit_comment(&mut self, comment: Comment<'a>) { panic!("Missing implementation") }

    fn visit_ident(&mut self, ident: Ident<'a>) { panic!("Missing implementation") }
}

#[derive(Debug)]
pub struct AstGraph<'a> {
    comments: Vec<Comment<'a>>,
    extensions: Vec<Ident<'a>>,
    graph: Vec<Value<'a>>,
    root: NodeId,
}

impl<'a> AstGraph<'a> {
    pub fn from_str(src: &'a str) -> Result<AstGraph<'a>> {
        let mut ast = AstGraph {
            comments: Vec::new(),
            extensions: Vec::new(),
            graph: Vec::new(),
            root: NodeId(0),
        };

        Ast::visit_file(src, &mut ast)?;

        Ok(ast)
    }
}

impl<'a> Visitor<'a> for AstGraph<'a> {
    fn visit_extension(&mut self, extension: Extension<'a>) {
        struct ExtensionVisitor<'a, 'b> {
            ast_graph: &'b mut AstGraph<'a>,
        }

        impl<'a, 'b> Visitor<'a> for ExtensionVisitor<'a, 'b> {
            fn visit_comment(&mut self, comment: Comment<'a>) {
                self.ast_graph.visit_comment(comment);
            }

            fn visit_ident(&mut self, ident: Ident<'a>) {
                self.ast_graph.extensions.push(ident);
            }
        }

        Ast::visit_extension(extension, &mut ExtensionVisitor {
            ast_graph: self,
        });
    }

    fn visit_value(&mut self, value: Value<'a>) {
        println!("value: {:#?}", value);
    }

    fn visit_comment(&mut self, comment: Comment<'a>) {
        self.comments.push(comment);
    }
}

/// A node pointing to a `Value`.
#[derive(Debug)]
pub struct NodeId(usize);

/// Creates an AST element that simply wraps a pair.
macro_rules! ast_element {
    ($name:ident, $content:ident) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            pub pair: Pair<'a, Rule>,
        }

        impl<'a> $name<'a> {
            pub fn span(&self) -> Span<'a> {
                self.pair.as_span()
            }

            pub fn $content(&self) -> &str {
                self.span().as_str()
            }
        }

        impl<'a> From<Pair<'a, Rule>> for $name<'a> {
            fn from(pair: Pair<'a, Rule>) -> $name<'a> {
                $name {
                    pair,
                }
            }
        }
    };
}

ast_element!(Comment, content);
ast_element!(Extension, as_str);
ast_element!(Ident, as_str);
ast_element!(Value, source_str);

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct RonParser;
