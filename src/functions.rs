/*
 * @Author: vibraion007 dogingate@qq.com
 * @Date: 2022-05-04 12:58:59
 * @LastEditors: vibraion007 dogingate@qq.com
 * @LastEditTime: 2022-05-04 18:58:21
 * @FilePath: \rust\hello\src\functions.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
pub fn run() {
    greeting("hello", "stephen");
    let get_sum = add(3, 5);
    println!("the sum is {:?}", get_sum);

    //closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;j
    println!("sum is {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{},{},nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
