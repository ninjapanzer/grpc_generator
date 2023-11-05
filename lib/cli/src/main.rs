use std::fs::create_dir_all;
use std::process::Command;
use clap::{ Parser, Subcommand };
use pathsearch::find_executable_in_path;

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

fn find_plugin(exe: &str) -> Result<String, &'static str> {
    match find_executable_in_path(exe) {
        Some(path) => Ok(path.to_str().unwrap().to_string()),
        None => Err("failed to find plugin"),
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.generate {
        Some(GenerateCommands::Generate { ruby, python, javascript, path, .. }) => {
            let mut ran_any_command = false;
            let mut args = vec![];

            if *ruby {
                println!("printing ruby lists...");
                create_dir_all("../../../../lang/ruby/rbi")
                    .expect("failed to create directory");
                let plugin_path = find_plugin("grpc_tools_ruby_protoc_plugin")
                    .expect("GRPC Tools Ruby plugin not found");

                args.push("--plugin=protoc-gen-grpc=".to_owned() + plugin_path.as_str());
                args.push("--ruby_out=../../../../lang/ruby".to_string());
                args.push("--grpc_out=../../../../lang/ruby".to_string());
                args.push("--rbi_out=../../../../lang/ruby/rbi".to_string());
                ran_any_command = true;
            }

            if *python {
                create_dir_all("../../../../lang/python")
                    .expect("failed to create directory");
                let pedantic_plugin_location = find_plugin("protoc-gen-protobuf-to-pydantic")
                    .expect("GRPC Tools Python plugin not found");
                println!("printing python lists...");
                args.push("--plugin=protoc-gen-protobuf-to-pydantic=".to_owned() + pedantic_plugin_location.trim_end());
                args.push("--python_out=../../../../lang/python".to_string());
                args.push("--pyi_out=../../../../lang/python".to_string());
                args.push("--protobuf-to-pydantic_out=../../../../lang/python".to_string());
                ran_any_command = true;
            }

            if *javascript {
                println!("printing javascript lists...");
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
            }
        }
        _ => {}
    }
}

