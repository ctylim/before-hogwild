extern crate ndarray;
extern crate rayon;

pub mod hogwild;
use hogwild::Hogwild;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn input_file(file_name: &str) -> BufReader<File> {
    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            panic!("An error occurred while opening file {}:{}", file_name, e);
        }
    };
    BufReader::new(file)
}

struct A {
    pub v: Hogwild<Vec<i32>>,
}

impl A {
    fn new() -> Self {
        Self {
            v: Hogwild::new(vec![0; 5]),
        }
    }
}

fn main() {

    let reader = input_file("");

    let a = A::new();
    let mut v = vec![0; 100000];

    for (i, _) in reader.lines().enumerate() {
        v[i % 100000 as usize] = i;
        if i % 100000 == 99999 {
            v.par_iter().for_each(|x| a.v.get()[x % 5 as usize] += 1);
        }
    }
    println!("{:?}", a.v.get());
}
