use std::io::{Write, Error, stdout, stdin};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::{clear, cursor, color, style};
use rand::seq::IteratorRandom;

const LENGTH: usize = 5;
const ATTEMPTS: usize = 6;

fn main() {
  if let Err(e) = Rordle::new().play() {
    println!("\r{}Error: {:?}", color::Fg(color::Red), e)
  }
}

struct Rordle {
  stdout: Box<dyn Write>,
  objective: String,
  attempts: Vec<String>,
}

impl Rordle {
  fn new() -> Self {
    Self {
      stdout: Box::new(stdout().into_raw_mode().unwrap()),
      objective: include_str!("dict.txt")
        .lines()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string(),
      attempts: vec![String::with_capacity(LENGTH)],
    }
  }

  fn play(&mut self) -> Result<(), Error> {
    self.render()?;
    for c in stdin().keys() {
      if let Ok(c) = c {
        match c {
          Key::Char(c) => {
            let attempt = self.attempts.last_mut().unwrap();
            if c.is_alphabetic() && attempt.len() < LENGTH {
              attempt.push(c.to_uppercase().next().unwrap());
            }
            if c == '\n' && attempt.len() == LENGTH {
              if *attempt == self.objective {
                write!(
                  self.stdout,
                  "{}{}{}You Won!",
                  cursor::Goto(1, ATTEMPTS as u16 * 2 + 3),
                  color::Fg(color::Green),
                  style::Bold
                )?;
                break;
              }
              if self.attempts.len() == ATTEMPTS {
                write!(
                  self.stdout,
                  "{}{}{}You Lost!{}\n\rThe word was {}\"{}\"{}.",
                  cursor::Goto(1, ATTEMPTS as u16 * 2 + 3),
                  color::Fg(color::Red),
                  style::Bold,
                  style::Reset,
                  style::Italic,
                  self.objective,
                  style::Reset
                )?;
                break;
              }
              self.attempts.push(String::with_capacity(LENGTH));
            }
          }
          Key::Backspace => {
            self.attempts.last_mut().unwrap().pop();
          }
          Key::Esc => break,
          _ => {}
        }
      }
      self.render()?;
    }
    Ok(())
  }

  fn render(&mut self) -> Result<(), Error> {
    write!(
      self.stdout,
      "{}{}{}{}Rordle{}",
      clear::All,
      cursor::Goto(1, 1),
      color::Fg(color::Blue),
      style::Bold,
      style::Reset
    )?;

    for i in 0..ATTEMPTS {
      write!(self.stdout, "{}", cursor::Goto(1, i as u16 * 2 + 3))?;
      for j in 0..LENGTH {
        let mut chr = ' ';
        write!(self.stdout, " {}", color::Bg(color::Black))?;
        if let Some(attempt) = self.attempts.get(i) {
          if let Some(c) = attempt.chars().nth(j) {
            chr = c;
            if i != self.attempts.len() - 1 {
              if c == self.objective.chars().nth(j).unwrap() {
                write!(
                  self.stdout,
                  "{}{}",
                  color::Bg(color::Green),
                  color::Fg(color::Black)
                )?;
              } else if self.objective.contains(c) {
                write!(
                  self.stdout,
                  "{}{}",
                  color::Bg(color::Yellow),
                  color::Fg(color::Black)
                )?;
              }
            }
          }
        }
        write!(self.stdout, "{}{} ", chr, style::Reset)?;
      }
    }
    write!(
      self.stdout,
      "{}",
      cursor::Goto(
        (self.attempts.last().unwrap().len()) as u16 * 3 + 2,
        (self.attempts.len() - 1) as u16 * 2 + 3
      )
    )?;
    self.stdout.flush()?;
    Ok(())
  }
}
