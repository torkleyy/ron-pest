use ron_pest::ast::AstGraph;

fn main() {
    AstGraph::from_str(r#"Struct(
    bar: "Bar", // Comment
)"#);
}
