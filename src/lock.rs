use rand::Rng;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};

fn update_model(model: &Vec<RwLock<i64>>) {
    for _ in 0..1000 {
        let mut rng = rand::thread_rng();
        let r = rng.gen::<usize>() % 1000;
        let mut m = model[r].write().unwrap();
        *m += 1;
    }
}

fn main() {
    println!("threads: {}", rayon::current_num_threads());
    let k: usize = 1000;
    let n = 10000000;
    let model: Arc<Vec<RwLock<i64>>> = Arc::new((0..k).map(|_| RwLock::new(0)).collect());
    let (tx, rx) = mpsc::channel();
    for _ in 0..n {
        let m = model.clone();
        let tx = tx.clone();
        rayon::spawn(move || {
            update_model(&*m);
            tx.send(()).unwrap();
        })
    }
    for _ in 0..n {
        rx.recv().unwrap();
    }
    println!("model: {:?}", *model);
    let mut sum = 0;
    for i in 0..k {
        sum += *model[i].read().unwrap();
    }
    println!("sum: {}", sum);
}
