use std::process::Command;
use clap::{ Parser, Subcommand };
use clap::parser::ValueSource::CommandLine;
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
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// does testing things
    Generate {
        /// Path to .proto files
        // #[arg(short, long, default_value=".")]
        path: String,
        /// Path to includes
        #[arg(short, long, default_value="./includes")]
        includes: String,
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
        Some(GenerateCommands::Generate { ruby, python, javascript, path, .. }) => {
            let mut ran_any_command = false;
            let mut args = vec![];

            if *ruby {
                println!("printing ruby lists...");
                Command::new ("mkdir")
                    .args(["-p", "../../../../lang/ruby/rbi"])
                    .output()
                    .expect("failed to execute process");
                let output = Command::new ("which")
                    .arg("grpc_tools_ruby_protoc_plugin")
                    .output()
                    .expect("GRPC Tools Ruby plugin not found");
                args.push("--plugin=protoc-gen-grpc=".to_owned() + String::from_utf8_lossy(&output.stdout).trim_end());
                args.push("--ruby_out=../../../../lang/ruby".to_string());
                args.push("--grpc_out=../../../../lang/ruby".to_string());
                args.push("--rbi_out=../../../../lang/ruby/rbi".to_string());
                println!("Standard Output: {}", String::from_utf8_lossy(&output.stdout));
                println!("Standard Error: {}", String::from_utf8_lossy(&output.stderr));
                // Command::new("./rscript.sh").status().expect("Failed to run the Ruby script");
                // Command::new("./rscript.sh").status().expect("Failed to run the Ruby script");
                ran_any_command = true;
            }

            if *python {
                let pedantic_plugin_location = Command::new ("which")
                    .arg("protoc-gen-protobuf-to-pydantic")
                    .output()
                    .expect("Pydantic plugin not found");
                Command::new ("mkdir")
                    .args(["-p", "../../../../lang/python"])
                    .output()
                    .expect("failed to execute process");
                println!("printing python lists...");
                args.push("--plugin=protoc-gen-protobuf-to-pydantic=".to_owned() + String::from_utf8_lossy(&pedantic_plugin_location.stdout).trim_end());
                args.push("--python_out=../../../../lang/python".to_string());
                args.push("--pyi_out=../../../../lang/python".to_string());
                args.push("--protobuf-to-pydantic_out=../../../../lang/python".to_string());
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
            } else {
                let output = Command::new("protoc")
                    .args(args)
                    .args([
                        "--proto_path=../../../../",
                        path
                    ]).output()
                    .expect("failed to execute process");
                println!("Standard Output: {}", String::from_utf8_lossy(&output.stdout));
                println!("Standard Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        _ => {}
    }
}

