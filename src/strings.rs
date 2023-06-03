pub fn run(){
    let mut hello = String::from("Hello ");
    println!("{:?}",(hello));

    hello.push('a');
    println!("{:?}",(hello));
    hello.push_str("stronaut");
    println!("{:?}",(hello));
    println!("capacity : {:?}",(hello.capacity()));
    println!("is empty : {:?}",(hello.is_empty()));

    println!("does it contain astronaut: {}",hello.contains("astronaut"));

    println!("Replace : {}",hello.replace("astronaut","pilot"));
    println!("lenth of string {}",hello.len());

    //loop through a string 
    for word in hello.split_whitespace(){
        println!("{}",word);
    }
//create string with capacity
let mut test_string = String::with_capacity(10);
test_string.push('a');
test_string.push('b');

println!("{}",test_string);
//ASSERTION OR Equal or comparision
assert_eq!(2,test_string.len());
assert_eq!(10,test_string.capacity());


}