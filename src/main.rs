fn pyramid(number: i32) -> i32 {
    number * (number + 1) * ((2 * number) + 1) / 6
}

fn square(number: i32) -> i32 {
    number * number
}

fn main() {
    let _k: i32 = 24;
    let _n: i32 = 70;

    'outer: for i in 4..100 {
        for j in 4..100 {
            let pyramid_result: i32 = pyramid(i);
            let square_result: i32 = square(j);

            println!("{}:{} - pyramid: {}, square: {}", i, j, pyramid_result, square_result);
            if pyramid_result == square_result {
                println!("DONE");
                break 'outer;
            }
        }
    }
}
