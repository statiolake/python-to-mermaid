use anyhow::Result;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let source = {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        buf
    };

    let markdown = python_to_mermaid::python_file_to_markdown(&source)?;
    print!("{}", markdown);

    Ok(())
}
