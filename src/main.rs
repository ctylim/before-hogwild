extern crate ndarray;
extern crate rayon;

pub mod hogwild;
use rayon::prelude::*;
use hogwild::{Hogwild};

fn main() {
    let sz = 50000000;
    let mut a = vec![0; sz];
    for i in 0..sz {
        a[i] = i;
    }
    
    let hog = Hogwild::new(vec![0; 5]);

    a.par_iter().for_each( |x| hog.get()[x % 5 as usize] += 1 );
    println!("{:?}", hog.get());
}
