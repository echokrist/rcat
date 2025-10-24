use std::{env, path::PathBuf};

pub fn parse_args() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(test_args) = env::var("RCAT_TEST_ARGS") {
        let mut parts = test_args.splitn(2, char::is_whitespace);
        let _prog = parts.next();
        let path_str = parts
            .next()
            .ok_or_else(|| "RCAT_TEST_ARGS must contain a path".to_string())?;
        let path_buf = PathBuf::from(path_str);
        if !path_buf.is_file() {
            return Err(format!("Path '{}' is not a valid file.", path_str).into());
        }
        return Ok(path_buf);
    }

    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "program".into());

    let path_arg = args
        .next()
        .ok_or_else(|| format!("Usage: {} <file_path>", program_name))?;

    let path_buf = PathBuf::from(&path_arg);
    if !path_buf.is_file() {
        return Err(format!("Path '{}' is not a valid file.", path_arg).into());
    }

    Ok(path_buf)
}
