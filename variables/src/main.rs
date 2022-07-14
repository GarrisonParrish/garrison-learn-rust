const DEMO_CONSTANT: u16 = 6;

fn main() {
    /*
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */
    let x = 6;

    println!("x = {x}");

    {
        let x = x * 2;
        println!("x = {x}");
    }

    println!("The value of x is: {x}");
    println!("The value of the constant is: {DEMO_CONSTANT}");
}
