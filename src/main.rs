// This is a program written in Rust to simulate
// what the matrix screens looked like.

/*
t u   v i
y c   u
x
*/
use termion::color;
use termion::style;
use termion::clear;
use termion::cursor;
use termion::async_stdin;
//use termion::event::Key;

use termion::event::{Key, Event, MouseEvent};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::thread;
use std::time::Duration;
use std::io::{Read, Write, stdout, stdin};
//use rand::prelude::*;
use rand::{Rng, distributions::Distribution, distributions::Alphanumeric, distributions::Uniform};
use termion::screen::AlternateScreen;

fn main() {
    //let stdin = stdin()
    {
    let mut stdout = AlternateScreen::from(stdout());
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout.into_raw_mode().unwrap();
    //println!("{red}Hello, world!{reset}", red=color::Fg(color::Red), reset = color::Fg(color::Reset));
    let mut lls = gen_letter_lists();
    //println!("{}", lls[0]);
    write!(stdout, "{}", clear::All).unwrap();
    //write!(stdout, "{}", lls[0]).unwrap();
    //stdout.flush().unwrap();
    let mut rng = rand::thread_rng();
    //let uniform_range = Uniform::new_inclusive(1, 100);
    let tsize = termion::terminal_size().unwrap();
    let mut offset: Vec<i16> = Uniform::new_inclusive(1, tsize.1 as i16 * 2).sample_iter(rng).take(lls.len()).collect();
    // Now generate a string which can be printed
    // and has all the correct letters
    //[100][8] == [8][100]

    loop{
        thread::sleep(Duration::from_millis(100));

        //write!(stdout, "{}", cursor::Goto(2, 2)).unwrap();
        for (i, s) in lls.iter_mut().enumerate(){
            //offset[i] += 1;
            let mut tempoff: isize = offset[i] as isize;
            for (j, c) in s.chars().enumerate(){
                //println!("Somehting");
                //(1-(LETTER_LIST_LENGTH as i32 * 3))
                if tempoff + j as isize >= 0 && (tempoff + j as isize) < tsize.1 as isize   {
                    if j == LETTER_LIST_LENGTH -1 {
                        write!(stdout, "{}{}{}", color::Fg(color::White), cursor::Goto( i as u16 + 1,  (tempoff + j as isize) as u16 + 1), c).unwrap();

                    }else{
                        write!(stdout, "{}{}{}", color::Fg(color::Green), cursor::Goto( i as u16 + 1,  (tempoff + j as isize) as u16 + 1), c).unwrap();

                    }
            }else{
                //tempoff += 1;
            }
                if offset[i] > (tsize.1 as i16 + LETTER_LIST_LENGTH as i16 * 2) {
                    offset[i] = Uniform::new_inclusive(-(tsize.1 as i16),  -(LETTER_LIST_LENGTH as i16)).sample(&mut rng);
                    //tempoff = offset[i];
                    //tempoff = offset[i] as isize;
                }

            }

            // make the first char random
            //s = &String::from((s[1..]));
            //s.as_str().rotate_left(1);
            s.remove(0); //drain(0..1);
            s.push(rng.sample(&Alphanumeric));


        }
        stdout.flush().unwrap();
        write!(stdout, "{}", clear::All).unwrap();
        for i in &mut offset{
            *i += 1;
        }



        // if wanted a new letter to be at the start each time, just
        // change the string representation, remove the last, shift the chars
        // and add a new first

        //offset.into_iter().map(|x| x+1);//[i] += 1;
        //write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        // Input
        let b = stdin.next();
        //write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();
        let tsize = termion::terminal_size().unwrap();
        //println!("{}, {}", tsize.0, tsize.1);
        if let Some(Ok(b'q')) = b {


            //write!(stdout, "{}", termion::screen::ToMainScreen).unwrap();
            break;
        }
        /*for c in stdin().events(){
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char('q')) => break,
                _ => {}
            }
        }*/

    }}

    println!("{:?}", gen_letter_lists());
/*
        for c in stdin().keys() {
    match c.unwrap() {
        Key::Char('q') => break,
/*        Key::Char(c) => println!("{}", c),
        Key::Alt(c) => println!("^{}", c),
        Key::Ctrl(c) => println!("*{}", c),
        Key::Esc => println!("ESC"),
        Key::Left => println!("←"),
        Key::Right => println!("→"),
        Key::Up => println!("↑"),
        Key::Down => println!("↓"),
        Key::Backspace => println!("×"),*/
        _ => {}
    }
    stdout.flush().unwrap();
}*/



}

const LETTER_LIST_LENGTH: usize = 16;

fn gen_letter_lists() -> Vec<String>{
    let rng = rand::thread_rng();
    let tsize = termion::terminal_size().unwrap();

    let mut gen = vec![rng.sample_iter(&Alphanumeric).take(LETTER_LIST_LENGTH).collect::<String>(); tsize.0 as usize];
    //let lls = vec![LetterList::new(rng); t.size];
    for i in 0..gen.len(){
        gen[i] = rng.sample_iter(&Alphanumeric).take(LETTER_LIST_LENGTH).collect::<String>();
    }

    gen
}


// wrap a number in it and only return the number if it is positive
struct posStruct{

}

/*
// just an abstraction
struct LetterList{
    letters: Vec<char>,
}

impl LetterList{

    fn new(randGen: ThreadRng) -> LetterList{
        return LetterList{ letters: vec![randGen.gen(); LETTER_LIST_LENGTH]}
    }
}
*/
