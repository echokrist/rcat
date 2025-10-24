use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

fn absolute_path(arg: &str) -> io::Result<PathBuf> {
    let p = Path::new(arg);
    if p.is_absolute() {
        Ok(p.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(p))
    }
}

pub fn read_lines<P: AsRef<Path>>(
    filename: P,
) -> io::Result<impl Iterator<Item = io::Result<Vec<u8>>>> {
    let path = absolute_path(&filename.as_ref().to_string_lossy())?;
    let meta = fs::metadata(&path).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path '{}' is not a valid file.", path.display()),
        )
    })?;
    if !meta.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path '{}' is not a valid file.", path.display()),
        ));
    }
    let file = File::open(&path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("failed to open '{}': {}", path.display(), e),
        )
    })?;
    let reader = BufReader::with_capacity(128 * 1024, file);
    Ok(reader.split(b'\n').map(move |res| {
        res.map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("error while reading '{}': {}", path.display(), e),
            )
        })
        .map(|mut line| {
            if let Some(&b'\r') = line.last() {
                line.pop();
            }
            line
        })
    }))
}

pub fn rcat<P, W>(filename: P, mut out: W) -> io::Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    let mut buf = Vec::with_capacity(8 * 1024);
    for line_res in read_lines(&filename)? {
        let mut line = line_res?;
        std::mem::swap(&mut buf, &mut line);
        out.write_all(&buf)?;
        buf.clear();
    }
    out.flush()
}
