use std::{
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
    let mut table_data: Vec<Vec<String>> = table_data
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.into()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let col_sizes = {
        let mut sizes: Vec<usize> = Vec::new();
        for cell_no in 0..table_data[0].len() {
            let mut biggest_size: usize = 0;
            for row_no in 0..table_data.len() {
                let req_size: usize = table_data[row_no][cell_no].len() + 2;
                if req_size > biggest_size {
                    biggest_size = req_size;
                }
            }
            sizes.push(biggest_size);
        }
        sizes
    };

    let draw_hr_line = || {
        let mut line: String = String::from("+");
        col_sizes.iter().for_each(|size| {
            line.push_str("-".repeat(*size).as_str());
            line.push('+');
        });
        println!("{}", line);
    };

    for cell_no in 0..table_data[0].len() {
        for row_no in 0..table_data.len() {
            let req_col_space = table_data[row_no][cell_no].len();
            let space_diff: usize = col_sizes[cell_no] - req_col_space;
            let spacing_count: (usize, usize) = {
                match (col_sizes[cell_no] == req_col_space, space_diff % 2 == 0) {
                    (true, _) => (1, 1),
                    (false, true) => (space_diff / 2, space_diff / 2),
                    (false, false) => (space_diff / 2 + 1, (space_diff / 2)),
                }
            };
            table_data[row_no][cell_no] = format!(
                "{}{}{}",
                " ".repeat(spacing_count.0),
                table_data[row_no][cell_no],
                " ".repeat(spacing_count.1)
            )
        }
    }

    for row_no in 0..table_data.len() {
        draw_hr_line();
        println!("|{}|", table_data[row_no].join("|"));
    }
    draw_hr_line();
}
