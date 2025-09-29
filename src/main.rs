use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

mod renderer;
mod terminal;
mod world;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dynamically get terminal size
    let terminal_size = crossterm::terminal::size().unwrap_or((60, 40 * 2));

    let width: usize = terminal_size.0 as usize;
    let height: usize = ((terminal_size.1 - 1) * 2) as usize; // Half-block rendering

    let world = world::World::new();
    let terminal = terminal::Terminal::new(width, height)?;
    let mut renderer = renderer::Renderer::new(width, height);

    let start_time = Instant::now();

    // DEBUG: Display texture atlas and exit
    // terminal.display_texture_atlas(&world.textures)?;
    // Ok(())

    loop {
        // Check for keyboard input (non-blocking)
        if event::poll(Duration::from_millis(0))?
            && let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
        {
            match code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    break;
                }
                _ => {} // Ignore other keys
            }
        }

        let elapsed = start_time.elapsed().as_millis() as f32;
        let frame = renderer.render(&world, elapsed);
        terminal.display(frame)?;

        // ~100 FPS target
        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
