/*
 * @Author: vibraion007 dogingate@qq.com
 * @Date: 2022-05-04 13:55:01
 * @LastEditors: vibraion007 dogingate@qq.com
 * @LastEditTime: 2022-05-04 13:59:38
 * @FilePath: \rust\hello\src\pointers.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
pub fn run() {
    //primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("value are :{:?}", (arr1, arr2));
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("value are :{:?}", (&vec1, &vec2));
}
