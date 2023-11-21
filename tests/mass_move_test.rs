use tempdir::TempDir;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use mmove::mass_move::mass_move;

fn test_mmv_with_existing_files(temp_dir: TempDir, path_s: &str, path_d: &str, source_pattern: &str,
                                dest_pattern: &str, file_paths_source: Vec<&str>,
                                file_paths_dest: Vec<&str>, force: bool) -> Result<(), String> {
    // Specify the source file paths
    let path_source = temp_dir.path().join(path_s);
    let path_destination = temp_dir.path().join(path_d);
    // Write "hello_world" to the specified files and print their paths and contents
    for file_path in &file_paths_source {
        let full_path = path_source.as_path().join(file_path);
        if let Some(parent_dir) = full_path.parent() {
            // Create parent directories if they don't exist
            fs::create_dir_all(parent_dir).expect("Failed to create parent directories");
        }
        let content = "hello_world";

        // Create and write to the file
        let mut file = File::create(&full_path).expect("Failed to create file");
        file.write_all(content.as_bytes()).expect("Failed to write to file");
    }

    let full_path_source = path_source.as_path().join(source_pattern);
    let full_path_destination = path_destination.as_path().join(dest_pattern);

    let result = mass_move(&full_path_source.to_string_lossy(),
                           &full_path_destination.to_string_lossy(), force);
    match result {
        Err(e) => return Err(format!("{}", e)),
        Ok(_) => {}
    }

    if let Ok(entries) = fs::read_dir(path_source.clone()) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name();
                assert!(!file_paths_source.contains(&&*filename.to_string_lossy()));
            }
        }
    }

    if let Ok(entries) = fs::read_dir(path_destination.clone()) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name();

                assert!(file_paths_dest.contains(&&*filename.to_string_lossy()));

                let full_path_destination = path_destination.as_path().join(filename);
                let mut file = File::open(&full_path_destination).expect("Can't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Can't read file");

                assert_eq!(contents, "hello_world");
            }
        }
    }
    Ok(())
}

#[test]
fn test_mmv_with_different_paths() {
    let temp_dir = TempDir::new("my_temp_dir").expect("Failed to create temporary directory");
    let path_s = "path/to/";
    let path_d = "path2/to/";
    let source_pattern = "some_*_filename.*";
    let dest_pattern = "change_#1_filename.#2";
    let file_paths_source = vec![
        "some_A_filename.bin",
        "some_B_filename.txt",
        "some_C_filename.exe",
    ];
    // Specify the destination file paths
    let file_paths_dest = vec![
        "change_A_filename.bin",
        "change_B_filename.txt",
        "change_C_filename.exe",
    ];
    assert!(!test_mmv_with_existing_files(temp_dir, path_s, path_d, source_pattern,
                                          dest_pattern, file_paths_source, file_paths_dest, false).is_err());
}


#[test]
fn test_mmv_with_same_paths() {
    let temp_dir = TempDir::new("my_temp_dir").expect("Failed to create temporary directory");
    let path_s = "path/to/";
    let path_d = "path/to/";
    let source_pattern = "some_*_filename.*";
    let dest_pattern = "change_#1_filename.#2";
    let file_paths_source = vec![
        "some_A_filename.bin",
        "some_B_filename.txt",
        "some_C_filename.exe",
    ];
    // Specify the destination file paths
    let file_paths_dest = vec![
        "change_A_filename.bin",
        "change_B_filename.txt",
        "change_C_filename.exe",
    ];

    assert!(!test_mmv_with_existing_files(temp_dir, path_s, path_d, source_pattern,
                                          dest_pattern, file_paths_source, file_paths_dest, false).is_err());
}


#[test]
fn test_mmv_with_nonexistent_files() {
    let temp_dir = TempDir::new("my_temp_dir").expect("Failed to create temporary directory");
    let path_s = "path/to/";
    let path_d = "path/to/";
    let source_pattern = "some_*_file.*";
    let dest_pattern = "change_#1_filename.#2";
    let full_path = temp_dir.path().join(path_s).join(source_pattern);
    let file_paths_source = vec![
        "some_A_filename.bin",
        "some_B_filename.txt",
        "some_C_filename.exe",
    ];
    let file_paths_dest = vec![
        "change_A_filename.bin",
        "change_B_filename.txt",
        "change_C_filename.exe",
    ];

    let result = test_mmv_with_existing_files(temp_dir, path_s, path_d, source_pattern,
                                              dest_pattern, file_paths_source, file_paths_dest, false);

    assert!(result.is_err());
    assert_eq!(format!("mmv: Files for pattern '{}' not found", full_path.to_string_lossy()),
               format!("{}", result.unwrap_err()));
}


#[test]
fn test_mmv_with_existent_files() {
    let temp_dir = TempDir::new("my_temp_dir").expect("Failed to create temporary directory");
    let path_s = "path/to/";
    let path_d = "path2/to/";
    let source_pattern = "some_*_filename.*";
    let dest_pattern = "change_#1_filename.#2";
    let dest_path = temp_dir.path().join(path_d).join("change_A_filename.bin");
    let file_paths_source = vec![
        "some_A_filename.bin",
        "some_B_filename.txt",
        "some_C_filename.exe",
    ];
    let file_paths_dest = vec![
        "change_A_filename.bin",
        "change_B_filename.txt",
        "change_C_filename.exe",
    ];

    if let Some(parent_dir) = dest_path.parent() {
        fs::create_dir_all(parent_dir).expect("Failed to create parent directories");
    }
    let _ = File::create(&dest_path).expect("Failed to create file");

    let result = test_mmv_with_existing_files(temp_dir, path_s, path_d, source_pattern,
                                              dest_pattern, file_paths_source, file_paths_dest, false);

    // Use an assertion to check if the result is an error
    assert!(result.is_err());
    assert_eq!(format!("mmv: Not able to replace existing file: {}", dest_path.to_string_lossy()),
               format!("{}", result.unwrap_err()));
}

#[test]
fn test_mmv_with_existent_files_force() {
    let temp_dir = TempDir::new("my_temp_dir").expect("Failed to create temporary directory");
    let path_s = "path/to/";
    let path_d = "path2/to/";
    let source_pattern = "some_*_filename.*";
    let dest_pattern = "change_#1_filename.#2";
    let dest_path = temp_dir.path().join(path_d).join("change_A_filename.bin");
    let file_paths_source = vec![
        "some_A_filename.bin",
        "some_B_filename.txt",
        "some_C_filename.exe",
    ];
    let file_paths_dest = vec![
        "change_A_filename.bin",
        "change_B_filename.txt",
        "change_C_filename.exe",
    ];

    if let Some(parent_dir) = dest_path.parent() {
        fs::create_dir_all(parent_dir).expect("Failed to create parent directories");
    }

    let content = "HELLO_WORLD";
    // Create and write to the file
    let mut file = File::create(&dest_path).expect("Failed to create file");
    file.write_all(content.as_bytes()).expect("Failed to write to file");

    assert!(!test_mmv_with_existing_files(temp_dir, path_s, path_d, source_pattern,
                                          dest_pattern, file_paths_source, file_paths_dest, true).is_err());

}
