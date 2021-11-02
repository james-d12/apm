use std::fs;

const NPM_FILES: [&str; 3] = [
    "./node_modules",
    "./package.json",
    "./package-lock.json"
];

pub fn check_is_npm() -> bool {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let file: String = path.unwrap().path().display().to_string();

        if NPM_FILES.contains(&file.as_str()) {
            return true;
        }
    }
    return false;
}