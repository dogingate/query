
/*
 * @Author: vibration007 dogingate@qq.com
 * @Date: 2022-05-06 10:28:10
 * @LastEditors: vibration007 dogingate@qq.com
 * @LastEditTime: 2022-05-06 15:58:02
 * @FilePath: \rust\query\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::collections::HashMap;
fn main() {
    let mut concrete = HashMap::new();
    concrete.insert(String::from("C15"), 1.27);
    concrete.insert(String::from("C20"), 1.54);
    concrete.insert(String::from("C25"), 1.78);
    concrete.insert(String::from("C30"), 2.01);
    concrete.insert(String::from("C35"), 2.20);
    concrete.insert(String::from("C40"), 2.40);
    concrete.insert(String::from("C50"), 2.65);
    concrete.insert(String::from("C55"), 2.74);
    concrete.insert(String::from("C60"), 2.85);
    concrete.insert(String::from("C65"), 2.93);
    concrete.insert(String::from("C70"), 3.00);
    concrete.insert(String::from("C75"), 3.05);
    concrete.insert(String::from("C80"), 3.10);

    let grade = String::from("C80");
    let ftk = concrete.get(&grade);

    match ftk {
        Some(s) => println!("{} ftk= {} MPa", grade, s),
        None => println!("your query is not existed"),
    }
    let grade = String::from("C50");

    // for (k, v) in concrete.iter() {
    for (k, v) in &concrete {
        println!("key:{},value:{}", k, v);
    }

    println!("{:#?}", concrete);
}
