use crate::*;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

pub const LINE_BREAK: &str = "__LINE_BREAK__";

const HEADER: &str = "
    ////////////////////////////////////////////////////////////////////////////////
    // 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
    // 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
    ////////////////////////////////////////////////////////////////////////////////
";

pub fn write(dir: &str, path: &str, content: TokenStream) {
    let content = format!("{}\n\n{}", HEADER, pre(content));
    let path = make_dir(dir, path);

    File::create(&path)
        .expect(&format!("Cannot create file: {:?}", path))
        .write_all(content.as_bytes())
        .expect(&format!("Cannot write in file: {:?}", path));
}

fn pre(tokens: TokenStream) -> String {
    let string = tokens
        .to_string()
        .replace(&format!(r#""{}""#, LINE_BREAK), "\n\n");

    Doc::replace(string)
}

fn make_dir(dir: &str, p: &str) -> PathBuf {
    let path = dir.to_string() + p;
    let path = Path::new(&path);
    let dir = path
        .parent()
        .expect(&format!("Cannot get parent directory of path: {:?}", path));

    create_dir_all(dir).expect(&format!("Cannot create location: {:?}", dir));

    path.to_path_buf()
}