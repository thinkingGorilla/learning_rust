fn main() {
    for y in 1392..1451 {
        print!("서력 {} 년 = ", y);
        if y >= 1419 {
            print_year_to_joseon_era(y, 1419, String::from("세종"));
        } else if y >= 1401 {
            print_year_to_joseon_era(y, 1401, String::from("태종"));
        } else if y >= 1399 {
            print_year_to_joseon_era(y, 1399, String::from("정종"));
        } else if y >= 1392 {
            print_year_to_joseon_era(y, 1392, String::from("태조"));
        }
    }
}

fn print_year_to_joseon_era(year: i32, dynasty_year: i32, dynasty_name: String) {
    if year == dynasty_year {
        println!("{} 원년", dynasty_name);
    } else {
        println!("{} {} 년 ", dynasty_name, year - dynasty_year + 1);
    }
}
