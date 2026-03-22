
use std::{
    fs::*,
    path::*,
    env::*
};

const PATH_SEP: &str = if cfg!(any(target_os = "windows", feature = "safaos")) { ";" } else { ":" };

static mut PATHS : Vec<String> = Vec::new();

#[allow(static_mut_refs)]
pub fn set_path(path: &str) {
    unsafe { PATHS.clear() }; 
   
    let mut paths = path.to_string().split(PATH_SEP).map(|s| s.to_string()).collect();

    unsafe { PATHS.append(&mut paths);}
}

#[allow(static_mut_refs)]
pub fn resolve(util_name: &str) -> Option<String> {
    // bitchass cargo test expects it to be thread safe and allat 
    let paths = if cfg!(test) {
        //we dont give a damn if this one is slow
        let pathstr = match var("PATH") {
            Ok(v) => {
                v
            },
            Err(..) => {
                option_env!("REBOURNE_DEFAULT_PATH").unwrap_or("/usr/bin:/usr/local/bin:/usr/local/sbin").to_string()
            }
        };
        pathstr.to_string().split(PATH_SEP).map(|s| s.to_string()).collect()
    } else {
        unsafe { PATHS.clone() }
    };

    for p in paths {
        if !exists(&p).unwrap_or(false) {
            continue;
        }
        let i_hate_this_shi = Path::new(&p).join(util_name);
        let util_path = i_hate_this_shi.to_str().unwrap_or("");

        if !exists(&util_path).unwrap_or(false) {
            continue;
        }

        return Some(util_path.to_string()); 
    } 
    return None;
}
