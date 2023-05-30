use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env::args;
use std::process::exit;


struct Fm {
    command: String,
    path:    String,
    data:    String,
}


fn main()  {
    let args: Vec<String> = args().collect();

    if args.len() < 4 {
        param();
    }


    let mut fm = Fm {
        command: args[1].clone(),
        path: args[2].clone(),
        data: args[3].clone(),
    };

    match fm.command.as_str() {
        "fread" => {
            println!("Ok: fread");
        }
        "fwrite" => {
            println!("Ok: fwrite");
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
            eprintln!("Unknown command.");
            param();
        }
    }

}

fn param() {
    eprintln!("$ Usage:  <command> <path> <data>");
    eprintln!("Commands:\n\tfwrite\n\tfread\n\tffind\n\tfcreate\n\tfremove");
    exit(1);
}