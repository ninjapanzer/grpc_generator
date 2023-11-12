const PYTHON_TEMPLATE_TOML: &str = include_str!("../../res/python/template.pyproject.toml");
const PYTHON_TEMPLATE_README: &str = include_str!("../../res/python/template.README.md");
const PYTHON_TEMPLATE_INIT: &str = include_str!("../../res/python/template.__init__.py");

struct PythonPackage {
    toml: String,
    readme: String,
    init: String,
    some: Option<Vec<String>>,
}

impl Default for PythonPackage {
    fn default() -> Self {
        Self {
            toml: PYTHON_TEMPLATE_TOML.to_string(),
            readme: PYTHON_TEMPLATE_README.to_string(),
            init: PYTHON_TEMPLATE_INIT.to_string(),
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
                }
            }
            None => PythonPackage::default()
        }
    }
}

pub fn create(template_path: Option<String>, package_name: &str, output_path: &str) -> bool {
    let template = PythonPackage::check_template(template_path);

    let package = PythonPackage {
        toml: build_toml(&template, &package_name),
        readme: build_readme(&template, &package_name),
        init: build_init(&template, &package_name),
    };

    write_package(&package, output_path)
}

fn build_toml(template: &PythonPackage, package_name: &str) -> String {
    let kebab_package_name = package_name.replace("_", "-");
    let snake_package_name = package_name.clone();
    let package_name = package_name.replace("_", " ");

    template.toml
        .replace("${kebab_package_name}", &kebab_package_name)
        .replace("${title_package_name}", &package_name)
        .replace("${snake_package_name}", &snake_package_name)
}

fn build_readme(template: &PythonPackage, package_name: &str) -> String {
    let snake_package_name = package_name.clone();
    let package_name = package_name.replace("_", " ");

    template.readme
        .replace("${title_package_name}", &package_name)
        .replace("${snake_package_name}", &snake_package_name)
}

fn build_init(template: &PythonPackage, package_name: &str) -> String {
    let snake_package_name = package_name.clone();
    let package_name = package_name.replace("_", " ");

    template.init
        .replace("${title_package_name}", &package_name)
        .replace("${snake_package_name}", &snake_package_name)
}

fn write_package(package: &PythonPackage, output_path: &str) -> bool {

    true
}
