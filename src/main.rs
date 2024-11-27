use leek_ast::{
    ast::{terminal::identifier::Identifier, trivia::with_trivia::WithTrivia},
    parser::Parser,
    visitor::{writer::Writer, Visitor},
};

fn main() {
    let test_str = "   /* some test */   abc def /* gh */ xyx";
    let out = <Vec<WithTrivia<Identifier>> as Parser<&str>>::parse(test_str)
        .unwrap()
        .1;

    println!("{:#?}", out);

    let mut writer = Writer(String::new());
    writer.visit(&out);

    println!("{}", writer.0);
}
