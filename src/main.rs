use tree_sitter::{InputEdit, Language, Parser, Point};

fn main() {
    println!("Hello, world!");

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_dapper::LANGUAGE.into())
        .expect("Error loading grammar");

    let source_code = "type Whee string";
    let mut tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    println!("wowsa: {}", root_node)
}
