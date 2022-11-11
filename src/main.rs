use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let valid_sections: [&str; 3] = ["Unit", "Service", "Install"];
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();

    let path = std::env::args().nth(1);

    if path.is_none() {
        panic!("{}", "no path given");
    }

    let path_ref = path.as_ref().unwrap();

    let file_metadata = fs::symlink_metadata(path_ref)?;
    let file_type = file_metadata.file_type();

    let mut read_path: String = path_ref.to_string();

    if file_type.is_symlink() {
        read_path = fs::canonicalize(path_ref).unwrap().into_os_string().into_string().unwrap();
    }

    if ! Path::new(&read_path).exists() {
        panic!("file does not exist");
    }

    let file = File::open(&read_path)?;
    let reader = BufReader::new(file);

    let section_re = Regex::new(r"^\[(.*)\]").unwrap();
    let comment_re = Regex::new(r"^#").unwrap();

    let mut current_section = String::from("");

    for mut line in reader.lines() {
        let line_ref = &line.as_ref().unwrap();

        if section_re.is_match(line_ref) {
            let section = section_re.captures(line_ref).unwrap();
            if valid_sections.contains(&&section[1]) {
                current_section = String::from(&section[1]);
            }
            
        } else if comment_re.is_match(line_ref) || line_ref.is_empty() {
            continue;
        } else {

            let mut line_split = line.as_mut().unwrap().split('=');

            let systemd_key = line_split.next().unwrap().to_string();
            let systemd_val = line_split.next().unwrap().to_string();

            let mut hashmap_tmp: HashMap<String, String> = HashMap::new();

            hashmap_tmp.insert(systemd_key, systemd_val);

            if result.contains_key(&current_section) {
                let mut update_section = result.get_mut(&current_section).expect("Unexpected Error").clone();
                update_section.push(hashmap_tmp);
                result.insert(current_section.clone(), update_section);
            } else {
                let vector_tmp = vec![hashmap_tmp];
                result.insert(current_section.clone(), vector_tmp);
            }

        }
    }

    println!("{:#?}", &result);

    Ok(())

}
