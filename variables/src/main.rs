fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);
    x = 6;
    println!("The value of x is {}",x);

    let y = 12;
    let y = y * 12;
    println!("The value of y is {}",y);

    let guess:u32 = "121243".parse().expect("Not a valid number");
    println!("The value of guess is {}",guess);

    let u32 = 412_34isize;
    println!("The value of u32 is {}",u32);

    let f64:f64 = 24.42400;
    let ret = f64 / 74 as f64;
    println!("The value of f64/100 is {}",ret);
}
