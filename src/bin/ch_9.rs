#![allow(unused)]

use std::{
    fs::{self, File},
    io::{self, Error, ErrorKind, Read},
};

const FILE_PATH: &str = "hello.txt";

fn main() {
    let username_from_file = read_username_from_file();

    if let Err(MyCustomError::FileNotFound { path, source }) = &username_from_file {
        println!("Read user name from file response => path: {path}");
        println!("Read user name from file response => source: {source:#?}");
    }
}

fn read_or_create() {
    let result = File::open(&FILE_PATH);

    match result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(&FILE_PATH) {
                Ok(fc) => fc,
                Err(e) => panic!("{err:?}", err = e),
            },
            _ => {
                panic!("{err:?}");
            }
        },
    };
}

fn read() -> Result<String, Error> {
    let mut username = String::new();

    File::open(&FILE_PATH)?.read_to_string(&mut username)?;
    //fs::read_to_string(path)?;

    Ok(username)
}

#[derive(Debug)]
enum MyCustomError {
    // İçine hem işletim sisteminin hatasını hem de dosya yolunu saklıyoruz
    FileNotFound { path: String, source: io::Error },
    OtherIOErrors(io::Error),
}

fn read_username_from_file() -> Result<String, MyCustomError> {
    let mut username = String::new();

    // 2. Dosya açma hatasını yakalayıp kendi özel enum'ımıza sarıyoruz
    let mut file = File::open(&FILE_PATH).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            MyCustomError::FileNotFound {
                path: String::from(FILE_PATH),
                source: err,
            }
        } else {
            MyCustomError::OtherIOErrors(err)
        }
    })?;

    file.read_to_string(&mut username)
        .map_err(MyCustomError::OtherIOErrors)?;

    Ok(username)
}
