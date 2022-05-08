/*
 * @Author: vibraion007 dogingate@qq.com
 * @Date: 2022-05-07 19:01:40
 * @LastEditors: vibraion007 dogingate@qq.com
 * @LastEditTime: 2022-05-07 19:32:59
 * @FilePath: \rust\query\src\rectangle.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#[derive(Debug)]
struct Rectanlge {
    width: u32,
    height: u32,
}
impl Rectanlge {
    fn area(&self) -> u32 {
        let area = self.width * self.height;
        area
    }
    fn canhold(&self, other: &Rectanlge) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
impl Rectanlge {
    fn square(size: u32) -> Rectanlge {
        Rectanlge {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rect1: Rectanlge = Rectanlge {
        width: 10,
        height: 20,
    };
    let rect2: Rectanlge = Rectanlge::square(20);
    println!("the rect is {:?}", rect2);
    println!("the area is {}", rect2.area());
    println!("rect1 can hold rect2 :{}", rect1.canhold(&rect2));
    println!("rect2 can hold rect1 :{}", rect2.canhold(&rect1));
}
