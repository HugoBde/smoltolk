use color_eyre::Result;
use smoltolk::app::App;

fn main() -> Result<()> {
  let mut terminal = ratatui::init();
  let mut app = App::new();
  let result = app.run(&mut terminal);
  ratatui::restore();
  return result;
}
