use super::{HEADER, HEADER_PART, LINE_BREAK, TAG_END, TAG_START};
use std::{
    env::var,
    fs::{create_dir_all, read, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    str::from_utf8,
};

pub fn write(path: &str, content: String) {
    let content = format!("{}\n\n{}", HEADER, pre(content));
    let path = make_dir(path);

    File::create(&path)
        .expect(&format!("Cannot create file: {:?}", path))
        .write_all(content.as_bytes())
        .expect(&format!("Cannot write in file: {:?}", path));

    Command::new("rustfmt")
        .arg(&path)
        .output()
        .expect(&format!("Cannot run rustfmt on file: {:?}", path));
}

pub fn write_part(path: &str, tag: &str, content: String) {
    let path = make_dir(path);

    let file = read(&path).expect(&format!("Cannot read file: {:?}", path));
    let file = from_utf8(&file).expect(&format!("Invalid UTF-8 in file: {:?}", file));

    let tag_start = TAG_START.to_string() + tag + "\n";
    let tag_end = TAG_END.to_string() + tag + "\n";

    if let Some(start) = file.find(&tag_start) {
        let start = start + tag_start.len();
        if let Some(end) = file.find(&tag_end) {
            let content = format!("{}\n\n{}", HEADER_PART.trim(), pre(content).trim());
            let content = file[..start].to_string() + &content + &file[end..];

            File::create(&path)
                .expect(&format!("Cannot create file: {:?}", path))
                .write_all(content.as_bytes())
                .expect(&format!("Cannot write in file: {:?}", path));

            Command::new("rustfmt")
                .arg(&path)
                .output()
                .expect(&format!("Cannot run rustfmt on file: {:?}", path));
        } else {
            panic!("Missing {:?} in file: {:?}", tag_end, path);
        }
    } else {
        panic!("Missing {:?} in file: {:?}", tag_start, path);
    }
}

fn pre(s: String) -> String {
    let s = s.replace(&format!(r#""{}""#, LINE_BREAK), "\n\n");
    s.lines().map(doc).collect::<String>()
}

fn make_dir(p: &str) -> PathBuf {
    let path =
        var("CARGO_MANIFEST_DIR").expect("Cannot get CARGO_MANIFEST_DIR environment variable");
    let path = path.to_string() + "/src/" + p;
    let path = Path::new(&path);
    let dir = path
        .parent()
        .expect(&format!("Cannot get parent directory of path: {:?}", path));

    create_dir_all(dir).expect(&format!("Cannot create location: {:?}", dir));

    path.to_path_buf()
}

fn doc(s: &str) -> String {
    let s = s.to_string();
    let s = replace(s, "# [doc = ", "/// ");
    let s = replace(s, "# ! [doc = ", "//! ");

    s + "\n"
}

fn replace(mut s: String, old: &str, new: &str) -> String {
    let mut index = 0;

    while let Some(i) = &s[index..].find(old) {
        let i = index + i;
        let mut j = i + old.len();
        if &s[j..j + 1] == "r" {
            j += 1;
        }
        let before = &s[..i];
        let after = &s[j + 1..].trim_start();

        let k = after
            .find("\"]")
            .expect("Cannot find \"] after #[doc = r?\"");
        let doc = after[..k].to_string();
        let after = after[k + 2..].to_string();

        s = before.to_string() + new + &doc + "\n";
        index = s.len();
        s = s + &after;
    }

    s
}
