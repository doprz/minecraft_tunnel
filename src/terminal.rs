use crossterm::{ExecutableCommand, QueueableCommand, cursor, style};
use std::io::{Write, stdout};

pub struct Terminal {
    width: usize,
    height: usize,
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Result<Self, Box<dyn std::error::Error>> {
        crossterm::terminal::enable_raw_mode()?;
        stdout().execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
        stdout().execute(cursor::Hide)?;

        Ok(Self { width, height })
    }

    /// Display frame using half-block characters for double vertical resolution
    pub fn display(&self, frame: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        stdout.queue(cursor::MoveTo(0, 0))?;

        // Process two rows at a time for half-block rendering
        for row in (0..self.height).step_by(2) {
            for col in 0..self.width {
                let top_idx = row * self.width + col;
                let bottom_idx = (row + 1) * self.width + col;

                // Extract RGB from packed u32
                let top_rgb = self.unpack_rgb(frame[top_idx]);
                let bottom_rgb = if bottom_idx < frame.len() {
                    self.unpack_rgb(frame[bottom_idx])
                } else {
                    (0, 0, 0) // Black if out of bounds
                };

                // Set colors and draw upper half block
                stdout.queue(style::SetForegroundColor(style::Color::Rgb {
                    r: top_rgb.0,
                    g: top_rgb.1,
                    b: top_rgb.2,
                }))?;
                stdout.queue(style::SetBackgroundColor(style::Color::Rgb {
                    r: bottom_rgb.0,
                    g: bottom_rgb.1,
                    b: bottom_rgb.2,
                }))?;
                stdout.queue(style::Print("▀"))?;
            }
            stdout.queue(style::Print("\r\n"))?;
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn display_texture_atlas(
        &self,
        textures: &[u32],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        stdout.queue(cursor::MoveTo(0, 0))?;

        for block in 0..10 {
            stdout.queue(style::Print(format!("Texture Block {}\r\n", block)))?;
            for y in 0..16 * 3 {
                for x in 0..16 {
                    let idx = x + y * 16 + block * 256 * 3;
                    let color = self.unpack_rgb(textures[idx]);
                    stdout.queue(style::SetForegroundColor(style::Color::Rgb {
                        r: color.0,
                        g: color.1,
                        b: color.2,
                    }))?;
                    stdout.queue(style::Print("██"))?;
                    stdout.execute(style::ResetColor).unwrap();
                }
                stdout.queue(style::Print("\r\n"))?;
                if y == 15 || y == 31 {
                    stdout.queue(style::Print("\r\n"))?;
                }
            }
            stdout.queue(style::Print("\r\n"))?;
        }

        stdout.flush()?;
        stdout.execute(style::ResetColor).unwrap();
        Ok(())
    }

    fn unpack_rgb(&self, color: u32) -> (u8, u8, u8) {
        (
            ((color >> 16) & 0xFF) as u8,
            ((color >> 8) & 0xFF) as u8,
            (color & 0xFF) as u8,
        )
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        stdout().execute(style::ResetColor).unwrap();
        stdout().execute(cursor::Show).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
