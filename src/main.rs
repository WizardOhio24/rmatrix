// This is a program written in Rust to simulate
// what the matrix screens looked like.
// It was inspired by cmatrix

/*
t u   v i
y c   u
x
*/

use termion::color;
//use termion::style;
use termion::async_stdin;
use termion::clear;
use termion::cursor;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
//use termion::event::Key;

//use termion::event::{Key, Event, MouseEvent};
//use termion::input::TermRead;
use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;

use clap::{Arg, App};


use rand::{
    distributions::{Alphanumeric, Distribution, Uniform},
    Rng,
};

fn main() {


    //
    // Clap,
    let matches = App::new("rmatrix")
                      .version("1.0")
                      .author("Richard Menzies rdmen2000@gmail.com")
                      .about("Displays letters flowing down the screen, like in the film the matrix")
                      .arg(Arg::with_name("length")
                           .short("l")
                           .long("listlength")
                           .value_name("LENGTH")
                           .help("Set the length of the strands of letters")
                           .default_value("16")
                           .takes_value(true))
                      .arg(Arg::with_name("color")
                            .short("c")
                            .long("lettercolor")
                            .value_name("COLOR")
                            .help("Sets the letter color")
                            .default_value("green")
                            .takes_value(true)
                       )
                       .arg(Arg::with_name("speed")
                             .short("s")
                             .long("speed")
                             .value_name("SPEED")
                             .help("Sets the speed the letter go down the screen")
                             .default_value("1")
                             .takes_value(true)
                        )
                        .arg(Arg::with_name("letters")
                             .help("Sets the letters to flow down the screen")
                             .value_name("LETTERS")
                             .default_value("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
                             .takes_value(true)
                        )
                      .get_matches();


    let letter_list_length: usize = matches.value_of("length").unwrap().parse::<>().unwrap();
    let letters: Vec<char> = matches.value_of("letters").unwrap().chars().collect();
    let let_len = letters.len();
    let speed: u64 = ((1.0 / matches.value_of("speed").unwrap().parse::<f64>().unwrap()) * 100 as f64) as u64;

    let letter_color: &str = match matches.value_of("color").unwrap(){
        "green" => color::Green.fg_str(),
        "blue" => color::Blue.fg_str(),
        "black" => color::Black.fg_str(),
        "white" => color::White.fg_str(),
        "magenta" => color::Magenta.fg_str(),
        "cyan" => color::Cyan.fg_str(),
        "red" => color::Red.fg_str(),
        "yellow" => color::Yellow.fg_str(),
        "lightred" => color::LightRed.fg_str(),
        "lightblue" => color::LightBlue.fg_str(),
        "lightgreen" => color::LightGreen.fg_str(),
        "lightmagenta" => color::LightMagenta.fg_str(),
        "lightyellow" => color::LightYellow.fg_str(),
        "lightwhite" => color::LightWhite.fg_str(),
        "lightcyan" => color::LightCyan.fg_str(),
        _ => {color::Green.fg_str()}
    };

    let stdout = AlternateScreen::from(stdout());
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let mut lls = gen_letter_lists(letter_list_length);
    write!(stdout, "{}", clear::All).unwrap();
    let mut rng = rand::thread_rng();
    let tsize = termion::terminal_size().unwrap();
    let mut offset: Vec<i16> = Uniform::new_inclusive(1, tsize.1 as i16 * 2)
        .sample_iter(rng)
        .take(lls.len())
        .collect();

    //[100][8] == [8][100]
    // print the string vertically rather than horizontally

    loop {
        thread::sleep(Duration::from_millis(speed));

        for (i, s) in lls.iter_mut().enumerate() {
            for (j, c) in s.chars().enumerate() {
                if offset[i] as isize + j as isize >= 0
                    && (offset[i] as isize + j as isize) < tsize.1 as isize
                {
                    if j == letter_list_length - 1 {
                        write!(
                            stdout,
                            "{}{}{}",
                            color::Fg(color::White),
                            cursor::Goto(
                                i as u16 + 1,
                                (offset[i] as isize + j as isize) as u16 + 1
                            ),
                            c
                        )
                        .unwrap();
                    } else {
                        write!(
                            stdout,
                            "{}{}{}",
                            letter_color,
                            cursor::Goto(
                                i as u16 + 1,
                                (offset[i] as isize + j as isize) as u16 + 1
                            ),
                            c
                        )
                        .unwrap();
                    }
                }
                if offset[i] > (tsize.1 as i16 + letter_list_length as i16 * 2) {
                    offset[i] =
                        Uniform::new_inclusive(-(tsize.1 as i16), -(letter_list_length as i16))
                            .sample(&mut rng);
                }
            }

            // Add a new character to the start and remove the last character
            s.remove(0); //drain(0..1);
            s.push(letters[rand::thread_rng().gen_range(0, let_len)]);//rng.sample(&Alphanumeric));
        }
        stdout.flush().unwrap();
        write!(stdout, "{}", clear::All).unwrap();
        for i in &mut offset {
            *i += 1;
        }

        // Input
        let b = stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }
    }
    //println!("{:?}", gen_letter_lists());
}

//const letter_list_length: usize = 16;

fn gen_letter_lists(letter_list_length: usize) -> Vec<String> {
    let rng = rand::thread_rng();
    let tsize = termion::terminal_size().unwrap();

    let mut gen = vec![
        String::from("");
        tsize.0 as usize
    ];

    for i in 0..gen.len() {
        gen[i] = rng
            .sample_iter(&Alphanumeric)
            .take(letter_list_length)
            .collect::<String>();
    }

    gen
}
