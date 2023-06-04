pub fn run(nums: Vec<i32>) -> bool {
    let mut map = std::collections::HashSet::new();
    for num in nums{
        if(map.contains(&num)){
            return true;
        }
        map.insert(num);
    }
    return false;
}
