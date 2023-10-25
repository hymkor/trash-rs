extern crate glob;

use core::ffi::c_void;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::UI::Shell::{
    SHFileOperationW, FOF_ALLOWUNDO, FOF_NOCONFIRMATION, FO_DELETE, SHFILEOPSTRUCTW,
};

fn append_fname(buffer: &mut Vec<u16>, fname: &str) {
    println!("{}", fname);
    let mut fname_vec: Vec<u16> = fname.encode_utf16().collect();
    buffer.append(&mut fname_vec);
    buffer.push(0)
}

fn trash() -> Result<i32, Box<dyn std::error::Error>> {
    let mut source: Vec<u16> = Vec::new();
    for fname in std::env::args().skip(1) {
        let mut glob_ok = false;
        for filename in glob::glob(&fname)? {
            if let Some(filename) = filename?.to_str() {
                append_fname(&mut source, &filename);
                glob_ok = true;
            }
        }
        if !glob_ok {
            append_fname(&mut source, &fname);
        }
    }
    if source.len() <= 0 {
        return Ok(0)
    }
    source.push(0);
    let mut sh_file_op_struct = SHFILEOPSTRUCTW {
        hwnd: HWND(0),
        wFunc: FO_DELETE,
        pFrom: PCWSTR(source.as_mut_ptr()),
        pTo: w!(""),
        fFlags: (FOF_ALLOWUNDO | FOF_NOCONFIRMATION) as u16,
        fAnyOperationsAborted: BOOL(0),
        hNameMappings: 0 as *mut c_void,
        lpszProgressTitle: w!("to trash"),
    };
    unsafe {
        Ok(SHFileOperationW(&mut sh_file_op_struct))
    }
}

fn main() {
    match trash(){
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
        Ok(n) => std::process::exit(n),
    }
}

// References:
//    google: RecycleBin Windows api
//    https://okwave.jp/qa/q2470114.html
//    http://uchukamen.com/Programming1/ToRecycleBin/index.htm
//    https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.SHFILEOPSTRUCTW.html
//    https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/fn.SHFileOperationW.html
