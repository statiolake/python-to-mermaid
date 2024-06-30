use std::fmt::Write as _;

use anyhow::Result;

use convert::{flowchart_to_mermaid, python_to_flowchart};

pub mod convert;
pub mod flowchart;
pub mod mermaid;

pub fn python_file_to_markdown(source: &str) -> Result<String> {
    let fn_defs = python_to_flowchart::enumerate_fn_defs(source)?;

    let mut buf = String::new();

    for (i, fn_def) in fn_defs.into_iter().enumerate() {
        let name = fn_def.name.clone();

        let fc = python_to_flowchart::convert(fn_def)?;
        let mfc = flowchart_to_mermaid::convert(&fc);

        if i > 0 {
            // Add a newline between flowcharts
            writeln!(buf)?;
        }

        writeln!(buf, "## `{name}`")?;
        writeln!(buf)?;
        writeln!(buf, "```mermaid")?;
        let mut s = String::new();
        mfc.render(&mut s);
        write!(buf, "{}", s)?;
        writeln!(buf, "```")?;
    }

    Ok(buf)
}
