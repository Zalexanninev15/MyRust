use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

fn get_input(query: &str) -> std::io::Result<String> {
    println!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;


    Ok(buffer.trim().to_owned())
}

fn organize_dir(dir_path: PathBuf) {
    if !dir_path.exists() {
        println!("Directory \"{}\" does not exist!", dir_path.display());
    }
    else {
        let dir_files = match dir_path.read_dir() {
            Ok(dir_files) => dir_files,
            Err(err) => {
                println!("Error reading directory \"{}\": {}", dir_path.display(), err);
                return
            }
        };

        for file in dir_files {
            if let Ok(file) = file {
                if file.path().is_dir() {
                    println!("Path \"{}\" is a directory!", file.path().display());
                    continue
                }
                if !file.path().display().to_string().ends_with(".directory") {
                    let file_extension = match file.path().extension() {
                        Some(file_extension) => {
                            match file_extension.to_str() {
                                Some(file_extension) => file_extension.to_lowercase(),
                                None => continue
                            }
                        },
                        None => {
                            println!("Can't get file extension for file \"{}\"", file.path().display());
                            continue
                        }
                    };

                    let extention_dir = PathBuf::from(dir_path.join(file_extension));
                    create_dir_if_not_exists(&extention_dir);
                    move_file(&file.path(), &extention_dir.join(file.file_name()));
                }
            }
        }
    }
}

fn move_file(from: &PathBuf, to: &PathBuf) {
    if let Err(err) = fs::rename(from, to) {
        println!("Error moving file \"{}\" to \"{}\": {}", from.display(), to.display(), err);
    }
}

fn create_dir_if_not_exists(dir_path: &PathBuf) {
    if !dir_path.exists() {
        if let Err(err) = fs::create_dir(dir_path) {
            println!("Error creating directory \"{}\": {}", dir_path.display(), err);
        }
    }
}

fn main() {
    loop {
        let dir_path: String = match get_input("Enter a path to organize files: ") {
            Ok(dir_path) => dir_path,
            Err(err) => {
                println!("Error getting user input: {}", err);
                continue
            }
        };

        let now = Instant::now();
        organize_dir(PathBuf::from(dir_path));
        println!("Time to organize: {:?}", now.elapsed().as_secs_f64());
    }
}
