fn main() {
    let price = 3950;
    println!("{}원 거스름돈 조합", price);

    for i500 in 1..11 {
        for i100 in 0..4 {
            for i50 in 0..11 {
                let total = i50 * 50 + i100 * 100 + i500 * 500;
                if price == total {
                    println!(
                        "500원 {}개 + 100원 {}개 + 50원 {}개",
                        i500, i100, i50
                    );
                }
            }
        }
    }
}
