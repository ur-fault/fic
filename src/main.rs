use std::fs::File;
use std::io::Read;
use std::path::{self, PathBuf};

use clap::Parser;
use colored::{Color, Colorize};

#[derive(Parser, Debug)]
#[command(author, about)]
struct Args {
    file: PathBuf,
}

fn to_color(num: u8) -> (Color, Color) {
    fn to_color_inner(num: u8) -> Color {
        match num {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::White,
            8 => Color::BrightBlack,
            9 => Color::BrightRed,
            10 => Color::BrightGreen,
            11 => Color::BrightYellow,
            12 => Color::BrightBlue,
            13 => Color::BrightMagenta,
            14 => Color::BrightCyan,
            15 => Color::BrightWhite,
            _ => panic!("Invalid number"),
        }
    }

    let (div, rem) = (num / 16, num % 16);
    // dbg!("{} {}", div, rem);

    (to_color_inner(div), to_color_inner(rem))
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(path::Path::new(&args.file)).expect(&"File not found".red().to_string());
    for buf in file.bytes() {
        match buf {
            Ok(b) => {
                let (fg, bg) = to_color(b);
                print!("{}", "#".on_color(bg).color(fg));
            },
            Err(e) => println!("{}", e),
        }
    }
}
