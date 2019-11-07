use rand::Rng;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};

fn update_model(model: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    let r = rng.gen::<usize>() % 500;
    model[r] += 1
}

fn main() {
    let model = Arc::new(RwLock::new(vec![0; 500]));
    let (tx, rx) = mpsc::channel();

    for _ in 0..5000000 {
        let m = model.clone();
        let tx = tx.clone();
        rayon::spawn(move || {
            let mut m = m.write().unwrap();
            update_model(&mut *m);
            tx.send(()).unwrap();
        })
    }
    for _ in 0..5000000 {
        rx.recv().unwrap();
    }
    let m = model.read().unwrap();
    println!("model: {:?}", *m);
    let mut sum = 0;
    for i in 0..500 {
        sum += &m[i];
    }
    println!("sum: {}", sum);
}
