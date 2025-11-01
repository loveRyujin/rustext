#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    let mut editor = Editor::new();

    editor.run();
}
