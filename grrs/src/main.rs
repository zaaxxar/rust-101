use clap::Parser;       
// since we want to derive a certain class from a library, we will specifiy the class to be called here ^ and then we will use #[derive(Parser)] right before we plan to use it.

/// search for a pattern in a file and display the lines that contain it. 
#[derive(Parser)] 
// i guess this means we are calling the parser class from library: Clap. 
struct Cli{
    pattern: String, 
    path: std::path::PathBuf, 
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("Could not read file.");

    for line in content.lines(){
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
