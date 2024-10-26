use std::fs;
use std::fs::metadata;
use std::path::Path;

fn get_name(path: &Path)-> &str{
    path.file_name().unwrap().to_str().unwrap()
}

fn scandir(path: &str, target: &String, type_find: &String){
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {

                    match entry {
                        Ok(dir_entry) => {
                            let path = dir_entry.path();
                            match fs::metadata(&path) {
                                Ok(metadata) => {
                                    println!("{:?}",path.display());
                                    if path.file_name().unwrap().to_str().unwrap() == "AppData"{continue}else {
                                        if metadata.is_dir() {
                                            // println!("dir: {}", path.display());
                                            // Рекурсивный вызов для подкаталога
                                            if path.file_name().unwrap().to_str().unwrap() == target{
                                                println!("file: {}| {}", path.display(), path.display());
                                                break
                                            }else{
                                                scandir(path.to_str().unwrap(), target, type_find);
                                            }

                                        } else {
                                            if path.file_name().unwrap().to_str().unwrap() == target{
                                                print!("12");
                                                print!("file {}", path.file_name().unwrap().to_str().unwrap());
                                                println!("file: {}| {}", path.display(), path.display());
                                                break
                                            }
                                        }
                                    }

                                }
                                Err(e) => {
                                    eprintln!("Error accessing metadata for {:?}: {:?}", path, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading entry: {:?}", e);
                        }
                    }
                }

            }

        Err(e) => {
            eprintln!("Failed to read directory {:?}: {:?}", path, e);
        }
    }
}

fn main() {
    let x: &str = r"C:\";
    scandir(&x, &"pubg".to_string(), &"folder".to_string());
}
