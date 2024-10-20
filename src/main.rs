use tree_sitter::{InputEdit, Language, Parser, Point, TreeCursor};

fn main() {
    println!("Hello, world!");

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_dapper::LANGUAGE.into())
        .expect("Error loading grammar");

    let source_code = "type Whee string";
    let mut tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    println!("wowsa: {}", root_node);

    myprint(&source_code, &mut root_node.walk());
}

fn myprint(doc: &impl AsRef<[u8]>, tc: &mut TreeCursor) {
    let n = tc.node();
    println!("{}", n.kind());

    // TODO: some kind of condition like finding labelled nodes would be good here.  or leaf nodes?
    // or just print an elided-middle fixed size and do so unconditionally.  unconditional can be good.
    if n.is_named() {
        println!("  val: {:?}", n.utf8_text(doc.as_ref()).expect("utf8"))
    }

    // TODO: print more things, like any field names.
    // It seems really odd to me that you can't get a field name except by `n.field_name_for_child(child_index)` on the *parent*.
    // Ah.  Do it on the cursor.  Ok lol.

    // Walk children.
    if !tc.goto_first_child() {
        return;
    }
    myprint(doc, tc);
    while tc.goto_next_sibling() {
        myprint(doc, tc)
    }
}
