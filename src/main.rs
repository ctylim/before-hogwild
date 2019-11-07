#[macro_use]
extern crate lazy_static;

use rand::Rng;
use std::cell::UnsafeCell;
use std::sync::mpsc;
use std::sync::Arc;

#[derive(Clone)]
struct Hogwild<T>(Arc<UnsafeCell<T>>);

impl<T> Hogwild<T> {
    pub fn new(target: T) -> Hogwild<T> {
        Hogwild(Arc::new(UnsafeCell::new(target)))
    }

    pub fn get(&self) -> &mut T {
        let ptr = self.0.as_ref().get();
        unsafe { &mut *ptr }
    }
}

unsafe impl<T> Send for Hogwild<T> {}
unsafe impl<T> Sync for Hogwild<T> {}

lazy_static! {
    static ref MODEL: Hogwild<Vec<i32>> = { Hogwild::new(vec![0; 500]) };
}

fn update_model(model: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    let r = rng.gen::<usize>() % 500;
    model[r] += 1
}

fn main() {
    let (tx, rx) = mpsc::channel();
    for _ in 0..5000000 {
        let tx = tx.clone();
        rayon::spawn(move || {
            update_model(MODEL.get());
            tx.send(()).unwrap();
        })
    }
    for _ in 0..5000000 {
        rx.recv().unwrap();
    }
    println!("hog: {:?}", MODEL.get());
    let mut sum = 0;
    for i in 0..500 {
        sum += MODEL.get()[i];
    }
    println!("sum: {}", sum);
}
