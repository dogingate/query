/*
 * @Author: vibraion007 dogingate@qq.com
 * @Date: 2022-05-03 22:06:05
 * @LastEditors: vibraion007 dogingate@qq.com
 * @LastEditTime: 2022-05-03 22:20:44
 * @FilePath: \rust\hello\src\types.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 * intergers:u8,i8,u16,i16,u32,i32,u64,i64,u128,i128
 * floats:f32,f64
 * characters(char)
 * tuples
 * arrays
 */

pub fn run() {
    //default is i3
    let x = 1;
    //default is f64
    let y = 2.5;
    //add explicit type
    let z: i64 = 2342038402834;
    println!("i32 max is {}", std::i32::MAX);
    println!("i64 max is {}", std::i64::MAX);
    let (name, age) = ("stephen", "40");
    println!("name is {},age is {}", name, age);
    //boolean
    let is_active: bool = false;
    //get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
