use crate::package::python_parser;
use std::fs::create_dir_all;
use std::path::PathBuf;

const PYTHON_TEMPLATE_TOML: &str = include_str!("../../res/python/template.pyproject.toml");
const PYTHON_TEMPLATE_README: &str = include_str!("../../res/python/template.README.md");
const PYTHON_TEMPLATE_INIT: &str = include_str!("../../res/python/template.__init__.py");

struct PythonPackage {
    toml: String,
    readme: String,
    init: String,
    files: Option<Vec<PathBuf>>,
}

impl Default for PythonPackage {
    fn default() -> Self {
        Self {
            toml: PYTHON_TEMPLATE_TOML.to_string(),
            readme: PYTHON_TEMPLATE_README.to_string(),
            init: PYTHON_TEMPLATE_INIT.to_string(),
            files: None,
        }
    }
}

impl PythonPackage {
    pub fn check_template(template_path: Option<String>) -> PythonPackage {
        match template_path {
            Some(path) => {
                let toml_path = format!("{}/template.pyproject.toml", path);
                let readme_path = format!("{}/template.README.md", path);
                let init_path = format!("{}/template.__init__.py", path);

                PythonPackage {
                    toml: std::fs::read_to_string(toml_path).unwrap_or_default(),
                    readme: std::fs::read_to_string(readme_path).unwrap_or_default(),
                    init: std::fs::read_to_string(init_path).unwrap_or_default(),
                    ..Default::default()
                }
            }
            None => PythonPackage::default(),
        }
    }
}

pub fn create(
    template_path: Option<String>,
    package_name: &str,
    output_path: &str,
    generated_files: Vec<PathBuf>,
) -> bool {
    let template = PythonPackage::check_template(template_path);

    let package = PythonPackage {
        toml: build_toml(&template, &package_name),
        readme: build_readme(&template, &package_name),
        init: build_init(&template, &generated_files, &package_name),
        files: Some(generated_files),
    };

    write_package(&package, &package_name, output_path)
}

fn build_toml(template: &PythonPackage, package_name: &str) -> String {
    let kebab_package_name = package_name.replace("_", "-");
    let snake_package_name = package_name.clone();
    let package_name = package_name.replace("_", " ");

    template
        .toml
        .replace("${kebab_package_name}", &kebab_package_name)
        .replace("${title_package_name}", &package_name)
        .replace("${snake_package_name}", &snake_package_name)
}

fn build_readme(template: &PythonPackage, package_name: &str) -> String {
    let snake_package_name = package_name.clone();
    let package_name = package_name.replace("_", " ");

    template
        .readme
        .replace("${title_package_name}", &package_name)
        .replace("${snake_package_name}", &snake_package_name)
}

fn build_init(template: &PythonPackage, files: &Vec<PathBuf>, package_name: &str) -> String {
    let mut imports: Vec<String> = vec![];
    // from .GreatThingRequest_p2p import GreatThingProperties, GreatThingRequest

    files.iter().for_each(|f| {
        if f.file_name()
            .unwrap()
            .to_string_lossy()
            .ends_with("_grpc.py")
        {
            imports.push(create_import(&f));
        }
        if f.file_name()
            .unwrap()
            .to_string_lossy()
            .ends_with("_p2p.py")
        {
            imports.push(create_import(&f));
        }
    });

    imports.sort_by_key(|s| (!s.to_lowercase().contains("grpc"), s.clone()));

    template
        .init
        .replace("${imports}", imports.join("\n").as_str())
}

fn create_import(file: &PathBuf) -> String {
    let file_name = file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .replace(".py", "");
    let classes = python_parser::Parser::collect_classes(&file)
        .classes
        .join(", ");
    format!("from .{} import {}", file_name, classes)
}

fn write_package(package: &PythonPackage, package_name: &str, output_path: &str) -> bool {
    println!(
        "Writing Python Package: {} to {}/{}",
        package_name, output_path, package_name
    );
    create_dir_all(format!("{}/{}", output_path, package_name))
        .expect("failed to create directory");
    std::fs::write(
        format!("{}/{}.toml", output_path, package_name),
        package.toml.as_bytes(),
    )
    .expect("failed to write toml file");
    std::fs::write(
        format!("{}/README.md", output_path),
        package.readme.as_bytes(),
    )
    .expect("failed to write readme file");
    std::fs::write(
        format!("{}/{}/__init__.py", output_path, package_name),
        package.init.as_bytes(),
    )
    .expect("failed to write init file");
    package.files.as_ref().unwrap().iter().for_each(|file| {
        std::fs::copy(
            file,
            format!(
                "{}/{}/{}",
                output_path,
                package_name,
                file.file_name().unwrap().to_string_lossy()
            ),
        )
        .expect(format!("failed to copy file: {} to {}", output_path, package_name).as_str());
    });
    true
}
