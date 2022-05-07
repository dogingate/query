/*
 * @Author: vibraion007 dogingate@qq.com
 * @Date: 2022-05-03 22:21:16
 * @LastEditors: vibraion007 dogingate@qq.com
 * @LastEditTime: 2022-05-04 10:20:23
 * @FilePath: \rust\hello\src\strings.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
pub fn run() {
    let hello = String::from("Hello ");
    let mut hello2 = String::from("Hello ");

    println!("{}", hello);
    println!("length is {}", hello.len());
    //push char
    hello2.push('&');
    //push string
    hello2.push_str(" bush");
    println!("{}", hello2);
    //capacity in bytes
    println!("capacity is {}", hello.capacity());
    //check if empty
    println!("is empty: {}", hello.is_empty());
    //contains
    println!("{} contains: {}", hello2, hello);
    //replace
    println!("repalce :{} ", hello2.replace("bush", "trump"));
    //loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }
    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);
    //assertion tesing
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());
}
