use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("skii.dat");

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(),
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    let lines = reader.lines();
    let data: Vec<Vec<u32>> = lines.map(|x|
                                        x.unwrap().split(" ").map(|x|
                                                                  x.parse().unwrap()
                                                                 ).collect()
                                       ).collect();
    let height = data[0][0];
    let width = data[0][1];
    let skii_map = &data[1..];

    skii_map[-1];
    for (i, row) in skii_map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            print!("{}", val);
        }
        println!("");
    }
}
