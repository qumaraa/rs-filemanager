use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::process::exit;

struct Fm {
    command: String,
    path: String,
    data: Option<String>,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 || args.len() > 4 {
        param();
    }
    let mut fm = Fm {
        command: args[1].clone(),
        path: args[2].clone(),
        data: if args.len() > 3 {
            Some(args[3].clone())
        } else {
            None
        },
    };
    match fm.command.as_str() {
        "fread" => {
            let mut fs: File = match File::open(&fm.path) {
                Ok(f) => f,
                Err(why) => {
                    println!("[rustfm] Error: {why}");

                    return ();
                }
            };
            let mut buf = String::new();
            match fs.read_to_string(&mut buf) {
                Ok(i) => i,
                Err(why) => {
                    println!("[rustfm] Error: {why}");
                    return ();
                }
            };
            println!("{buf}");
        }
        "fwrite" => {
            // using `unwrap()` while testing.
            // in future, of course, we'll handle the Errors

            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&fm.path)
                .unwrap();

            if let Some(data) = fm.data {
                file.write_all(data.as_bytes()).unwrap();
            }
            println!(
                "[rustfm] Data has been successfully written in {}",
                &fm.path
            );
        }
        "ffind" => {
            println!("Ok: ffind");
        }
        "fcreate" => {
            println!("Ok: fcreate");
        }
        "fremove" => {
            println!("Ok: fremove");
        }
        _ => {
            param();
        }
    }
}

fn param() {
    eprintln!("$ Usage:  <command> <path> <data>");
    eprintln!("Commands:\n\tfwrite\n\tfread\n\tffind\n\tfcreate\n\tfremove");
    exit(1);
}
