use rand::Rng;
pub mod config;
use crate::config::Config;

fn update_model(model: &mut Vec<i64>, c: &usize, k: &usize) {
    for _ in 0..*c {
        let mut rng = rand::thread_rng();
        let r = rng.gen::<usize>() % *k;
        model[r] += 1
    }
}
fn main() {
    let config = Config::new();

    let k = config.k;
    let n = config.n;
    let c = config.c;
    let mut model = vec![0; k];
    for _ in 0..n {
        update_model(&mut model, &c, &k);
    }
    // println!("hog: {:?}", model);
    let mut sum = 0;
    for i in 0..k {
        sum += model[i];
    }
    println!("sum: {}", sum);
}
