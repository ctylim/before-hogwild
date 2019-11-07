use rand::Rng;

fn update_model(model: &mut Vec<i64>) {
    for _ in 0..1000 {
        let mut rng = rand::thread_rng();
        let r = rng.gen::<usize>() % 1000;
        model[r] += 1
    }
}
fn main() {
    let k = 1000;
    let n = 10000000;
    let mut model = vec![0; k];
    for _ in 0..n {
        update_model(&mut model);
    }
    println!("hog: {:?}", model);
    let mut sum = 0;
    for i in 0..k {
        sum += model[i];
    }
    println!("sum: {}", sum);
}
