
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", numbers);
    //get single value
    println!("single value at index 0 is {}", numbers[0]);
    numbers[0] = 2;
    println!("single value at index 0 is {}", numbers[0]);
    //get vector length
    println!("vector length: {}", numbers.len());
    //vectors are stack allocated
    println!("vector occupies {} bytes ", mem::size_of_val(&numbers));
    //Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("silce :{:?}", slice);

    //add to vector
    numbers.push(34);
    numbers.push(898);
    println!("new vector {:?}",numbers);

    //pop off last value
    numbers.pop();
    println!("new vector after pop {:?}",numbers);

    //loop through the vector

    for x in numbers.iter(){
        println!("number in the vector {}",x);
    }
    

}
