extern crate glob;

use std::io::{self, BufRead};
use trash::TrashBox;

fn trash_from_reader(
    r: &mut impl std::io::Read,
    tbox: &mut TrashBox,
) -> Result<(), std::io::Error> {
    let br = io::BufReader::new(r);
    for line in br.lines() {
        let line = match line {
            Err(err) => return Err(err),
            Ok(line) => line,
        };
        println!("{}", &line);
        tbox.add(&line);
    }
    Ok(())
}

fn trash() -> Result<i32, Box<dyn std::error::Error>> {
    let mut trashbox = TrashBox::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg1) = args.next() {
        if arg1 == "-from-file" {
            match args.next() {
                Some(filename) => {
                    if filename == "-" {
                        trash_from_reader(&mut std::io::stdin(), &mut trashbox)?;
                    } else {
                        trash_from_reader(&mut std::fs::File::open(filename)?, &mut trashbox)?;
                    }
                }
                None => return Err(format!("Too few arguments for -from-file").into()),
            }
        } else {
            let mut done = false;
            for filename in glob::glob(&arg1)? {
                if let Some(filename) = filename?.to_str() {
                    println!("{}", arg1);
                    trashbox.add(&filename);
                    done = true;
                }
            }
            if !done {
                return Err(format!("no matches found for pattern: {}", arg1).into());
            }
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
