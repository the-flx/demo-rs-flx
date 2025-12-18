include!("./flx.rs");

use std::ffi::CString;

fn main() {
    let str = CString::new("buffer-file-name").expect("CString::new failed");
    let query = CString::new("bfn").expect("CString::new failed");

    unsafe {
        let result = flx_score(str.as_ptr(), query.as_ptr());

        println!("Hello, world!: {:p}", result);
    }
}
