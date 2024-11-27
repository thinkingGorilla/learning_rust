fn main() {
    // print_by_nested_loop();
    print_by_list_comprehension();
}

fn print_by_nested_loop() {
    for y in 1..10 {
        for x in 1..10 {
            print!("{:3}", x * y);
        }
        println!();
    }
}

fn print_by_list_comprehension() {
    for y in 1..10 {
        let s = (1..10)
            .map(|x| format!("{:3}", x * y))
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", s);
    }
}
