use {
    std::{
        env::*, 
        io::*
    },
};

struct ReplState {
    history_num: usize,
    command_num: usize
}

mod prompt;
mod userstuff;

fn print_prompt(repl_state: &mut ReplState) {
   match var("PS1") {
        Ok(v) => {
            print!("{}",prompt::process(&v, repl_state));
        },
        Err(..) => {
            let default_ps1 = option_env!("RB_DEFAULT_PS1").unwrap_or("[\\u@\\h \\W]\\$ ");
            unsafe { set_var("PS1",default_ps1) };
            print!("{}",prompt::process(default_ps1, repl_state));
        }
    } 
    stdout().flush().unwrap();
}

pub fn run() {
    let mut repl_state = ReplState { 
        history_num: 0, 
        command_num: 0 
    };
    loop {
        print_prompt(&mut repl_state);
        let mut instr = String::new(); 
        stdin().read_line(&mut instr).unwrap();
        super::interp::run(instr.as_str());
        repl_state.command_num += 1;
    }
}
