use std::{
    env::*, fs::read_to_string, path::*
};

mod resolver;
mod parser;
mod interp;
mod repl;

#[cfg(test)]
mod tests;

fn exec_script(path: &Path) {
    if !path.exists() {
        eprintln!("\"{}\" doesn't exist",path.display());
    }

    let contents = read_to_string(path).expect("failed to read script file");

    interp::run(&contents);
}

fn main() {
    match var("PATH") {
        Ok(v) => {
            resolver::set_path(&v)
        },
        Err(..) => {
            let default_path: &str = option_env!("REBOURNE_DEFAULT_PATH").unwrap_or("/usr/bin:/usr/local/bin:/usr/local/sbin");
            unsafe { set_var("PATH",default_path) };
            resolver::set_path(default_path)
        }
    }
    match var("PWD") {
        Err(..) => unsafe {
            set_var("PWD",current_dir().unwrap()) 
        },
        _ => () 
    };
    
    
    let mut args = args();

    if args.len() < 2 {
        match var("HOME") {
            Ok(v) => {
                let rbrc_path = Path::new(&v).join(".rbrc"); 
                if rbrc_path.exists() {
                    exec_script(&rbrc_path);
                }
            },
            _ => () 
        };

        repl::run();
    } else if args.len() < 4 {
        let second_arg  = args.nth(1).unwrap();
        if second_arg == "--help" {
            println!("just run the shell lmao, not like I have any options xD");
        } else {
            exec_script(Path::new(&second_arg));
        }
    }
}
