use std::process::Command;
use clap::{ Parser, Subcommand };
// use std::path::PathBuf;
// mod python
// mod ruby
// mod javascript

// use git_events::GitEvents;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Generate Proto Classes
    #[command(subcommand)]
    generate: Option<GenerateCommands>,
    /// Path to .proto files
    #[arg(short, long, default_value=".")]
    path: String,
    /// Path to includes
    #[arg(short, long, default_value="md")]
    includes: String,
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// does testing things
    Generate {
        /// Generate Ruby Proto Classes
        #[arg(short, long)]
        ruby: bool,
        /// Generate Python Proto Classes
        #[arg(short, long)]
        python: bool,
        /// Generate JavaScript Proto Classes
        #[arg(short, long)]
        javascript: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.generate {
        Some(GenerateCommands::Generate { ruby, python, javascript }) => {
            let mut ran_any_command = false;

            if *ruby {
                println!("printing ruby lists...");
                // Command::new("./rscript.sh").status().expect("Failed to run the Ruby script");
                ran_any_command = true;
            }

            if *python {
                println!("printing python lists...");
                // Command::new("./pscript.sh").status().expect("Failed to run the Python script");
                ran_any_command = true;
            }

            if *javascript {
                println!("printing javascript lists...");
                // Command::new("./jscript.sh").status().expect("Failed to run the JavaScript script");
                ran_any_command = true;
            }

            if !ran_any_command {
                println!("Not printing testing lists...");
            }
        }
        _ => {}
    }
}

