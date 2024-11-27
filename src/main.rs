use leek_ast::parser::Parser;
use leek_ast::serializer::Writer;
use leek_ast::visitor::{Visitor, VisitorMut};
use leek_ast::{Identifier, WithTrivia};

fn main() {
    let test_str = "   /* some test */   abc def /* gh */ xyx";
    let mut out = <Vec<WithTrivia<Identifier>> as Parser<&str>>::parse(test_str)
        .unwrap()
        .1;

    println!("{:#?}", out);

    let mut writer = Writer(String::new());
    writer.visit(&out);

    println!("{}", writer.0);
}
