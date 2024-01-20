use std::ffi::CString;

const BANNAR: &str = r#"
 ███▄ ▄███▓ ▄▄▄       ██▀███   ▄████▄   ▒█████  
▓██▒▀█▀ ██▒▒████▄    ▓██ ▒ ██▒▒██▀ ▀█  ▒██▒  ██▒
▓██    ▓██░▒██  ▀█▄  ▓██ ░▄█ ▒▒▓█    ▄ ▒██░  ██▒
▒██    ▒██ ░██▄▄▄▄██ ▒██▀▀█▄  ▒▓▓▄ ▄██▒▒██   ██░
▒██▒   ░██▒ ▓█   ▓██▒░██▓ ▒██▒▒ ▓███▀ ░░ ████▓▒░
░ ▒░   ░  ░ ▒▒   ▓▒█░░ ▒▓ ░▒▓░░ ░▒ ▒  ░░ ▒░▒░▒░ 
░  ░      ░  ▒   ▒▒ ░  ░▒ ░ ▒░  ░  ▒     ░ ▒ ▒░ 
░      ░     ░   ▒     ░░   ░ ░        ░ ░ ░ ▒  
       ░         ░  ░   ░     ░ ░          ░ ░  
                              ░                 


developer toyko2001@gmail.com
"#;

#[no_mangle]
pub fn get_bannar() -> *const i8 {
    let bannar = CString::new(BANNAR).expect("CString::new failed");
    bannar.into_raw()
}

#[no_mangle]
pub extern "C" fn free(ptr: *const i8) {
    unsafe {
        if ptr.is_null() { return }
        let _ = CString::from_raw(ptr as *mut i8);
    }
}
