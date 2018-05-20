fn mean(vec: Vec<f64>) -> f64 {
    let (mut total, mut count) = (0.0, 0.0);
    for num in &vec {
        total += num;
        count += 1.0;
    }
    total / count
}

fn main() {
    let v = vec![10.0, 2.0, 3.0, 4.0, 5.0];
    let m = mean(v);
    println!("The mean is {}", m);
}
