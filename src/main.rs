use regex::Regex;
use std::path::Path;
fn main() {
    // Get first argument: Path to directory
    let args: Vec<String> = std::env::args().collect();
    let obsidian_path = &args[1];
    let target_path = &args[2];

    println!("Watching: {}", obsidian_path);
    println!("Publishing to: {}", target_path);

    // Create a new watcher
    let mut watcher = Watcher::new(obsidian_path.to_string(), target_path.to_string());
    watcher.watch();
}

// Class that handles the file watcher
struct Watcher {
    // The path to the directory to watch
    obsidian_path: String,
    target_path: String,
    // The list of files in the directory
    files: Vec<String>,
}

impl Watcher {
    // Create a new watcher
    fn new(obsidian_path: String, target_path: String) -> Watcher {
        Watcher {
            obsidian_path,
            target_path: target_path,
            files: Vec::new(),
        }
    }

    // Watch function: Checks every minute if there are new files to publish
    fn watch(&mut self) {
        loop {
            self.list_files();
            println!("\n");
            self.copy_files();
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    }

    // List all files in the directory
    fn list_files(&mut self) {
        for entry in walkdir::WalkDir::new(&self.obsidian_path) {
            let entry = entry.unwrap();
            let path = entry.path();
            // check for .md files
            if path.to_str().unwrap().ends_with(".md") {
                self.check_file(path);
            }
        }
    }

    // Checks if a file contains #publish
    fn check_file(&mut self, file: &Path) {
        let contents = std::fs::read_to_string(file).unwrap();
        if contents.to_ascii_lowercase().contains("#publish") {
            // add to files list
            if !self.files.contains(&file.to_str().unwrap().to_string()) {
                self.files.push(file.to_str().unwrap().to_string());
                println!("\nAdded: {}", file.to_str().unwrap());

                // add files that are embedded using ![[file|displayname]] syntax. keep only the file name, not the display name
                let re = Regex::new(r"!\[\[(.*?)\]\]").unwrap();
                for cap in re.captures_iter(&contents) {
                    let new_file = cap[1].to_string();
                    // prepend obsidian_path
                    let new_file = Path::new(&self.obsidian_path).join(&new_file);
                    // check if file is already in list
                    if !self.files.contains(&new_file.to_str().unwrap().to_string()) {
                        // add to files list
                        if !self.files.contains(&new_file.to_str().unwrap().to_string()) {
                            self.files.push(new_file.to_str().unwrap().to_string());
                            println!("Added: {}", new_file.to_str().unwrap());
                        }
                    }
                }
            }
        }
    }
    // copy files to target directory
    fn copy_files(&self) {
        for file in &self.files {
            let source = Path::new(&self.obsidian_path).join(&file);

            // set target path: Replace obsidian_path in file with target_path
            let mut target = Path::new(&self.target_path).join(&file);
            target = Path::new(
                &target
                    .to_str()
                    .unwrap()
                    .replace(&self.obsidian_path, &self.target_path),
            )
            .to_path_buf();

            // copy file, create directories if necessary
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            println!(
                "Copying {} to {}",
                source.to_str().unwrap(),
                target.to_str().unwrap()
            );
            // try copying, if it fails, search for the file in the folder and cor
            std::fs::copy(source, target).unwrap();
        }
    }
}
