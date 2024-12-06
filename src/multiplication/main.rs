fn main() {
    let lambda_multiplication = |a: i64, b: i64| -> i64 { a * b };

    let ex1 = lambda_multiplication(3, 5);
    println!("3 * 5 = {}", ex1);
    let ex2 = MULTIPLICATION(3, 5);
    println!("3 * 5 = {}", ex2);
    let ex3 = multiplication(3, 5);
    println!("3 * 5 = {}", ex3);
}

// 함수 포인터 상수
const MULTIPLICATION: fn(i64, i64) -> i64 = multiplication;

fn multiplication(a: i64, b: i64) -> i64 {
    a * b
}
