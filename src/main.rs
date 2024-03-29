
#![warn(clippy::all)]
mod editor;
mod terminal;
mod document;
mod row;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;
pub use document::Document;

fn main() {
    Editor::default().run(); //run editor that does the work
}
