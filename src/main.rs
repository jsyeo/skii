use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Vertex {
    elevation: i32,
    position: (usize, usize)
}

fn main() {
    let path = Path::new("skii.dat");

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(),
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    let lines = reader.lines();
    let data: Vec<Vec<i32>> = lines.map(|x|
                                        x.unwrap().split(" ").map(|x|
                                                                  x.parse().unwrap()
                                                                 ).collect()
                                       ).collect();
    let height = data[0][0];
    let width = data[0][1];
    let skii_map = &data[1..];

    let adjacent_vertices = |(i, j): (i32, i32), vertices: &HashMap<(i32, i32), Vertex>| -> Vec<Vertex> {
        let positions = [((i - 1), j),
                         (i, (j - 1)),
                         ((i + 1), j),
                         (i, (j + 1))];
        let u = vertices.get(&(i, j)).unwrap();
        positions.iter()
                 .filter_map(|position| vertices.get(position))
                 .cloned()
                 .filter(|&Vertex { elevation, .. }| elevation <  u.elevation )
                 .collect()
    };

    let mut vertices = HashMap::new();


    // vertices are indexed by their cell position on the skii map
    for (i, row) in skii_map.iter().enumerate() {
        for (j, elevation) in row.iter().enumerate() {
            let v = Vertex { elevation: *elevation, position: (i, j) };
            vertices.insert((i as i32, j as i32), v);
        }
    }

    for (position, vertex) in &vertices {
        println!("{:?}'s adjacent vertices are:", vertex);
        for n in adjacent_vertices(*position, &vertices).iter() {
            print!("{:?} ", n);
        }
        println!("");
    }
}
