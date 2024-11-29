use leek_ast::{
    ast::{structure::File, terminal::identifier::Identifier, trivia::with_trivia::WithTrivia},
    parser::Parser,
    visitor::{writer::Writer, Visitor},
};

fn main() {
    let test_str = r#"
// Set une globale test
global a;
"#;
    let out: File = <_ as Parser<&str>>::parse(test_str).unwrap().1;

    println!("{:#?}", out);

    let mut writer = Writer(String::new());
    writer.visit(&out);

    println!("{}", writer.0);
}
