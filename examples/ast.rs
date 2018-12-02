use ron_pest::Values;

fn main() {
    Values::from_str(r#"Struct(
    bar: "Bar", // Comment
)"#).unwrap();
}
