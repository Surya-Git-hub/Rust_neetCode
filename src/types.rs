pub fn run(){
    let x = 1;
    let y = 90.45;
    let byte_v = 0;
    let bool_v = true;
    let yi:i64 = 45498548054850349;
    let num:i128 = 34234234234234234235345345;
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("max i32: {}",std::i32::MAX);
    println!("max i64: {}",std::i64::MAX);
    println!("max i128: {}",std::i128::MAX);
    println!("{:?}",(x,y,byte_v,bool_v,yi,num,a1,face));

}