// use ziphere::{Configs, Formats, _comde::Compress};
// use ziphere::_formats::_sevenz::{SevenzComde, SevenzConfigs};
use clap::{ Parser, Subcommand};

fn main() {
    println!("Hello, world!");

    // if let Formats::SevenZ(zip) = Formats::new_with("zip")
    //     && let Configs::ZipConfig(zipc) = Configs::new_compress("zip")
    // {
    //     let result = zip.compress_file("input", "output", zipc).unwrap();
    //     println!("{}", result);
    // }
    
    ziphere::run_test();

    let app = App::parse();
    
    match app.command {
        Commands::Hello => println!("Hello"),
        _ => println!("Ooops")
    }

}

#[derive(Parser)]
#[command(version, about)]
struct App {

    #[arg(short, long)]
    verbose: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello
}