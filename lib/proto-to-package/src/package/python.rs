pub struct Python {
    pub template_path: String,
    pub output_path: String,
    pub package_name: String,
}

impl Python {
    pub fn create(&self) -> &String {
        &self.template_path
    }
}