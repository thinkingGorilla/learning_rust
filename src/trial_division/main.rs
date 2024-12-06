fn main() {
    let mut primes = [0; 100];
    get_primes(&mut primes);
    // 배열 타입의 값을 출력하는 경우 {}가 아니라 {:?}를 지정한다.
    println!("{:?}", primes);
}

fn is_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn get_primes(primes: &mut [usize; 100]) {
    let mut i = 2;
    let mut count = 0;
    while count < 100 {
        if is_prime(i as u64) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}
