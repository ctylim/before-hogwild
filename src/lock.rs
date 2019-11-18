pub mod config;
use crate::config::Config;
use rand::Rng;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};

fn update_model(model: &Vec<RwLock<i64>>, c: &usize, k: &usize) {
    for _ in 0..*c {
        let mut rng = rand::thread_rng();
        let r = rng.gen::<usize>() % *k;
        let mut m = model[r].write().unwrap();
        *m += 1;
    }
}

fn main() {
    let config = Config::new();
    println!("threads: {}", rayon::current_num_threads());
    let k = config.k;
    let n = config.n;
    let c = config.c;
    let model: Arc<Vec<RwLock<i64>>> = Arc::new((0..k).map(|_| RwLock::new(0)).collect());
    let (tx, rx) = mpsc::channel();
    for _ in 0..n {
        let m = model.clone();
        let tx = tx.clone();
        rayon::spawn(move || {
            update_model(&*m, &c, &k);
            tx.send(()).unwrap();
        })
    }
    for _ in 0..n {
        rx.recv().unwrap();
    }
    // println!("model: {:?}", *model);
    let mut sum = 0;
    for i in 0..k {
        sum += *model[i].read().unwrap();
    }
    println!("sum: {}", sum);
}
