use std::{io::{self, stdout}, time::Duration};
// Import necessary modules from crossterm
use crossterm::{event::{self, KeyCode, Event, KeyEvent, MouseEvent, MouseEventKind}, terminal::{self, Clear}, cursor, ExecutableCommand, style::Stylize};

pub fn read_line() -> io::Result<()> {
    let mut stdout = stdout();
    loop {
        println!("loop");
        println!("{}","should be red".red());
        if let Event::Mouse(MouseEvent {kind: MouseEventKind::Moved, column, row, ..}) = event::read()? {
        }
    }

}

#[cfg(test)]
mod tests {
    use super::read_line;


}