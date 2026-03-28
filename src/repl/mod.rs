use {
    std::{
        env::*, 
        io::*
    },
};

mod prompt;

fn print_prompt() {
   match var("PS1") {
        Ok(v) => {
            print!("{}",prompt::process(&v));
        },
        Err(..) => {
            print!("{}",prompt::process("\\w> "));
        }
    } 
    stdout().flush().unwrap();
}

pub fn run() {
    loop {
        print_prompt();
        let mut instr = String::new(); 
        stdin().read_line(&mut instr).unwrap();
        super::interp::run(instr.as_str());
    }
}
