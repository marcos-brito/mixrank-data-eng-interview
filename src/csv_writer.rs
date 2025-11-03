use std::io::{self, BufWriter, Stdout, Write};

pub struct CsvWriter<W: Write> {
    buf: BufWriter<W>,
}

impl<W: Write> CsvWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            buf: BufWriter::new(writer),
        }
    }

    pub fn add_record<T: ToString>(&mut self, record: T) {
        writeln!(self.buf, "{}", record.to_string()).unwrap();
    }
}

impl Default for CsvWriter<Stdout> {
    fn default() -> Self {
        CsvWriter::new(io::stdout())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_writer() {
        let buf = Vec::new();
        let mut writer = CsvWriter::new(buf);

        writer.add_record("birth, age, gender");
        writer.add_record("null, 43, male");
        writer.add_record("05/21, 23, female");

        assert_eq!(
            str::from_utf8(&writer.buf.into_inner().unwrap()).unwrap(),
            "birth, age, gender\nnull, 43, male\n05/21, 23, female\n"
        )
    }
}
