use serde_json::Value;
use std::fs;
use std::path::Path;
use std::io::Write;
use walkdir::WalkDir;

fn main() {
    let directory_path = "E:/projects/yiff_toolkit/ponyxl_loras";
    for entry in WalkDir::new(directory_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap_or_default() == "json" {
            if let Err(e) = format_json_file(&path) {
                println!("Failed to format {}: {}", path.display(), e);
            }
        }
    }
}

fn format_json_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing file: {}", path.display());
    let file_content = fs::read_to_string(&path)?;
    let json: Value = serde_json::from_str(&file_content)?;
    let pretty_json = serde_json::to_string_pretty(&json)?;
    let mut file = fs::File::create(&path)?;
    file.write_all(pretty_json.as_bytes())?;
    println!("Formatted {} successfully.", path.display());
    Ok(())
}
