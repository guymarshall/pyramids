fn main() {
    let k: i32 = 24;
    let n: i32 = 70;

    let pyramid: i32 = k * (k + 1) * ((2 * k) + 1) / 6;
    let square:i32 = n * n;

    println!("pyramid: {}, square: {}", pyramid, square);
}
