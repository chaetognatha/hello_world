use clap::{App, Arg};

fn main() {
    // Create a new clap app
    let matches = App::new("Interactive hello world cli app")
    // Add a command line argument
    .arg(Arg::with_name("say")
    // Add a help message for the argument
    .help("What do you want to say?")
    // Make the argument required
    .required(true)
    // Specify that it is the first argument
    .index(1))
    // Get the matches
    .get_matches();
    // get the value of the say argument
    let say = matches.value_of("say").unwrap();
    // Print the value
    println!("{}", say);
}
