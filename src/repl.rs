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

fn get_curr_dir() -> String {
    let dir = current_dir()
                .expect("Failed to get current directory")
                .to_str()
                .expect("Failed to convert current directory to string")
                .to_string();
    
    match home_dir() {
        Some(v) => {
            dir.replace(v.to_str().expect("Couldn't convert home directory to string"), "~")
        },
        None => dir
    }
}

fn get_curr_dir_basename() -> String {
    let dir = current_dir()
                .expect("Failed to get current directory")
                .file_name()
                .expect("Failed to get the basename of the current directory")
                .to_str()
                .expect("Failed to convert current directory basename to string")
                .to_string();

    dir
}

fn get_version_string(patchlevel: bool) -> String {
    let maj   = env!("CARGO_PKG_VERSION_MAJOR");
    let min   = env!("CARGO_PKG_VERSION_MINOR");
    let patch = env!("CARGO_PKG_VERSION_PATCH");

    if patchlevel {
        format!("{}.{}.{}", maj, min, patch)
    } else {
        format!("{}.{}", maj, min)
    }
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
                   'w' => buffer.push_str(&get_curr_dir()), 
                   'W' => buffer.push_str(&get_curr_dir_basename()), 
                   'v' => buffer.push_str(&get_version_string(false)), 
                   'V' => buffer.push_str(&get_version_string(true)), 
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
