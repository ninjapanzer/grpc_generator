## Python Structure (Whl)
    output_dir/
    ├── ${project}.toml
    ├── README.md
    └── ${project}/
        ├── __init__.py
        └── ... (grpc stubs, messages, and pydantic interfaces)
## Ruby Structure (Gem)
    output_dir/
    ├── ${project}.gemspec
    ├── README.md
    └── ${project}/
        ├── ${project}.rb
        └── ... (grpc stubs, messages, and sorbet interfaces)
## OAS Structure (ZIP)
    output_dir/
    ├── README.md
    └── ${project}/
        └── ... (OASv3 API YAML)
    
