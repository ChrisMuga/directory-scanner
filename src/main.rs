use std::{fs, io};

fn main() -> Result<(), std::io::Error> {
    println!("Directory Listing:\n=======");

    let mut count = 0;

    return scan(".", false, &mut count);
}

// Given directory:
//  - List all the entries for all the non-hidden files
//  - Nest if a directory has more sub-directory
//  - The nesting should be visually pleasing or distinguishing

fn scan(dir: &str, nest: bool, count:&mut usize) -> Result<(), std::io::Error> {
    let ignore_directories: [&str; 1] = [&"node_modules"];

    let entries = fs::read_dir(dir)
                    .unwrap()
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in entries.iter() {

        let path = entry.as_path();
        let absolute_path = fs::canonicalize(&entry).unwrap();

        let entry_name = path 
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap();

        let tab_space = &"\t".repeat(*count);

        let separator = match nest {
            false => "",
            true => tab_space
        };

        println!("{}{}", separator, &entry_name);

        let metadata = fs::metadata(&absolute_path);

        // Check if file is a directory, then call recursively 
        if metadata?.is_dir() {
            let is_dotfile = entry_name
                                .chars()
                                .nth(0)
                                .unwrap() == '.';

            let should_ignore = ignore_directories
                                    .iter()
                                    .any(|&x| x == entry_name);

            if should_ignore || is_dotfile {
                continue;
            }
            
            *count += 1;

            let _ = scan(&absolute_path.to_str().unwrap(), true, count);

            *count -= 1
        }
    }

    Ok(())
}
