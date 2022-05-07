use std::mem;
pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    //get single val
    println!("single value is :{}", numbers[0]);

    let mut numbers_mut: [i32; 5] = [1, 2, 3, 4, 5];
    numbers_mut[2] = 20;
    println!("{:?}", numbers_mut);

    //get array length
    println!("Array length is :{}", numbers_mut.len());

    //arrays are stack allocated
    println!("array occpuies {} bytes", mem::size_of_val((&numbers_mut)));
    //get slice
    let slice: &[i32] = &numbers_mut[0..3];
    println!("{:?}", slice);
}
