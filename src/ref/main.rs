fn main() {
    let mut v = 10;
    set_value(&mut v);
    println!("v = {}, address = {:p}", v, &v);
}

// &mut 가변 참조라는 것을 명시
fn set_value(arg: &mut u32) {
    // 역참조로 값을 변경
    *arg = 100;
}
