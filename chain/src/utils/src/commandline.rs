use std::{
    cell,
    io::{Error, Write},
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn write_to_console(message: impl Into<String>, color: Option<Color>) -> Result<(), Error> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(color))?;
    writeln!(&mut stdout, "{}", message.into())?;
    Ok(())
}

const BANNER_ART: &str = r" 
                                       ___         ___         ___                   ___     
       ___      ___                   /  /\       /__/\       /  /\      ___        /__/\    
      /__/\    /  /\                 /  /:/       \  \:\     /  /::\    /  /\       \  \:\   
      \  \:\  /  /:/   ___     ___  /  /:/         \__\:\   /  /:/\:\  /  /:/        \  \:\  
       \  \:\/__/::\  /__/\   /  /\/  /:/  ___ ___ /  /::\ /  /:/~/::\/__/::\    _____\__\:\ 
        \__\:\__\/\:\_\  \:\ /  /:/__/:/  /  //__/\  /:/\:/__/:/ /:/\:\__\/\:\__/__/::::::::\ 
  /__/\ |  |:|  \  \:\/\  \:\  /:/\  \:\ /  /:\  \:\/:/__\\  \:\/:/__\/  \  \:\/\  \:\~~\~~\/
  \  \:\|  |:|   \__\::/\  \:\/:/  \  \:\  /:/ \  \::/     \  \::/        \__\::/\  \:\  ~~~ 
   \  \:\__|:|   /__/:/  \  \::/    \  \:\/:/   \  \:\      \  \:\        /__/:/  \  \:\     
    \__\::::/    \__\/    \__\/      \  \::/     \  \:\      \  \:\       \__\/    \  \:\    
        ~~~~                          \__\/       \__\/       \__\/                 \__\/    
";

const HR_LINE : &str = "+-------------------------------------------------------------------------------------------------------+";

pub fn draw_hr_line(color: Option<Color>) {
    write_to_console(HR_LINE, color).unwrap();
}

pub fn print_banner<T>(table_vec: Option<Vec<Vec<T>>>)
where
    T: Into<String>,
{
    write_to_console(BANNER_ART, Some(Color::Red)).unwrap();
    write_to_console("", Some(Color::Blue)).unwrap();
    if let Some(vec) = table_vec {
        print_table(vec);
    }
}

pub fn print_table<T: Into<String>>(table_data: Vec<Vec<T>>) {
    let table_data: Vec<Vec<String>> = table_data
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.into()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    todo!()
}
