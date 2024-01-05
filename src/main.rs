use clap::Parser;
use std::process::Command;

// below are macros, a fast and convenient way to avoid boilerplate
// I think of them a bit like python decorators
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// a struct that defines which cli args are expected
struct Args {
    // attribute that provides info on how to parse the argument
    // in this case you can use the -s or --say flag to pass a string
    #[arg(short, long)]
    say: Option<String>,
    #[arg(short, long)]
    list: Option<bool>,
}

fn main() {
    // parse the cli args
    let args = Args::parse();
    
    // check if say is set 
    if let Some(say) = args.say {
        // if say is set, print the value
        println!("You said: {}", say);
    } else if args.list.is_some() {
        let result = Command::new("ls")
            .arg("-l")
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&result.stdout));
    }
    }
    
