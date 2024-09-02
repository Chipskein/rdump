use std::path::PathBuf;
use std::io::{Read,Error};

pub fn dump(path_buf: PathBuf)->Result<String,Error> {
    let mut dump_str = String::new();
    let file = match std::fs::File::open(path_buf) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut buffer = [0; 16];
    let mut buf_reader = std::io::BufReader::new(file);
    let mut total_bytes:usize =0;
    loop {
        let mut line=String::from(format!("{:08x} ", total_bytes));
        let bytes_read = match buf_reader.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => return Err(e),
        };
        total_bytes += bytes_read;
        
        for i in (0..bytes_read).step_by(2) {
            if i+1<bytes_read {
                let bytes= format!("{:02x?}{:02x?}", buffer[i+1],buffer[i]);
                line.push_str(&bytes);
            } else {
                let bytes= format!("{:02x?}", buffer[i]);
                line.push_str(&bytes);
            }
            if i+2<bytes_read {
                line.push(' ');
            }
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
        let t=dump(PathBuf::from("Cargo.toml")).unwrap();
        assert_eq!(t, "0000000 705b 6361 616b 6567 0a5d 616e 656d 3d20\n0000010 2220 6472 6d75 2270 760a 7265 6973 6e6f\n0000020 3d20 2220 2e30 2e31 2230 650a 6964 6974\n0000030 6e6f 3d20 2220 3032 3132 0a22 645b 7065\n0000040 6e65 6564 636e 6569 5d73\n000004a \n"
    );
    }
}
