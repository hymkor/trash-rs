extern crate glob;

use trash::TrashBox;

fn trash() -> Result<i32, Box<dyn std::error::Error>> {
    let mut trashbox = TrashBox::new();
    for fname in std::env::args().skip(1) {
        let mut done = false;
        for filename in glob::glob(&fname)? {
            if let Some(filename) = filename?.to_str() {
                println!("{}", fname);
                trashbox.add(&filename);
                done = true;
            }
        }
        if !done {
            return Err(format!("no matches found for pattern: {}", fname).into());
        }
    }
    Ok(trashbox.throw())
}

fn main() {
    match trash() {
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
