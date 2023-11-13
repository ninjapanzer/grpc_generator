use std::path::PathBuf;

struct RubyPackage {
    gemspec: String,
    readme: String,
    loader: String,
    files: Option<Vec<PathBuf>>,
}

impl Default for RubyPackage {
    fn default() -> Self {
        Self {
            gemspec: "".parse().unwrap(),
            readme: "".parse().unwrap(),
            loader: "".parse().unwrap(),
            files: None,
        }
    }
}

impl RubyPackage {
    pub fn check_template(template_path: Option<String>) -> RubyPackage {
        match template_path {
            Some(path) => {
                // let toml_path = format!("{}/template.pyproject.toml", path);
                // let readme_path = format!("{}/template.README.md", path);
                // let init_path = format!("{}/template.__init__.py", path);
                //
                // RubyPackage {
                //     toml: std::fs::read_to_string(toml_path).unwrap_or_default(),
                //     readme: std::fs::read_to_string(readme_path).unwrap_or_default(),
                //     init: std::fs::read_to_string(init_path).unwrap_or_default(),
                //     .. Default::default()
                // }
                RubyPackage::default()
            }
            None => RubyPackage::default()
        }
    }
}

pub fn create(template_path: Option<String>, package_name: &str, output_path: &str, generated_files: Vec<PathBuf>) -> bool {
    let template = RubyPackage::check_template(template_path);

    let package = RubyPackage {
        gemspec: build_gemspec(&template, &package_name),
        readme: build_readme(&template, &package_name),
        loader: build_loader(&template, &generated_files),
        files: Some(generated_files),
    };

    write_package(&package, &package_name, output_path)
}

fn build_gemspec(package: &RubyPackage, package_name: &str) -> String {
    todo!()
}

fn build_readme(package: &RubyPackage, package_name: &str) -> String {
    todo!()
}

fn build_loader(package: &RubyPackage, files: &Vec<PathBuf>) -> String {
    todo!()
}

fn write_package(package: &RubyPackage, package_name: &str, output_path: &str) -> bool {
    todo!()
}