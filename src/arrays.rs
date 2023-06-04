use std::mem;
pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", numbers);
    //get single value 
    println!("single value at index 0 is {}", numbers[0]);
    numbers[0]=2;
    println!("single value at index 0 is {}", numbers[0]);
    //get array length
    println!("Array length: {}",numbers.len());
    //Arrays are stack allocated 
    println!("Array occupies {} bytes ",mem::size_of_val(&numbers));
    //Get Slice 
    let slice:&[i32] = &numbers[0..2];
    println!("silce :{:?}",slice);


}
