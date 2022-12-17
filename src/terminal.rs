use std::io::{self,stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size{ //struct for the size that can be referenced
    pub width: u16,
    pub height: u16,
}

pub struct Terminal{ //public constructor
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal{
    pub fn default() -> Result<Self, std::io::Error>{
        let size = termion::terminal_size()?;
        Ok(Self { //default constructor
            size: Size{
                width: size.0, //setting size struct
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size{ //accessor
        &self.size
    }
    pub fn clear_screen(){ //clears screen
        print!("{}", termion::clear::All);
    }
    pub fn cursor_position(x: u16, y: u16){ //reset cursor
        let x = x.saturating_add(1); //do this to hide that termion is 1 based, not 0 based
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x,y));
    }
    pub fn flush() -> Result<(), std::io::Error>{ //flush
        io::stdout().flush()
    }
    pub fn read_key() -> Result<Key, std::io::Error>{ //reading keys from stdin
        loop{
            if let Some(key) = io::stdin().lock().keys().next(){
                return key;
            }
        }
    }
    pub fn cursor_hide(){
        print!("{}", termion::cursor::Hide);
    }
    pub fn cursor_show(){
        print!("{}", termion::cursor::Show);
    }
}