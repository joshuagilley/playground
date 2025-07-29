use std::{error::Error, time::Duration};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{backend::CrosstermBackend, Terminal};
use sysinfo::{System};  // Only importing SystemExt to handle system data (not CpuExt)

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App loop
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = std::time::Instant::now();

    // Create system instance
    let mut sys = System::new_all();  // This will automatically load all info

    loop {
        // Refresh system information (CPU, memory, etc.)
        sys.refresh_cpu_all();  // Refresh CPU info
        sys.refresh_memory(); // Refresh memory info

        // Calculate average CPU usage
        let cpu_usage = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

        // Render UI
        terminal.draw(|f| {
            let size = f.area();
            let block = ratatui::widgets::Block::default()
                .title("CPU Usage")
                .borders(ratatui::widgets::Borders::ALL);

            let gauge = ratatui::widgets::Gauge::default()
                .block(block)
                .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green))
                .label(format!("{:.1}%", cpu_usage))
                .ratio(cpu_usage as f64 / 100.0);
            f.render_widget(gauge, size);
        })?;

        // Event handling
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }

    // Cleanup
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
