
use std::fs;
use std::fs::File;
use std::io::Read;

use crate::{
    program,
};

const FILE: &str = "build/tmp.gb";

#[test]
fn disasm() {
    fs::create_dir_all("build").unwrap();
    program::run().unwrap(); 

    let mut file = File::open(&FILE).unwrap();
    let metadata = fs::metadata(&FILE).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).unwrap();


}
