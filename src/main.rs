use mmove::mass_move;

use clap::Parser;

/// Mass Move Files
///
/// Takes two arguments: a file selection pattern and a destination path pattern.
/// Moves files, overwriting existing ones, and display the original and new paths.
///
/// # Arguments
/// * `source_pattern` -   A pattern for selecting files, containing path, name, and the `*` character
///   to represent a substring of any length (including an empty string). The `*` character can
///   only appear in the filename.
/// * `destination_pattern` - A pattern for the destination path, formed with regular characters
///   and special markers like `#1`, `#2`, and so on. These markers indicate which portions
///   marked with asterisks in the source file pattern should be inserted into the new filename.
///
/// # Flags
///
/// * `-h`, `--help` - Show help documentation.
/// * `-f`, `--force` - Overwrite existing files if they exist.
///
/// # Example
/// ```
/// ./mmv 'source_pattern' 'target_pattern' --force(optional)
/// ```
///
#[derive(Parser, Debug)]
/// Command-line arguments for the 'mmv' tool.
struct Args {
    ///   A pattern for selecting files, containing path, name, and the `*` character
    ///   to represent a substring of any length (including an empty string). The `*` character can
    ///   only appear in the filename.
    pub source_pattern: String,
    ///  A pattern for the destination path, formed with regular characters
    ///   and special markers like `#1`, `#2`, and so on. These markers indicate which portions
    ///   marked with asterisks in the source file pattern should be inserted into the new filename.
    pub target_pattern: String,
    /// Force mode: Replace existing files in the destination directory (optional).
    #[clap(short, long)]
    pub force: bool,
}

/// The entry point of the 'mmv' tool. Parses command-line arguments and invokes the file
/// renaming operation.
fn main() {
    let args = Args::parse();
    let result = mass_move::mass_move(&args.source_pattern, &args.target_pattern, args.force);
    match result {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprint!("{}", e);
            std::process::exit(1)
        }
    }
}


