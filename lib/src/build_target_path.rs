use regex::Regex;
use crate::search_by_pattern;
use search_by_pattern::parse_full_path;

/// Extract Generic Parts
///
/// Extracts generic parts from a filename which are hidden under `*` based on
/// a file pattern and returns them as a vector of strings.
///
/// This function takes two full paths as input: one containing the full filename and another containing
/// a file pattern with placeholders. It extracts the parts of the filename that match the pattern and
/// returns them as a vector of strings.
///
/// # Arguments
///
/// * `full_path_with_filename` - The full path containing the filename.
/// * `full_path_with_file_pattern` - The full path containing the file pattern.
///
/// # Returns
///
/// A vector of strings representing the extracted generic parts.
///
/// # Example
///
/// ```no
/// let full_path_with_filename = "path/to/some_A_filename.bin";
/// let full_path_with_file_pattern = "path/to/some_*_filename.*";
/// let generic_parts = extract_generic_parts(full_path_with_filename, full_path_with_file_pattern);
/// ```
///
/// This will extract the generic parts from the filename based on the file pattern.
///
pub fn extract_generic_parts(full_path_with_filename: &str,
                             full_path_with_file_pattern: &str) -> Vec<String> {
    let (_, filename) = parse_full_path(full_path_with_filename);
    let (_, file_pattern) = parse_full_path(full_path_with_file_pattern);
    let regex_file_pattern = format!("^{}$",
                                     file_pattern.replace(".", r"\.").replace("*", "(.*?)"));
    let regex = Regex::new(&regex_file_pattern).unwrap();

    if let Some(captures) = regex.captures(filename) {
        return captures
            .iter()
            .skip(1)
            .filter_map(|capture| capture.map(|c|
                c.as_str().to_string()))
            .collect();
    }
    Vec::new()
}

/// Build Target Path
///
/// Builds a target path by inserting extracted parts into a given output path pattern.
///
/// This function takes a vector of extracted parts and an output path pattern with placeholders (#1, #2, etc.),
/// and constructs the target path by replacing the placeholders with the extracted parts.
///
/// # Arguments
///
/// * `substr_to_insert` - A vector of extracted parts.
/// * `full_output_path_pattern` - The full output path pattern with placeholders.
///
/// # Returns
///
/// The constructed target path as a `String`.
///
/// # Panics
///
/// This function will panic if it encounters an invalid index in the output path pattern.
///
/// # Example
///
/// ```no
/// let substr_to_insert = vec!["A".to_string(), "filename".to_string()];
/// let full_output_path_pattern = "path2/to/changed_#1_filename.#2";
/// let target_path = build_target_path(substr_to_insert, full_output_path_pattern);
/// ```
///
/// This will build the target path by inserting the extracted parts into the output path pattern.
///
pub fn build_target_path(substr_to_insert: Vec<String>, full_output_path_pattern: &str) -> String {
    let (output_path, pattern) = parse_full_path(full_output_path_pattern);
    let regex = Regex::new(r"#(\d+)").unwrap();
    let filename_with_substr = regex.replace_all(pattern, |caps: &regex::Captures| {
        let index: usize = caps[1].parse().expect("Invalid index");
        if index <= substr_to_insert.len() {
            substr_to_insert[index - 1].as_str()
        } else {
            // If the index is out of range, replace with an empty string
            ""
        }
    });
    let full_path = format!("{}/{}", output_path, filename_with_substr);
    full_path.to_string()
}

#[test]
fn test_extract_generic_parts() {
    assert_eq!(extract_generic_parts("some_file_name", "som*e_n*"),
               vec!["e_fil", "ame"]);
    assert_eq!(extract_generic_parts("a_bc_def_hello.txt", "*e*he*"),
               vec!["a_bc_d", "f_", "llo.txt"]);
    assert_eq!(extract_generic_parts("a_b", "a_*b"),
               vec![""]);
    assert_eq!(extract_generic_parts("a_b", "*a_*b"),
               vec!["", ""]);
}


#[test]
fn test_build_path_target() {
    let generic_parts: Vec<String> = vec![String::from("hello"), String::from("world"),
                                          String::from("txt")];
    let path = "path/to/#1_#2.#3";
    assert_eq!(build_target_path(generic_parts, path),
               "path/to/hello_world.txt");

    let generic_parts: Vec<String> = vec![String::from(""), String::from("he"),
                                          String::from("j")];
    let path = "path/to/#1#2#2#2_#3_#4.txt";
    assert_eq!(build_target_path(generic_parts, path),
               "path/to/hehehe_j_.txt");
}
