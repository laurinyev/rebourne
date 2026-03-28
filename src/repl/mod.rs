use {
    std::{
        env::*, 
        io::*
    },
};

mod prompt;
mod userstuff;

fn print_prompt() {
   match var("PS1") {
        Ok(v) => {
            print!("{}",prompt::process(&v));
        },
        Err(..) => {
            let default_ps1 = option_env!("RB_DEFAULT_PS1").unwrap_or("[\\u@\\h \\W]\\$ ");
            unsafe { set_var("PS1",default_ps1) };
            print!("{}",prompt::process(default_ps1));
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
