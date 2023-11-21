use crate::build_target_path;
use crate::search_by_pattern;
use std::path::Path;

/// Mass move files that match a source pattern to a destination pattern.
///
/// This function takes two patterns, a source pattern and a destination pattern,
/// and moves files matching the source pattern to the corresponding destination
/// paths. It also supports an optional `force` flag to replace existing files
/// in the destination directory.
///
/// # Arguments
///
/// * `source_pattern` - A string representing the pattern to match source files.
/// * `destination_pattern` - A string representing the pattern to generate destination paths.
/// * `force` - A boolean flag indicating whether to replace existing files (if `true`).
/// # Example
///
/// ```no
/// use mass_move::mass_move;
///
/// mass_move("path/to/some_*_filename.*", "path2/to/changed_#1_filename.#2", true);
/// ```
///
/// This function will display the original file paths and their paths after the move, and it will
/// also move the files accordingly, overwriting existing files if the `-f` flag is specified.
///
pub fn mass_move(source_pattern: &str, destination_pattern: &str, force: bool) -> Result<(), String> {
    let result_source_files = search_by_pattern::find_matching_files(source_pattern);
    let source_files;
    match result_source_files {
        Ok(_) => source_files = result_source_files.unwrap(),
        Err(error) => return Err(format!("{}", error)),
    }
    let mut destination_paths = Vec::new();
    for source_file_with_path in source_files {
        let source_path = Path::new(&source_file_with_path);
        let parts_of_new_filename = build_target_path::extract_generic_parts(
            &source_file_with_path, source_pattern);
        let destination_path_filename = build_target_path::build_target_path(
            parts_of_new_filename, destination_pattern);
        let destination_path = Path::new(&destination_path_filename);
        if destination_path.exists() {
            if !force {
                return Err(format!("mmv: Not able to replace existing file: {}", destination_path.to_string_lossy()));
            } else {
                match std::fs::remove_file(destination_path) {
                    Ok(_) => destination_paths.push((source_path.to_path_buf(), destination_path.to_path_buf())),
                    Err(_) => return Err(format!("mmv: Not able to replace existing file")),
                }
            }
        } else {
            destination_paths.push((source_path.to_path_buf(), destination_path.to_path_buf()));
        }
    }
    let (directory, _) = search_by_pattern::parse_full_path(destination_pattern);
    let path = Path::new(directory);

    if !path.exists() {
        std::fs::create_dir_all(path).expect("mmv: Not able to move file");
    }
    for source_destination_paths in destination_paths {
        match std::fs::rename(Path::new(&source_destination_paths.0),
                              Path::new(&source_destination_paths.1)) {
            Ok(_) => println!("{} -> {}", &source_destination_paths.0.to_string_lossy(),
                              &source_destination_paths.1.to_string_lossy()),
            Err(e) => return Err(format!("Error: {}", e)),
        }
    }
    Ok(())
}

