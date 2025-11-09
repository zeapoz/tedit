use std::path::PathBuf;

use clap::Parser;

use crate::editor::{Editor, backend::TerminalBackend};

mod editor;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file to open. If empty, an empty buffer will be created instead.
    file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    TerminalBackend::initialize()?;

    let args = Args::parse();
    if let Ok(mut editor) = Editor::new(args.file) {
        editor.run()?;
    }

    TerminalBackend::deinitialize()?;

    Ok(())
}
