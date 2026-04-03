use {
    std::{
        env::*, 
    },
    super::userstuff::*
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

fn get_hname_up_to_first_dot() -> String {
    let hname = get_hname();
    
    match hname.split('.').nth(0) {
        Some(v) => v.to_string(),
        None => hname
    }
}

fn get_privilege_symbol() -> char {
    let uid = get_uid();

    if uid == 0 {
        return '#'
    } else {
        return '$'
    }
}

pub fn process(raw: &str, repl_state: &mut super::ReplState) -> String{
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
                   'u' => buffer.push_str(&get_uname()), 
                   'h' => buffer.push_str(&get_hname_up_to_first_dot()), 
                   'H' => buffer.push_str(&get_hname()), 
                   '!' => buffer.push_str(&format!("{}",repl_state.history_num)), 
                   '#' => buffer.push_str(&format!("{}",repl_state.command_num)), 
                   '$' => buffer.push(get_privilege_symbol()), 
                    _ => buffer.push(c)
                }
                parse_mode = PS1ParseMode::Normal;
            }
        }
    }
    return buffer;
}
