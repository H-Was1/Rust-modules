// #[derive(Debug)]

fn main() {
    let num = Some(-1);
    let plus_one = num
        .map(|num_1| num_1 + 1)
        .filter(|word| word > &10)
        .or_else(|| Some(3));
    println!("{:?}", plus_one.unwrap())
}
