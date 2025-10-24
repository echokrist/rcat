use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

pub fn read_lines<P: AsRef<Path>>(
    filename: P,
) -> io::Result<impl Iterator<Item = io::Result<Vec<u8>>>> {
    let file = File::open(filename)?;

    let reader = BufReader::with_capacity(128 * 1024, file);

    Ok(reader.split(b'\n').map(|res| {
        res.map(|mut line| {
            if let Some(&b'\r') = line.last() {
                line.pop();
            }
            line
        })
    }))
}

pub fn rcat<P: AsRef<Path>, W: Write>(filename: P, mut out: W) -> io::Result<()> {
    let mut reusable_buf = Vec::with_capacity(8 * 1024);

    for line_res in read_lines(filename)? {
        let mut line = line_res?;
        std::mem::swap(&mut reusable_buf, &mut line);
        out.write_all(&reusable_buf)?;
        reusable_buf.clear();
    }
    out.flush()
}
