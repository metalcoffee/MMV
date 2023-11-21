use std::fs;
use regex::Regex;

/// Wildcard to Regex Pattern
///
/// Converts a wildcard pattern to a regular expression pattern.
///
/// The function takes a wildcard pattern as input and returns a regular expression pattern that
/// matches the same set of strings as the wildcard.
///
/// # Arguments
///
/// * `wildcard` - A wildcard pattern to be converted to a regular expression pattern.
///
/// # Returns
///
/// A regular expression pattern as a `String`.
///
/// # Example
///
/// ```no
/// let wildcard = "path/to/some_*.txt";
/// let regex_pattern = wildcard_to_regex_pattern(wildcard);
/// ```
///
/// This will convert the `wildcard` pattern to a regular expression pattern for matching files.
///
pub fn wildcard_to_regex_pattern(wildcard: &str) -> String {
    let regex_pattern = wildcard
        .chars()
        .map(|c| {
            match c {
                '*' => ".*".to_string(),
                '.' => r"\.".to_string(),
                _ => regex::escape(&c.to_string()),
            }
        })
        .collect::<String>();

    format!("^{}$", regex_pattern)
}

/// Parse Full Path
///
/// Parses a full file path and splits it into the directory path and the filename.
///
/// # Arguments
///
/// * `full_path` - A full file path that includes the directory path and file pattern.
///
/// # Returns
///
/// A tuple containing the directory path and the file pattern.
///
/// # Example
///
/// ```no
/// let full_path = "path/to/some_*.txt";
/// let (dir_path, file_pattern) = parse_full_path(full_path);
/// ```
///
/// This will split the `full_path` into `dir_path` and `filename`.
///
pub fn parse_full_path(full_path: &str) -> (&str, &str) {
    if let Some(last_slash) = full_path.rfind('/') {
        let (directory, filename) = full_path.split_at(last_slash + 1);
        let directory = &directory[0..last_slash];
        let filename = &filename[0..];
        (directory, filename)
    } else {
        ("", full_path)
    }
}

/// Find Matching Files
///
/// Finds files that match the given file pattern in the specified directory.
///
/// # Arguments
///
/// * `full_path` - A full path pattern that includes the directory and file pattern.
///
/// # Returns
///
/// A vector of strings representing the matching file paths or an error.
///
/// # Example
///
/// ```no
/// let full_path = "path/to/some_*.txt";
/// let matching_files = find_matching_files(full_path);
/// ```
///
/// This will find and return a vector of matching file paths based on the `full_path` pattern.
///
pub fn find_matching_files(full_path: &str) -> Result<Vec<String>, String> {
    let (dir_path, file_pattern) = parse_full_path(full_path);
    let regex_pattern = wildcard_to_regex_pattern(file_pattern);
    let regex = Regex::new(&regex_pattern).unwrap();
    let mut matching_files: Vec<String> = vec![];
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name();
                if regex.is_match(&*filename.to_string_lossy()) {
                    let full_path = format!("{}/{}", dir_path, filename.to_string_lossy());
                    matching_files.push(full_path);
                }
            }
        }
    } else {
        return Err(format!("mmv: Not able to read directory"));
    }
    if matching_files.is_empty() {
        return Err(format!("mmv: Files for pattern '{}' not found", full_path));
    }
    Ok(matching_files)
}


#[test]
fn test_wildcard_to_regex_pattern() {
    assert_eq!(wildcard_to_regex_pattern("some_*file*_name.txt"),
               r"^some_.*file.*_name\.txt$");
    assert_eq!(wildcard_to_regex_pattern("*file*_name.*"),
               r"^.*file.*_name\..*$");
}


#[test]
fn test_parse_full_path() {
    assert_eq!(parse_full_path("path/to/file.txt"),
               ("path/to", "file.txt"));
    assert_eq!(parse_full_path("to/file.txt"),
               ("to", "file.txt"));
    assert_eq!(parse_full_path("file.txt"),
               ("", "file.txt"));
}

#[test]
fn test_find_matching_files() {
    let all_files = vec!["abba.txt", "aba.txt", "aba.bin",
                         "b.txt", "b.exe", "hello_b_world", "bba.exe.txt"];
    let temp_dir = tempdir::TempDir::new("my_temp_dir").expect("Failed to create temporary directory");
    let path = temp_dir.path().join("path/to/");

    for file_path in &all_files {
        let full_path = path.as_path().join(file_path);
        if let Some(parent_dir) = full_path.parent() {
            // Create parent directories if they don't exist
            fs::create_dir_all(parent_dir).expect("Failed to create parent directories");
        }

        // Create and write to the file
        let _ = fs::File::create(&full_path).expect("Failed to create file");
    }

    let pattern = "*b*.txt";
    let pattern_path = path.as_path().join(pattern);
    let mut result = find_matching_files(&pattern_path.to_string_lossy());
    let mut res_files = vec![temp_dir.path().join("abba.txt").to_string_lossy().to_string(),
                             temp_dir.path().join("aba.txt").to_string_lossy().to_string(),
                             temp_dir.path().join("bba.exe.txt").to_string_lossy().to_string(),
                             temp_dir.path().join("b.txt").to_string_lossy().to_string()];
    assert_eq!(res_files.sort(), result.unwrap().sort());
}


