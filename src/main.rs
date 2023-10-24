mod editor;
mod terminal;
mod document;
mod row;

pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
