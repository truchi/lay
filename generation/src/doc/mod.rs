use crate::*;

macro_rules! generate_doc {
    ($src:ident, $examples:ident, $name:literal) => {
        generate_doc($src, $examples, $name, include_str!($name));
    };
}

impl Generation {
    pub fn generate_docs(&self, src: &str, examples: &str) {
        generate_doc!(src, examples, "style.doc.rs");
    }
}

fn generate_doc(src: &str, examples: &str, name: &str, doc: &str) {
    write(src, name, doc);

    for (name, code) in get_code_blocks(&doc) {
        write(
            examples,
            &format!("{}.rs", name),
            format!("fn main() {{ {} }}", code),
        );
    }
}

fn get_code_blocks(mut input: &str) -> Vec<(&str, String)> {
    let mut blocks = vec![];

    let tag = "/// ```";
    let prefix = "///";
    let commented_prefix = "/// #";
    let example_command_start = "\n/// $ `cargo run --quiet --example ";
    let example_command_end = "`\n";

    while let Some(start) = input.find(tag) {
        let mut block = &input[start + tag.len()..];

        let end = block.find(tag).expect("Cannot find closing ```");
        input = &block[end + tag.len()..];
        block = &block[..end];

        let block = block
            .trim()
            .lines()
            .map(|line| {
                if line.starts_with(commented_prefix) {
                    &line[commented_prefix.len()..]
                } else if line.starts_with(prefix) {
                    &line[prefix.len()..]
                } else {
                    unreachable!("Code block not starting with doc comment")
                }
            })
            .map(|line| format!("{}\n", line))
            .collect::<String>();

        if !input.starts_with(example_command_start) {
            continue;
        }

        input = &input[example_command_start.len()..];
        let end = input
            .find(example_command_end)
            .expect("Cannot find example command end");
        let name = &input[..end];
        input = &input[end..];

        blocks.push((name.trim(), block));
    }

    blocks
}
