/*
 * @Author: vibraion007 dogingate@qq.com
 * @Date: 2022-05-04 11:16:50
 * @LastEditors: vibraion007 dogingate@qq.com
 * @LastEditTime: 2022-05-04 11:36:46
 * @FilePath: \rust\hello\src\vectors.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    //add on to vector
    numbers.push(5);
    numbers.push(8);
    numbers.pop();

    //get single val
    println!("single value is :{}", numbers[0]);

    println!("{:?}", numbers);

    //get array length
    println!("Array length is :{}", numbers.len());

    //arrays are stack allocated
    println!("array occpuies {} bytes", mem::size_of_val((&numbers)));
    //get slice
    let slice: &[i32] = &numbers[0..3];
    println!("{:?}", slice);
    //loop through vector values
    for x in numbers.iter() {
        println!("number is {}", x);
    }
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}
