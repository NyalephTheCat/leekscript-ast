use std::{path::PathBuf, str::FromStr};

use leek_ast::{
    ast::structure::file::File,
    parser::Parser,
    visitor::{includer::Includer, writer::Writer, Visitor, VisitorMut},
};

fn main() {
    let test_str = r#"
include('test.leek');
include('test.leek');

if (1) b

"#;
    let mut out: File = <_ as Parser<&str>>::parse(test_str).unwrap().1;

    println!("{:#?}", out);

    let mut includer = Includer {
        root_file: PathBuf::from_str("tests/").unwrap(),
        files_included: Default::default(),
    };
    includer.visit_mut(&mut out);

    let mut writer = Writer(String::new());
    writer.visit(&out);

    println!("{}", writer.0);
}
