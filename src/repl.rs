use {
    std::{
        env::*, 
        io::*
    },
    super::interp
};

enum PS1ParseMode {
   Normal,
   Escape
}

fn process_prompt(raw: &str) -> String{
    let mut buffer = String::new();
    let mut parse_mode = PS1ParseMode::Normal;

    for c in raw.chars() {
        match parse_mode {
            PS1ParseMode::Normal => {
                match c {
                    '\\' => {
                        parse_mode = PS1ParseMode::Escape;
                    },
                    _ => buffer.push(c)
                }
            },
            PS1ParseMode::Escape => {
                match c {
                   'w' => buffer.push_str(current_dir().expect("Failed to get current directory").to_str().expect("Failed to get current directory")), 
                    _ => buffer.push(c)
                }
                parse_mode = PS1ParseMode::Normal;
            }
        }
    }
    return buffer;
}

fn print_prompt() {
   match var("PS1") {
        Ok(v) => {
            print!("{}",process_prompt(&v));
        },
        Err(..) => {
            print!("{}> ",current_dir().unwrap().display());
        }
    } 
    stdout().flush().unwrap();
}

pub fn run() {
    loop {
        print_prompt();
        let mut instr = String::new(); 
        stdin().read_line(&mut instr).unwrap();
        interp::run(instr.as_str());
    }
}
