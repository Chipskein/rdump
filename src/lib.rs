use std::io::{Error, Read};
use std::path::PathBuf;

pub fn dump(path_buf: PathBuf, canonical: bool) -> Result<String, Error> {
    let mut dump_str = String::new();
    let file = match std::fs::File::open(path_buf) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut buffer = [0; 16];
    let mut buf_reader = std::io::BufReader::new(file);
    let mut total_bytes: usize = 0;
    loop {
        let mut line = if !canonical {
            String::from(format!("{:07x} ", total_bytes))
        } else {
            String::from(format!("{:08x}  ", total_bytes))
        };
        let bytes_read = match buf_reader.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => return Err(e),
        };
        total_bytes += bytes_read;
        if !canonical {
            for i in (0..bytes_read).step_by(2) {
                if i + 1 < bytes_read {
                    let bytes = format!("{:02x?}{:02x?}", buffer[i + 1], buffer[i]);
                    line.push_str(&bytes);
                } else {
                    let bytes = format!("{:02x?}", buffer[i]);
                    line.push_str(&bytes);
                }
                if i + 2 < bytes_read {
                    line.push(' ');
                }
            }
        } else {
            let mut ascii_text = if bytes_read != 0 {
                String::from(" |")
            } else {
                String::from("")
            };
            for i in 0..16 {
                if bytes_read == 0{
                    continue;
                }
                if i >= bytes_read {
                    line.push_str("   ");
                    continue;
                }
                line.push_str(&format!("{:02x} ", buffer[i]));
                if i == 7 {
                    line.push(' ');
                }
                let c = buffer[i] as char;
                if c != '\n' && c != '\r' {
                    ascii_text.push(c);
                } else {
                    ascii_text.push('.');
                }
            }

            if bytes_read != 0 {
                ascii_text.push('|');
            }
            line.push_str(&ascii_text);
        }
        line.push('\n');
        dump_str.push_str(&line);
        buffer = [0; 16];
        if bytes_read == 0 {
            break;
        }
    }
    Ok(dump_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let t = dump(PathBuf::from("Cargo.toml"), false).unwrap();
        assert_eq!(t, "0000000 705b 6361 616b 6567 0a5d 616e 656d 3d20\n0000010 2220 6472 6d75 2270 760a 7265 6973 6e6f\n0000020 3d20 2220 2e30 2e31 2230 650a 6964 6974\n0000030 6e6f 3d20 2220 3032 3132 0a22 645b 7065\n0000040 6e65 6564 636e 6569 5d73\n000004a \n");
    }
    #[test]
    fn test_canonical() {
        let t = dump(PathBuf::from("Cargo.toml"), true).unwrap();
        assert_eq!(t, "00000000  5b 70 61 63 6b 61 67 65  5d 0a 6e 61 6d 65 20 3d  |[package].name =|\n00000010  20 22 72 64 75 6d 70 22  0a 76 65 72 73 69 6f 6e  | \"rdump\".version|\n00000020  20 3d 20 22 30 2e 31 2e  30 22 0a 65 64 69 74 69  | = \"0.1.0\".editi|\n00000030  6f 6e 20 3d 20 22 32 30  32 31 22 0a 5b 64 65 70  |on = \"2021\".[dep|\n00000040  65 6e 64 65 6e 63 69 65  73 5d                    |endencies]|\n0000004a  \n");
    }
}
