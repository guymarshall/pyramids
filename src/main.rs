fn square_pyramid(number: i32) -> i32 {
    number * (number + 1) * ((2 * number) + 1) / 6
}

fn square(number: i32) -> i32 {
    number * number
}

fn main() {
    'outer: for i in 4..100 {
        for j in 4..100 {
            let pyramid_result: i32 = square_pyramid(i);
            let square_result: i32 = square(j);

            println!("{}:{} - pyramid: {}, square: {}", i, j, pyramid_result, square_result);
            if pyramid_result == square_result {
                println!("DONE");
                break 'outer;
            }
        }
    }
}
