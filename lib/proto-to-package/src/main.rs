use clap::{Parser, Subcommand};
use package::python as PackagePython;
use pathsearch::find_executable_in_path;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

mod package;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProtoToPackage {
    /// Generate Proto Classes
    #[command(subcommand)]
    commands: Option<Commands>,
    /// Path to output
    #[arg(short, long, default_value = "./artifacts/lang")]
    output: String,
    /// Path to includes
    #[arg(short, long, default_value = "./include_protos")]
    includes: String,
}

#[derive(Subcommand)]
enum Commands {
    Clear {},
    //@ Generate Artifacts from Protos
    /// # Example
    /// ./proto-to-package --output ../../../../artifacts/lang --includes ../../../../api-common-protos-main generate ../../../../*.proto --python --ruby --oas
    Generate {
        /// Path to .proto files
        #[arg(default_value = "./*.proto")]
        path: String,
        /// Generate Ruby Proto Classes
        #[arg(short, long)]
        ruby: bool,
        /// Generate with Ruby Experimental Features
        #[arg(long, default_value = "false")]
        ruby_experimental: bool,
        /// Generate Python Proto Classes
        #[arg(short, long)]
        python: bool,
        /// Generate JavaScript Proto Classes
        #[arg(short, long)]
        javascript: bool,
        /// Generate OASv3 YAML
        #[arg(short, long)]
        oas: bool,
    },
    /// Create Packages from Artifacts
    Package {
        #[arg()]
        package: String,
        #[arg()]
        artifacts: String,
        /// Generate Ruby Proto Gem
        #[arg(short, long)]
        ruby: bool,
        /// Generate Python Whl
        #[arg(short, long)]
        python: bool,
        /// Generate JavaScript Package
        #[arg(short, long)]
        javascript: bool,
    },
    Publish {
        #[arg()]
        version: String,
        /// Publish Ruby Gem To GHPackages
        #[arg(short, long)]
        ruby: bool,
        /// Publish Python Whl to S3
        #[arg(short, long)]
        python: bool,
        /// Publish Package to GHPackages
        #[arg(short, long)]
        javascript: bool,
    },
}

fn find_plugin(exe: &str) -> Result<String, &'static str> {
    match find_executable_in_path(exe) {
        Some(path) => Ok(path.to_str().unwrap().trim_end().to_string()),
        None => Err("failed to find plugin"),
    }
}

/// Exposes a convenience CLI to generate code from Protobuf files.
///
/// # Commands
///
/// * `generate` - used to generate code from Protobuf files.
/// * `clear` - used to clear the output directory.
///
/// # Generate
/// ## Arguments
/// * `path` - Path to the Protobuf files.
/// * `output` - Path to the output directory.
/// * `includes` - Path to the includes directory.
/// * `ruby` - Generate Ruby code.
/// * `python` - Generate Python code.
/// * `javascript` - Generate JavaScript code.
/// * `oas` - Generate OASv3 YAML.
///
/// ```
/// let result = my_crate::utils::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Errors
///
/// This function doesn't produce any errors.
fn main() {
    let cli = ProtoToPackage::parse();

    match &cli.commands {
        Some(Commands::Package {
            python,
            package,
            artifacts,
            ..
        }) => {
            let mut files: Vec<PathBuf> = Vec::new();
            for entry in WalkDir::new(artifacts).into_iter().filter_map(|e| e.ok()) {
                let entry = entry.path();
                if entry.is_file() {
                    files.push(entry.to_path_buf());
                };
            }
            if *python {
                // Does nothing but getting this stubbed out
                PackagePython::create(None, package, &cli.output.clone(), files.clone());
            }
        }
        _ => {}
    }

    match &cli.commands {
        Some(Commands::Clear { .. }) => {
            remove_dir_all(cli.output.clone()).expect("failed to remove directory");
        }
        _ => {}
    }
    match &cli.commands {
        Some(Commands::Generate {
            ruby,
            ruby_experimental,
            python,
            javascript,
            path,
            oas,
            ..
        }) => {
            let mut ran_any_command = false;
            let mut args = vec![];

            if *ruby {
                let output_path = cli.output.clone();
                let ruby_output_path = output_path + "/ruby";
                let rbi_output_path = String::from(&ruby_output_path) + "/rbi";
                create_dir_all(&rbi_output_path).expect("failed to create directory");

                let plugin_path = find_plugin("grpc_tools_ruby_protoc_plugin")
                    .expect("GRPC Tools Ruby plugin not found");
                args.push(format!(
                    "{}{}",
                    "--plugin=protoc-gen-grpc_ruby=", plugin_path
                ));
                args.push(format!("{}{}", "--grpc_ruby_out=", &ruby_output_path));

                if *ruby_experimental {
                    let rbs_plugin_path =
                        find_plugin("protoc-gen-rbs").expect("RBS Ruby plugin not found");
                    args.push(format!("{}{}", "--plugin=protoc-gen-rbs=", rbs_plugin_path));
                    args.push(format!("{}{}", "--rbs_out=", &ruby_output_path));
                }

                args.push(format!("{}{}", "--ruby_out=", &ruby_output_path));
                args.push(format!("{}{}", "--rbi_out=", &rbi_output_path));
                ran_any_command = true;
            }

            if *python {
                let python_output_path = cli.output.clone() + "/python";
                create_dir_all(&python_output_path).expect("failed to create directory");

                let pedantic_plugin_location = find_plugin("protoc-gen-protobuf-to-pydantic")
                    .expect("GRPC Tools Python plugin not found");
                let grpcio_plugin_path = find_plugin("protoc-gen-grpclib_python")
                    .expect("GRPC Tools Python plugin not found");

                args.push(format!(
                    "{}{}",
                    "--plugin=protoc-gen-protobuf-to-pydantic=", pedantic_plugin_location
                ));
                args.push(format!(
                    "{}{}",
                    "--plugin=protoc-gen-grpc_python=", grpcio_plugin_path
                ));
                args.push(format!("{}{}", "--grpc_python_out=", &python_output_path));
                args.push(format!("{}{}", "--python_out=", &python_output_path));
                args.push(format!("{}{}", "--pyi_out=", &python_output_path));
                args.push(format!(
                    "{}{}",
                    "--protobuf-to-pydantic_out=", &python_output_path
                ));
                ran_any_command = true;
            }

            if *javascript {
                println!("printing javascript lists...");
                ran_any_command = true;
            }

            if *oas {
                let oas_output_path = cli.output.clone() + "/oas";
                create_dir_all(&oas_output_path).expect("failed to create directory");

                let oas_plugin_location =
                    find_plugin("protoc-gen-oas").expect("GRPC Tools Python plugin not found");

                args.push(format!(
                    "{}{}",
                    "--plugin=protoc-gen-oas=", oas_plugin_location
                ));
                args.push(format!("{}{}", "--oas_out=", &oas_output_path));
                ran_any_command = true;
            }

            if !ran_any_command {
                println!("Not printing testing lists...");
            } else {
                let includes_dir = cli.includes.clone();
                let source_dir = Path::new(path)
                    .parent()
                    .unwrap()
                    .to_str()
                    .expect("failed to get parent path");
                let output = Command::new("protoc")
                    .args(args)
                    .args([
                        // Path for external includes
                        format!("{}{}", "--proto_path=", &includes_dir),
                        // Path enclosing the source proto file
                        format!("{}{}", "--proto_path=", &source_dir),
                        // Path to target proto file
                        path.to_string(),
                    ])
                    .output()
                    .expect("failed to execute process");
                println!("status: {}", output.status);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stdout: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        _ => {}
    }
}
