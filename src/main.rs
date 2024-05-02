use buffer::Buffer;
use editor::Editor;

mod buffer;
mod editor;

fn main() -> anyhow::Result<()> {
    let file_name = std::env::args().nth(1);
    let buffer = Buffer::from_file(file_name);

    let mut editor = Editor::new(buffer)?;

    editor.run()?;
    Ok(())
}
