use std::fs;
use std::path::Path;

pub fn execute() {
    println!("executed!");
}

pub fn auto_binary_file_alias_generator() -> std::io::Result<()> {
    let binary_dir = "./src/bin";
    let binary_dir_path = Path::new(binary_dir);

    if !binary_dir_path.exists() {
        println!("Error: '{binary_dir}' directory not found!");
        return Ok(());
    }

    let mut alias_content = String::from("[alias]\n");
    let dirs = fs::read_dir(binary_dir_path)?;

    for dir in dirs {
        let dir = dir?;
        let path = dir.path();

        if !path.is_file() {
            continue;
        }

        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }

        let filename = match path.file_stem().and_then(|f| f.to_str()) {
            Some(fstem) => fstem,
            _ => continue,
        };

        alias_content.push_str(&format!("{} = \"run --bin {}\"\n", filename, filename));
    }

    fs::create_dir_all(".cargo")?;
    fs::write(".cargo/config.toml", &alias_content)?;

    println!("Updated as\n{}", &alias_content);
    Ok(())
}
