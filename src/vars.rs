pub fn run(){
    let name = "surya";
    let mut age = 25;
    println!("my name is {} and i am {}",name,age);
    println!("birthday came");
    age =26;
    println!("my name is {} and i am {}",name,age);

    //define constant

    const ID:i32 = 000;
    println!("ID: {}",ID);

    //Assing multiple variables
    let (my_name,my_age)=("brad",26);
    println!("{:?}",(my_name,my_age));
    
}