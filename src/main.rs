use anyhow::Result;
use python_to_mermaid::convert::{flowchart_to_mermaid, python_to_flowchart};
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let source = {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        buf
    };

    let fn_defs = python_to_flowchart::enumerate_fn_defs(&source)?;
    for fn_def in fn_defs {
        let name = fn_def.name.clone();

        let fc = python_to_flowchart::convert(fn_def)?;
        let mfc = flowchart_to_mermaid::convert(&fc);

        println!("## `{name}`");
        println!();
        println!("```mermaid");
        let mut s = String::new();
        mfc.render(&mut s);
        println!("{}", s);
        println!("```");
        println!();
    }

    Ok(())
}
