#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal; //public because syntax


fn main() {
    Editor::default().run(); //run editor that does the work
}
