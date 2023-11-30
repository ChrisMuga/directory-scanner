use std::{fs, io};

fn main() -> Result<(), std::io::Error> {
    println!("Directory Listing");
    return scan(".", false);
}

// Given directory:
//  - List all the entries for all the non-hidden files
//  - Nest if a directory has more sub-directory
//  - The nesting should be visually pleasing or distinguishing

fn scan(dir: &str, nest: bool) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(dir)?
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

        let separator = match nest {
            false => "",
            true => "\t"
        };

        println!("{}{}", separator, &entry_name);

        let metadata = fs::metadata(&absolute_path);

        // Check if file is a directory, then call recursively 
        if metadata?.is_dir() {
            let is_hidden = entry_name.chars().nth(0).unwrap() == '.';
            if !is_hidden {
                println!("\t ===> {}", entry_name);
                let _ = scan(&absolute_path.to_str().unwrap(), true);
            }
        }
    }

    Ok(())
}
