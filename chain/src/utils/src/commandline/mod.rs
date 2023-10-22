use std::io::{Error, Write};
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
    if let Some(vec) = table_vec {
        print_table(vec);
    }
}

pub fn print_table<T: Into<String>>(table_data: Vec<Vec<T>>) {
    let table_data: Vec<Vec<String>> = table_data
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.into()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let mut req_size_arr: Vec<usize> = table_data[0]
        .iter()
        .map(|header| header.len() + 2)
        .collect::<Vec<usize>>();

    for row_no in 1..table_data.len() {
        for cell_no in 0..table_data[row_no].len() {
            let req_table_data_size: usize = table_data[row_no][cell_no].len() + 2;
            if req_size_arr[cell_no] < req_table_data_size {
                req_size_arr[cell_no] = req_table_data_size;
            } else if req_size_arr[cell_no] > req_table_data_size
                && (req_size_arr[cell_no] - req_table_data_size) % 2 != 0
            {
                req_size_arr[cell_no] += 1;
            }
        }
    }

    let draw_row_line = || {
        for line_size in req_size_arr.iter() {
            print!("+{}", "-".repeat(*line_size));
        }
        print!("+\n");
    };

    // print table to console
    for row_no in 0..table_data.len() {
        draw_row_line();
        for cell_no in 0..table_data[0].len() {
            let spacing_count: usize =
                (req_size_arr[cell_no] - table_data[row_no][cell_no].len()) / 2;
            print!(
                "|{}{}{}",
                " ".repeat(spacing_count),
                table_data[row_no][cell_no],
                " ".repeat(spacing_count),
            );
        }
        print!("|\n");
    }
    draw_row_line();
    println!("");
}
