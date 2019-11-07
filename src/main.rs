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

fn update_model(model: &mut Vec<i64>) {
    for _ in 0..1000 {
        let mut rng = rand::thread_rng();
        let r = rng.gen::<usize>() % 1000;
        model[r] += 1
    }
}

fn main() {
    lazy_static! {
        static ref MODEL: Hogwild<Vec<i64>> = { Hogwild::new(vec![0; 1000]) };
    }
    let k = 1000;
    let n = 10000000;
    let (tx, rx) = mpsc::channel();
    for _ in 0..n {
        let tx = tx.clone();
        rayon::spawn(move || {
            update_model(MODEL.get());
            tx.send(()).unwrap();
        })
    }
    for _ in 0..n {
        rx.recv().unwrap();
    }
    println!("hog: {:?}", MODEL.get());
    let mut sum = 0;
    for i in 0..k {
        sum += MODEL.get()[i];
    }
    println!("sum: {}", sum);
}
