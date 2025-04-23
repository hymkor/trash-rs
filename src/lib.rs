use core::ffi::c_void;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::UI::Shell::{
    SHFileOperationW, FOF_ALLOWUNDO, FOF_NOCONFIRMATION, FO_DELETE, SHFILEOPSTRUCTW,
};

pub struct TrashBox {
    buffer: Vec<u16>,
}

impl TrashBox {
    pub fn new() -> TrashBox {
        TrashBox { buffer: Vec::new() }
    }
    pub fn add(&mut self, fname: &str) {
        for c in fname.encode_utf16() {
            self.buffer.push(c);
        }
        self.buffer.push(0);
    }
    pub fn throw(&mut self) -> i32 {
        if self.buffer.len() <= 0 {
            return 0;
        }
        self.buffer.push(0);
        let mut sh_file_op_struct = SHFILEOPSTRUCTW {
            hwnd: HWND(0),
            wFunc: FO_DELETE,
            pFrom: PCWSTR(self.buffer.as_ptr()),
            pTo: w!(""),
            fFlags: (FOF_ALLOWUNDO | FOF_NOCONFIRMATION) as u16,
            fAnyOperationsAborted: BOOL(0),
            hNameMappings: 0 as *mut c_void,
            lpszProgressTitle: w!("to trash"),
        };
        unsafe { SHFileOperationW(&mut sh_file_op_struct) }
    }
}
