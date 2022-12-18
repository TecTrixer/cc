use clap::Parser;
use std::fs::File;
use std::io::Read;
mod lexer;
use lexer::Lexer;

/// This is an ANSI C compiler
#[derive(Parser, Debug)]
#[command(author="TecTrixer", version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(default_value_t = String::from("main.c"))]
    file: String,
}

fn main() {
    // Getting cli arguments
    let args = Args::parse();
    let (bytes, len) = match open_file(args.file.clone()) {
        Ok((bytes, len)) => {
            println!("Successfully opened and read file \"{}\"", args.file);
            (bytes, len)
        }
        Err(e) => {
            println!("Could not open file \"{}\", the following error occured: {}", args.file, e);
            return;
        }
    };

    // Tokenizing the input file
    let mut lx = Lexer::new(bytes, 0, len);
    while let Some(token) = lx.get_token() {
        println!("{:?}", token);
    }

}

fn open_file(path: String) -> std::io::Result<(Vec<u8>, usize)> {
    // Trying to open file, failing if it does not exist
    let mut file = File::options()
        .create(false)
        .read(true)
        .write(false)
        .open(path)?;
    // Trying to read file
    let mut buffer = Vec::new();
    let len = file.read_to_end(&mut buffer)?;
    // Returning the byte array and the length of the byte array
    Ok((buffer, len))
}
