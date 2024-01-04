use clap::Parser;

// Parser is a derive macro that generates a struct that implements the clap::Parser trait
// the Parser trait provides a parse method that parses the cli args
// Debug is a derive macro that implements the Debug trait for the struct
// attributes in rust are used to provide metadata about the item they are attached to
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// a struct that defines which cli args are expected
struct Args {
    // attribute that provides info on how to parse the argument
    // in this case you can use the -s or --say flag to pass a string
    #[arg(short, long)]
    say: String,
}

fn main() {
    // parse the cli args
    let args = Args::parse();
    // print the cli args
    println!("You said: {}", args.say)
}