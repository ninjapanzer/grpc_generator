use std::path::PathBuf;
use rustpython_parser::{Parse, ast, Tok::Class as ClassDef};
use std::fs;

pub struct Parser {
    file: PathBuf,
    pub(crate) classes: Vec<String>,
}

impl Parser {
    pub fn collect_classes(file: &PathBuf) -> Self {
        let mut classes: Vec<String> = vec![];
        let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
        let ast = ast::Suite::parse(&contents, &file.parent().expect("No parent directory found").to_string_lossy());
        let program = ast::Suite::parse(&contents, "<embedded>");
        program.unwrap().iter().for_each(|node| {
            match node {
                ast::Stmt::ClassDef(node) => {
                    classes.push(node.name.to_string());
                }
                _ => {}
            }
        });

        println!("{:?}", classes);
        Self {
            file: file.clone(),
            classes,
        }
    }
}