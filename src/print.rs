pub fn run() {
    println!("hello from print rs");

    println!("printing a number : {}", 1);
    //basic formating
    println!("{} is {}", "surya", "awesome");
    //positional argument
    println!("{0} is from {1} and {0} like to code", "surya", "delhi");
    //named args
    println!(
        "{name} likes to play {activity}",
        name = "surya",
        activity = "computer game"
    );
    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic Math expressions
    println!("10+10={}", 10 + 10);
}
