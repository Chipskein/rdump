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
                    let bytes = format!("{:04x?}", buffer[i]);
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
                String::from(" ")
            };
            for i in 0..16 {
                if bytes_read == 0{
                    continue;
                }
                if i >= bytes_read {
                    line.push_str("   ");
                    if i == 7 {
                        line.push(' ');
                    }
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
    fn test_default() {
        assert_eq!(dump(PathBuf::from("Cargo.toml"), false).unwrap(), "0000000 705b 6361 616b 6567 0a5d 616e 656d 3d20\n0000010 2220 6472 6d75 2270 760a 7265 6973 6e6f\n0000020 3d20 2220 2e30 2e31 2230 650a 6964 6974\n0000030 6e6f 3d20 2220 3032 3132 0a22 645b 7065\n0000040 6e65 6564 636e 6569 5d73\n000004a \n");
        assert_eq!(dump(PathBuf::from("Cargo.lock"), false).unwrap(), "0000000 2023 6854 7369 6620 6c69 2065 7369 6120\n0000010 7475 6d6f 7461 6369 6c61 796c 4020 6567\n0000020 656e 6172 6574 2064 7962 4320 7261 6f67\n0000030 0a2e 2023 7449 6920 2073 6f6e 2074 6e69\n0000040 6574 646e 6465 6620 726f 6d20 6e61 6175\n0000050 206c 6465 7469 6e69 2e67 760a 7265 6973\n0000060 6e6f 3d20 3320 0a0a 5b5b 6170 6b63 6761\n0000070 5d65 0a5d 616e 656d 3d20 2220 6472 6d75\n0000080 2270 760a 7265 6973 6e6f 3d20 2220 2e30\n0000090 2e31 2230 000a\n0000095 \n");
        assert_eq!(dump(PathBuf::from(".gitignore"), false).unwrap(), "0000000 742f 7261 6567 0a74\n0000008 \n");
    }
    #[test]
    fn test_canonical() {
        assert_eq!(dump(PathBuf::from("Cargo.toml"), true).unwrap(), "00000000  5b 70 61 63 6b 61 67 65  5d 0a 6e 61 6d 65 20 3d  |[package].name =|\n00000010  20 22 72 64 75 6d 70 22  0a 76 65 72 73 69 6f 6e  | \"rdump\".version|\n00000020  20 3d 20 22 30 2e 31 2e  30 22 0a 65 64 69 74 69  | = \"0.1.0\".editi|\n00000030  6f 6e 20 3d 20 22 32 30  32 31 22 0a 5b 64 65 70  |on = \"2021\".[dep|\n00000040  65 6e 64 65 6e 63 69 65  73 5d                    |endencies]|\n0000004a   \n");
        assert_eq!(dump(PathBuf::from("Cargo.lock"), true).unwrap(), "00000000  23 20 54 68 69 73 20 66  69 6c 65 20 69 73 20 61  |# This file is a|\n00000010  75 74 6f 6d 61 74 69 63  61 6c 6c 79 20 40 67 65  |utomatically @ge|\n00000020  6e 65 72 61 74 65 64 20  62 79 20 43 61 72 67 6f  |nerated by Cargo|\n00000030  2e 0a 23 20 49 74 20 69  73 20 6e 6f 74 20 69 6e  |..# It is not in|\n00000040  74 65 6e 64 65 64 20 66  6f 72 20 6d 61 6e 75 61  |tended for manua|\n00000050  6c 20 65 64 69 74 69 6e  67 2e 0a 76 65 72 73 69  |l editing..versi|\n00000060  6f 6e 20 3d 20 33 0a 0a  5b 5b 70 61 63 6b 61 67  |on = 3..[[packag|\n00000070  65 5d 5d 0a 6e 61 6d 65  20 3d 20 22 72 64 75 6d  |e]].name = \"rdum|\n00000080  70 22 0a 76 65 72 73 69  6f 6e 20 3d 20 22 30 2e  |p\".version = \"0.|\n00000090  31 2e 30 22 0a                                    |1.0\".|\n00000095   \n");
        assert_eq!(dump(PathBuf::from(".gitignore"), true).unwrap(), "00000000  2f 74 61 72 67 65 74 0a                           |/target.|\n00000008   \n");
    }
}
