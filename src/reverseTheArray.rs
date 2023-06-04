pub fn run(mut test_array: Vec<i32>) -> Vec<i32> {
    let mut start_index = 0;
    let mut end_index = test_array.len() - 1;

    if start_index == end_index {
        return test_array;
    }
    let mut memoryV = 0;
    loop {
        memoryV = test_array[start_index];
        test_array[start_index] = test_array[end_index];
        test_array[end_index] = memoryV;
        start_index += start_index;
        end_index -= end_index;
        if start_index >= end_index {
            break;
        }
    }
    return test_array;
}
