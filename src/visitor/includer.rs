use std::{collections::HashSet, path::PathBuf};

use crate::{
    ast::{
        statements::{GlobalFlag, Statements},
        structure::file::File,
        trivia::trivia::Trivia,
        utils::flags::WithFlag,
    },
    parser::Parser,
    visitor::{VisitableMut, VisitorMut},
};

use super::{writer::Writer, Visitor};

#[derive(Debug, Default, Clone)]
pub struct Includer {
    pub root_file: PathBuf,
    pub files_included: HashSet<String>,
}
impl VisitorMut for Includer {
    fn visit_mut<T>(&mut self, t: &mut T)
    where
        T: VisitableMut<Self> + ?Sized,
    {
        t.accept_mut(self);
    }
}

impl<S: WithFlag<GlobalFlag>> VisitableMut<Includer> for Statements<S> {
    fn accept_mut(&mut self, v: &mut Includer) {
        match self {
            Self::IncludeStatement(_, node) => {
                let path = node.path.node.content.clone();
                let mut include_stmt = Writer(String::new());
                include_stmt.visit(node);

                if v.files_included.insert(path.clone()) {
                    // Todo: Parse the file and visit it
                    let filepath = v.root_file.join(path);
                    let mut content = std::fs::read_to_string(filepath).unwrap();
                    // Prepend a comment to the file to indicate it was included
                    content = format!(
                        "/* start: {} */\n{}\n/* end: {} */",
                        include_stmt.0, content, include_stmt.0
                    );

                    let mut file = <File as Parser<&str>>::parse(&content).unwrap().1;
                    file.accept_mut(v);
                    let statement = Statements::IncludedFile(Box::new(file));

                    *self = statement;
                } else {
                    let trivia = <Vec<Trivia> as Parser<&str>>::parse(
                        format!("/* skipped: {} (Already included) */", include_stmt.0).as_str(),
                    )
                    .unwrap()
                    .1;
                    *self = Statements::EmptyStatement(trivia);
                }
            }
            _ => {}
        }
    }
}
