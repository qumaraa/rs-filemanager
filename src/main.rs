use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::exit;


// constant global variable that contains current version of app
const VERSION: &'static str = "v0.1.1";

struct Fm
{
    command: String,
    path: String,
    data: Option<String>,
}
fn main() {
    // collecting string arguments in Vector of String type
    let args: Vec<String> = args().collect();
    // check if arguments are less than 3 or more than 4, call the `param`
    if args.len() < 3 || args.len() > 4 {
        param();
    }
    // `Fm` struct initializing
    let fm = Fm {
        command: args[1].clone(),
        path: args[2].clone(),
        data: if args.len() > 3 {
            Some(args[3].clone())
        } else {
            None
        },
    };
    // Matching commands
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
            let file = match File::open(&fm.path) {
                Ok(f) => f,
                Err(why) => {
                    println!("[rustfm] Error: {why}");
                    return ();
                }
            };
            let reader = BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                if let Some(data) = &fm.data.clone() {
                    if line.contains(data) {
                        println!(
                            "({}) Found `{}`: {} at line {}",
                            &fm.path,
                            &fm.data.clone().unwrap(),
                            line,
                            i + 1
                        );
                    }
                }
            }
        }
        "fcreate" => {
            let file = match File::create(&fm.path) {
                Ok(f) => f,
                Err(why) => {
                    println!("[rustfm] Error: {why}");
                    return ();
                }
            };
            println!("({}) created successfully!", &fm.path);
        }
        // if the commands do not satisfy many of the commands
        // defined here, then this is an "unknown command" and we call "param`
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
