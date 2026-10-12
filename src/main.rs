use std::io::{self, Write};
use std::time::Duration;

use clap::{Command, arg, command, value_parser};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("steps")
                .arg(
                    arg!(<steps>)
                    .value_parser(value_parser!(i32))
                )
                .about("Steps to 100%")
        )
        .get_matches();

    match matches.subcommand(){
        Some(("steps", sub_m)) => {
            let val = sub_m.get_one::<i32>("steps");
            progress(*val.unwrap());
        }
        _ => {println!("Please use it like 'cargo run -- steps 12'")}
    }
}

fn progress(num: i32){ // one step is 500ms
   for step in 0..=num{
        let empty = num - step;
        let filled = step;

        print!("\r[");
        for _ in 0..filled {print!("█")};
        for _ in 0..empty {print!("░")};
        print!("] {}%", (step * 100)/num);
        io::stdout().flush().unwrap();  
        std::thread::sleep(Duration::from_millis(100));
   } 
   println!()
}