use pest::Span;

use crate::{Result, ast};

#[derive(Debug)]
pub enum ValueRef<'a> {
    Bool(bool),
    Float(f64),
    String(&'a str),
}

#[derive(Debug)]
pub struct Values<'a> {
    comments: Vec<ast::Comment<'a>>,
    extensions: Vec<ast::Ident<'a>>,
    graph: Vec<ValueRef<'a>>,
    spans: Vec<Span<'a>>,
}

impl<'a> Values<'a> {
    pub fn from_str(src: &'a str) -> Result<Values<'a>> {
        let mut ast = Values {
            comments: Vec::new(),
            extensions: Vec::new(),
            graph: Vec::new(),
            spans: Vec::new(),
        };

        ast::Ast::visit_file(src, &mut ast)?;

        Ok(ast)
    }
}

impl<'a> ast::Visitor<'a> for Values<'a> {
    fn visit_extension_ident(&mut self, ident: ast::Ident<'a>) {
        self.extensions.push(ident);
    }

    fn visit_comment(&mut self, comment: ast::Comment<'a>) {
        self.comments.push(comment);
    }

    fn visit_value(&mut self, value: ast::Value<'a>) {
        self.spans.push(value.as_span());
        ast::Ast::visit_value(value, self);
    }

    fn visit_bool(&mut self, value: ast::Bool<'a>) {
        self.graph.push(ValueRef::Bool(value.as_bool()));
    }

    fn visit_float(&mut self, value: ast::Float<'a>) {
        self.graph.push(ValueRef::Float(value.as_float()));
    }
}

/// A node pointing to a `Value`.
#[derive(Debug)]
pub struct NodeId(usize);
