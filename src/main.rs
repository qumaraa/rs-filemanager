use std::env::args;

use std::fs::{
    remove_dir_all, 
    File, 
    OpenOptions
};

use std::io::{
    stdin, 
    stdout, 
    BufRead, 
    BufReader, 
    Read, 
    Write
};

use std::process::exit;

// constant global variable that contains current version of app
const VERSION: &'static str = "v0.1.2@zinc";

struct Fm {
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
            let mut file = match OpenOptions::new().write(true).append(true).open(&fm.path) {
                Ok(f) => f,
                Err(why) => {
                    println!("Failed to open file: {why}");
                    return;
                }
            };

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
                            "\x1B[37;42m({})\x1B[0m \x1B[37;41mFOUND\x1B[0m  \x1B[30;47m`{}`: \x1B[30;47m{}\x1B[0m \x1B[37;42mat line \x1B[0m \x1B[30;47m{}\x1B[0m",
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
        "fremove" => {
            let mut buf = [0u8; 1];
            println!("\x1B[37;41mAre you sure? This operation cannot be undone [y/n]:\x1B[0m");
            stdout().flush().unwrap();
            stdin().read(&mut buf).unwrap();

            let input = buf[0] as char;
            if input != 'y' && input != 'Y' {
                println!("\nAbort");
            } else {
                let file = match remove_dir_all(&fm.path) {
                    Ok(()) => println!("\x1B[37;42mSuccessfully removed!\x1B[0m"),
                    Err(why) => println!("Error: {why}"),
                };
            }
        }
        // if the commands do not satisfy many of the commands
        // defined here, then this is an "unknown command" and we call "param`
        _ => {
            param();
        }
    }
}

fn param() {
    eprintln!("{VERSION}");
    eprintln!("$ Usage:  <command> <path> <data>");
    eprintln!("Commands:\n\tfwrite\n\tfread\n\tffind\n\tfcreate\n\tfremove");

    exit(1); // return error code (1) - `EXIT_FAILURE`
}
