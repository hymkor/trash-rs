use core::ffi::c_void;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::UI::Shell::{SHFileOperationW, SHFILEOPSTRUCTW};

const FO_DELETE: u32 = 0x3;
const FOF_ALLOWUNDO: u16 = 0x40;
const FOF_NOCONFIRMATION: u16 = 0x10;

fn main() {
    let mut source: Vec<u16> = Vec::new();
    for fname in std::env::args().skip(1) {
        let mut fname_vec: Vec<u16> = fname.encode_utf16().collect();
        source.append(&mut fname_vec);
        source.push(0);
    }
    if source.len() > 0 {
        source.push(0);
        let mut sh_file_op_struct = SHFILEOPSTRUCTW {
            hwnd: HWND(0),
            wFunc: FO_DELETE,
            pFrom: PCWSTR(source.as_mut_ptr()),
            pTo: w!(""),
            fFlags: FOF_ALLOWUNDO | FOF_NOCONFIRMATION,
            fAnyOperationsAborted: BOOL(0),
            hNameMappings: 0 as *mut c_void,
            lpszProgressTitle: w!("to trash"),
        };
        unsafe {
            let _ = SHFileOperationW(&mut sh_file_op_struct);
        }
    }
}

// References:
//    google: RecycleBin Windows api
//    https://okwave.jp/qa/q2470114.html
//    http://uchukamen.com/Programming1/ToRecycleBin/index.htm
//    https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.SHFILEOPSTRUCTW.html
//    https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/fn.SHFileOperationW.html
