fn main() {
    println!("Hello, world!");

    let x = 6;
    let y = 5;

    let result = x.cmp(&y);
    println!("{:?}", result);   // the :? indicates Trait std::fmt::Debug
    // pretty-print the output with #?
    // 
}
