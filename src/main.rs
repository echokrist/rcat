use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

fn main() {
    let file_path_buf: PathBuf = match cli_arg_handler() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Argument error: {}", e);
            std::process::exit(1);
        }
    };

    let file_path: &Path = &file_path_buf;

    match read_file_contents_by_lines(file_path) {
        Ok(content) => {
            for line in content {
                if let Ok(l) = line {
                    println!("{}", l);
                } else if let Err(e) = line {
                    eprintln!("Warning: Error reading line from {:?}: {}", file_path, e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file {:?}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}

fn cli_arg_handler() -> Result<PathBuf, String> {
    let args: Vec<String> = env::args().collect();
    let program_name = args.get(0).map_or("program", |s| s.as_str());

    let Some(path_arg_str) = args.get(1) else {
        return Err(format!("Usage: {} <file_path>", program_name));
    };

    let path_buf = PathBuf::from(path_arg_str);
    let path_ref: &Path = &path_buf;

    if !path_ref.is_file() {
        return Err(format!("Path '{}' is not a valid file.", path_arg_str));
    }
    Ok(path_buf)
}

fn read_file_contents_by_lines<P: AsRef<Path>>(
    filename: P,
) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
