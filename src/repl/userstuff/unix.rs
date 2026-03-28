use {
    std::{
        ffi::*,
        os::raw::c_char,
        ptr::*
    }
};

//only the name is needed
#[repr(C)]
struct Passwd {
    pw_name: *const c_char,
}

unsafe extern "C" {
    fn getuid() -> u32;
    fn gethostname(buf: *const c_char, size: usize) -> u32;
    fn getpwuid(uid: u32) -> *const Passwd;
}

pub fn get_uid() -> u32 { 
    unsafe{ getuid() } 
}

pub fn get_uname() -> String {
    let uid;
    let passwd;
    
    unsafe {
        uid = getuid();
        passwd = getpwuid(uid);
    }

    if passwd == null() {
        "user".to_string()        
    } else {
        unsafe { CStr::from_ptr((*passwd).pw_name)
            .to_str()
            .expect("Failed to convert username to safe string")
            .to_string() }
    }
}

pub fn get_hname() -> String {
    let len = 34;
    let mut buf = std::vec::from_elem(0 as c_char,len);

    let err = unsafe { gethostname(buf.as_mut_ptr() as *mut c_char, len as usize) };

    if err != 0 {
        "hostname".to_string()        
    } else {
        unsafe { CStr::from_ptr(buf.as_mut_ptr())
            .to_str()
            .expect("Failed to convert username to safe string")
            .to_string() }
    }
}
