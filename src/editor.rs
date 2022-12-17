use termion::event::Key;
use crate::Terminal;

pub struct Editor{ //construtor
    should_quit: bool,
    terminal: Terminal,
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
    pub fn default() -> Self{
        Self {
            should_quit:false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error>{ //clearing screen and drawing rows
        Terminal::cursor_hide(); //cursor hide and show are to hide the cursor when drawing to the screen
        Terminal::clear_screen();
        if self.should_quit{
            println!("Goodbye. \r");
        }
        else{
            self.draw_rows();
            Terminal::cursor_position(0,0); //set cursor position to 0,0
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(),std::io::Error>{ //quitting helper function
        let pressed_key = Terminal::read_key()?;
        match pressed_key{
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
    fn draw_rows(&self){ //drawing ~'s at the beginning
        for _ in 0..self.terminal.size().height -1{
            println!("~\r");
        }
    }
}


fn die(e: std::io::Error){ //for panicking
    Terminal::clear_screen();
    panic!("{}", e);
}