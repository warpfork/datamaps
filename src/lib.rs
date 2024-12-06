use std::io::{self, Read};
use tree_sitter::{InputEdit, Language, Parser, Point, TreeCursor};

mod defns;

pub fn main() {
    println!("Hello, world!");

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_dapper::LANGUAGE.into())
        .expect("Error loading grammar");

    let source_code = "type Whee string\ntype Frob map{Foo:Foo}";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    println!("wowsa: {}", root_node);

    myprint(&mut io::stdout(), &source_code, &mut root_node.walk(), 0);
}

pub fn dapper_hey() {}

pub fn treedemo() -> String {
    Result::<(), ()>::Ok(()).expect("what");

    let mut parser = Parser::new();
    // parser
    //     .set_language(&tree_sitter_dapper::LANGUAGE.into())
    //     .expect("Error loading grammar");
    // well, it's not the expect.
    // is it something about tree-sitter-dapper's config?
    // nope, it's as soon as we refer to Parser at all.
    // aka, the first time an ffi to C Actually Happens.  great.
    // it doesn't look like the c on the bottom is doing anything with the environment.  it's just mem init stuff.
    // so okay, something's turbo fucked here.

    "Wot".to_owned()

    // let source_code = "type Whee string\ntype Frob map{Foo:Foo}";
    // let tree = parser.parse(source_code, None).unwrap();
    // let root_node = tree.root_node();

    // let mut buf = Vec::new();
    // myprint(&mut buf, &source_code, &mut root_node.walk(), 0);
    // String::from_utf8(buf.into()).expect("shhh")
}

pub fn myprint(
    out: &mut impl io::Write,
    doc: &impl AsRef<[u8]>,
    tc: &mut TreeCursor,
    depth: usize,
) {
    let n = tc.node();

    let is_field = tc.field_name().is_some();
    if n.is_named() || is_field {
        write!(out, "{}{}", "\t".repeat(depth), n.kind()).expect("output");
        if is_field {
            write!(out, " -- field: {}", tc.field_name().unwrap()).expect("output");
        }
        // TODO: this val body dump should probably print an elided-middle fixed size if over some threshhold.
        writeln!(out, " -- val: {}", n.utf8_text(doc.as_ref()).expect("utf8")).expect("output");
    } else {
        // You can print unnamed nodes too, sure -- but you'll get every little literal token.  Probably unwanted.
        // writeln!(out, "{}{}", indent, n.kind());
    }

    // Walk children.
    if !tc.goto_first_child() {
        return;
    }
    myprint(out, doc, tc, depth + 1);
    while tc.goto_next_sibling() {
        myprint(out, doc, tc, depth + 1)
    }
    tc.goto_parent();
}
