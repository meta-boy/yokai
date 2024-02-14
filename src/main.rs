mod bypass;
mod network;
mod util;
mod encryption;
mod config;

use clap::{arg, Parser};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the config file
    #[arg(short, long)]
    file_name: String,

}

fn main() {
    let args = Args::parse();

    match config::parser::parse_config(args.file_name) {
        Ok(config) => {
            println!("Successfully parsed config: {:?}", config);
            // Your application logic here, using the parsed config
        },
        Err(e) => {
            eprintln!("Failed to parse config: {}", e);
        },
    }
}
