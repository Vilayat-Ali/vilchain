pub mod banner;

use std::io::{Error, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn write_to_console(message: impl Into<String>, color: Option<Color>) -> Result<(), Error> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(color))?;
    writeln!(&mut stdout, "{}", message.into())?;
    Ok(())
}
