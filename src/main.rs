use anyhow::Result;
use smoltolk::app::App;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let mut terminal = ratatui::init();
  let mut app = App::new();
  let result = app.run(&mut terminal).await;
  ratatui::restore();
  return result;
}
