use std::fs::File;
use std::os::unix::fs::FileExt;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let mut bitcask = Bitcask::new("kek.txt");

    loop {
        println!("Enter a command:");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("failed to readline");

        println!("Enter a key:");
        let mut key = String::new();
        io::stdin().read_line(&mut key).expect("failed to readline");

        if command.eq("write\n") {
            println!("Enter a value:");
            let mut value = String::new();
            io::stdin().read_line(&mut value).expect("failed to readline");

            bitcask.write(key, value);
        } else if command.eq("read\n") {
            let value = bitcask.read(&key);
            println!("{}", value);
        }

        println!("----------------")
    }
}

struct Bitcask {
    file: File,
    index: HashMap<String, (u64, usize)>,
}

impl Bitcask {
    fn new<P: AsRef<Path>>(path: P) -> Bitcask {
        Bitcask {
            file: File::create(&path).unwrap(),
            index: HashMap::new(),
        }
    }

    fn write(&mut self, key: String, value: String) {
        let offset = self.file.metadata().unwrap().len();
        let bytes_value = value.as_bytes();

        self.index.insert(key, (offset, bytes_value.len()));
        self.file.write_all(value.as_bytes()).unwrap();
    }

    fn read(&mut self, key: &String) -> String {
        let (offset, length) = self.index.get(key).unwrap();
        println!("{}", offset);
        println!("{}", length);

        // let mut buf_reader = BufReader::new(self.file);
        let mut buf = Vec::with_capacity(*length);
        self.file.read_exact_at(&mut buf, *offset).unwrap();
        String::from_utf8(buf).unwrap()
    }
}
