use std::{env, ffi::OsString, path::PathBuf};

pub fn parse_args() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(test_args) = env::var("RCAT_TEST_ARGS") {
        let mut parts = test_args.splitn(2, char::is_whitespace);
        let _prog = parts.next(); // ignore program name
        let path_str = parts
            .next()
            .ok_or_else(|| "RCAT_TEST_ARGS must contain a path".to_string())?;
        return validate_path(path_str);
    }

    let mut args = env::args_os();
    let prog_name: OsString = args.next().unwrap_or_else(|| "program".into());

    let path_os: OsString = args
        .next()
        .ok_or_else(|| format!("Usage: {} <file_path>", prog_name.to_string_lossy()))?;

    let path_display = path_os.to_string_lossy().into_owned();
    validate_path(&path_display)
}

fn validate_path<S: AsRef<str>>(path_str: S) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let raw_path = PathBuf::from(path_str.as_ref());

    let abs_path = match std::fs::canonicalize(&raw_path) {
        Ok(p) => p,
        Err(_) => std::env::current_dir()?.join(&raw_path),
    };

    if !abs_path.is_file() {
        return Err(format!("Path '{}' is not a valid file.", abs_path.display()).into());
    }

    Ok(abs_path)
}
