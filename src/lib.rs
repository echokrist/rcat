mod utils;

pub use utils::cli_args_handler::parse_args;
pub use utils::file_handler::{rcat, read_lines};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = parse_args()?;

    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut writer = std::io::BufWriter::new(handle);

    rcat(&file_path, &mut writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs, io::Write, path::PathBuf};

    fn unique_tmp_path() -> PathBuf {
        let mut p = env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        p.push(format!("rcat_test_{nanos}.txt"));
        p
    }

    fn create_temp_file(lines: &[&str]) -> PathBuf {
        let path = unique_tmp_path();
        {
            let mut f = fs::File::create(&path).expect("cannot create temp file");
            for line in lines {
                writeln!(f, "{line}").expect("cannot write to temp file");
            }
        }
        path
    }

    #[test]
    fn arg_is_file() {
        let tmp_path = create_temp_file(&["first line", "second line"]);

        unsafe {
            env::set_var(
                "RCAT_TEST_ARGS",
                format!("{} {}", env::args().next().unwrap(), tmp_path.display()),
            );
        }

        let result = parse_args();
        assert!(result.is_ok(), "parser should accept a valid file");

        fs::remove_file(tmp_path).expect("could not delete temp file");
        unsafe {
            env::remove_var("RCAT_TEST_ARGS");
        }
    }
    #[test]
    fn read_lines_success() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_path = create_temp_file(&["alpha", "beta"]);

        let lines: Vec<String> = read_lines(&tmp_path)?
            .map(|res| {
                let mut s = String::from_utf8(res?)?;
                if s.ends_with('\n') {
                    s.pop();
                    if s.ends_with('\r') {
                        s.pop();
                    }
                }
                Ok(s)
            })
            .collect::<Result<_, Box<dyn std::error::Error>>>()?;

        assert_eq!(lines, vec!["alpha".to_string(), "beta".to_string()]);
        fs::remove_file(tmp_path)?;
        Ok(())
    }
}
