use crate::Document;
use crate::Row;
use termion::event::Key;
use crate::Terminal;
use std::env;
use termion::color;
use std::time::Duration;
use std::time::Instant;

const STATUS_FG_COLOR: color:: Rgb = color::Rgb(63,63,63);
const STATUS_BG_COLOR: color:: Rgb = color::Rgb(239,239,239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position{
    pub x: usize,
    pub y: usize,
}

struct StatusMessage{
    text: String,
    time: Instant,
}
    
impl StatusMessage{
    fn from(message: String) -> Self{
        Self { 
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor{ //construtor
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage, 
}

impl Editor{
    pub fn run(&mut self){
        loop{ //main loop for processing data
            if let Err(error) = self.refresh_screen(){
                die(error);
            }
            if self.should_quit{
                break;
            }
            if let Err(error) = self.process_keypress(){
                die(error); 
            }
        }
    }
    #[allow(clippy::unnecessary_unwrap)]
    pub fn default() -> Self{
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-C = quit");
        let document = if args.len() > 1{
            let file_name = &args[1];
            let doc = Document::open(file_name);
            if doc.is_ok(){
                doc.unwrap()
            } else{
                initial_status = format!("ERR: Could not open file: {file_name}");
                Document::default()
            }
        }
        else{
            Document::default()
        };

        Self {
            should_quit:false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{ //clearing screen and drawing rows
        Terminal::cursor_hide(); //cursor hide and show are to hide the cursor when drawing to the screen
        Terminal::cursor_position(&Position::default());
        if self.should_quit{
            Terminal::clear_screen();
            println!("Goodbye.\r");
        }
        else{
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                 x: self.cursor_position.x.saturating_sub(self.offset.x), 
                 y: self.cursor_position.y.saturating_sub(self.offset.y), 
                }); //set cursor position to 0,0
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(),std::io::Error>{ //quitting helper function
        let pressed_key = Terminal::read_key()?;
        match pressed_key{
            Key::Ctrl('c') => self.should_quit = true, //set exit button to ctrl + c like terminal
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right); //move cursor to the right after key is pressed
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
            Key:: Up 
            | Key::Down 
            | Key::Left 
            | Key::Right 
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }

        self.scroll();
        Ok(())
    }

    fn scroll(&mut self){
        let Position {x,y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y{
            offset.y = y;
        } 
        else if y >= offset.y.saturating_add(height){
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x{
            offset.x = x;
        } 
        else if x >= offset.x.saturating_add(width){
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key:Key){
        let terminal_height = self.terminal.size().height as usize;
        let Position {mut y, mut x} = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y){
            row.len()
        } else{
            0
        };

        match key{
            Key::Up => y = y.saturating_sub(1),            
            Key::Down =>{
                if y < height{
                    y = y.saturating_add(1);
                }
            }           
            Key::Left =>{
                if x > 0 {
                    x -= 1;
                } else if y > 0{
                    y -= 1;
                    if let Some(row) = self.document.row(y){
                        x = row.len();
                    } else{
                        x = 0;
                    }
                }
            }
            Key::Right =>{
                if x < width {
                    x += 1;
                } else if y < height{
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp =>{
                y = if y > terminal_height{
                    y - terminal_height
                } else{
                    0
                }
            }
            Key::PageDown =>{
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height
                } else{
                    height
                }
            }
            Key:: Home => x = 0,
            Key::End => x = width,
            _ => (),
        }

        width = if let Some(row) = self.document.row(y){
            row.len()
        } else{
            0
        };

        if x > width{
            x = width;
        }
        self.cursor_position = Position {x,y}
    }

    fn draw_welcome_message(&self){ //setup to support any terminal size
        let mut welcome_message = format!("Hecto editor -- version {VERSION}");
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2; //center screen
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}"); //addes space formatting
        welcome_message.truncate(width); //shortens if needed
        println!("{welcome_message}\r");
    }
    pub fn draw_row(&self, row: &Row){
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start,end);
        println!("{row}\r")
    }

    fn draw_rows(&self){ //drawing ~'s at the beginning
        let height = self.terminal.size().height;
        for terminal_row in 0 .. height{
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y){
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height/3{
                self.draw_welcome_message();
            }
            else{
                println!("~\r")
            }
        }
    }

    fn draw_status_bar(&self){
        let mut status;
        let width = self.terminal.size().width as usize;
        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name{
            file_name = name.clone();
            file_name.truncate(20);
        }

        status = format!("{} - {} lines", file_name, self.document.len());
        
        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        
        let len = status.len() + line_indicator.len();
        if width > len{
            status.push_str(&" ".repeat(width - len));
        }

        status = format!("{status}{line_indicator}");
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{status}\r");
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self){
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5,0){
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{text}");
        }
    }
}


fn die(e: std::io::Error){ //for panicking
    Terminal::clear_screen();
    panic!("{}", e);
}