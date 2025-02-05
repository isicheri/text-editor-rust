mod editor;
use editor::Editor;

#[warn(clippy::all, clippy::pedantic)]
fn main() {
  Editor::default().run();
}