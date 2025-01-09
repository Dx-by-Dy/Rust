fn main() {
    let mut a: &i64 = &mut 5;
    let temp = *a + 1;
    a = &temp;
    let b: i64 = 5;
    println!("{}", a + b);
}
