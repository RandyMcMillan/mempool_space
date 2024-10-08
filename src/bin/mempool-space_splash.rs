use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui_splash_screen::{SplashConfig, SplashScreen};

static SPLASH_CONFIG: SplashConfig = SplashConfig {
    image_data: include_bytes!("../../assets/splash.png"),
    sha256sum: Some("abc7e993ae85580df6a1349e89aa57d7d39cecdfe3cd5cc95f65b730aafab2cb"),
    render_steps: 3,
    use_colors: false,
};

fn splash() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut splash_screen = SplashScreen::new(SPLASH_CONFIG)?;
    while !splash_screen.is_rendered() {
        terminal.draw(|frame| {
            frame.render_widget(&mut splash_screen, frame.size());
        })?;
        sleep(Duration::from_millis(100));
    }

    sleep(Duration::from_secs(1));

    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
fn main() -> Result<()> {
    splash()
}
