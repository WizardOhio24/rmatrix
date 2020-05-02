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


use rand::{
    distributions::{Alphanumeric, Distribution, Uniform},
    Rng,
};


fn main() {
    let stdout = AlternateScreen::from(stdout());
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let mut lls = gen_letter_lists();
    write!(stdout, "{}", clear::All).unwrap();
    let mut rng = rand::thread_rng();
    let tsize = termion::terminal_size().unwrap();
    let mut offset: Vec<i16> = Uniform::new_inclusive(1, tsize.1 as i16 * 2)
        .sample_iter(rng)
        .take(lls.len())
        .collect();
    // Now generate a string which can be printed
    // and has all the correct letters
    //[100][8] == [8][100]
    // print the string vertically rather than horizontally

    loop {
        thread::sleep(Duration::from_millis(100));

        for (i, s) in lls.iter_mut().enumerate() {
            for (j, c) in s.chars().enumerate() {
                if offset[i] as isize + j as isize >= 0
                    && (offset[i] as isize + j as isize) < tsize.1 as isize
                {
                    if j == LETTER_LIST_LENGTH - 1 {
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
                            color::Fg(color::Green),
                            cursor::Goto(
                                i as u16 + 1,
                                (offset[i] as isize + j as isize) as u16 + 1
                            ),
                            c
                        )
                        .unwrap();
                    }
                }
                if offset[i] > (tsize.1 as i16 + LETTER_LIST_LENGTH as i16 * 2) {
                    offset[i] =
                        Uniform::new_inclusive(-(tsize.1 as i16), -(LETTER_LIST_LENGTH as i16))
                            .sample(&mut rng);
                }
            }

            // Add a new character to the start and remove the last character
            s.remove(0); //drain(0..1);
            s.push(rng.sample(&Alphanumeric));
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

const LETTER_LIST_LENGTH: usize = 16;

fn gen_letter_lists() -> Vec<String> {
    let rng = rand::thread_rng();
    let tsize = termion::terminal_size().unwrap();

    let mut gen = vec![
        String::from("");
        tsize.0 as usize
    ];

    for i in 0..gen.len() {
        gen[i] = rng
            .sample_iter(&Alphanumeric)
            .take(LETTER_LIST_LENGTH)
            .collect::<String>();
    }

    gen
}
