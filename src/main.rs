/*
 * @Author: your name
 * @Date: 2022-04-10 12:05:13
 * @LastEditTime: 2022-06-20 18:47:21
 * @LastEditors: vibration007 dogingate@qq.com
 * @Description: 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 * @FilePath: \hello\src\main.rs
 */
// cargo new
// cargo build --release
// cargo build
// cargo run
// use rand::Rng;
// use std::cmp::Ordering;
// use rust_decimal::MathematicalOps;
use std::io;
// use std::num;
fn queryLanesCount() {
    let direction = loop {
        println!("please choose the direction:[1]one way[2]two way");
        let mut s = String::new();
        // println!("the number you guess is {}", guess);
        io::stdin().read_line(&mut s).expect("Error reading");
        //shadow
        let lane_direction: u8 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if lane_direction == 1 || lane_direction == 2 {
            let width = loop {
                println!("please enter the land width(m):");
                let mut s = String::new();
                // println!("the number you guess is {}", guess);
                io::stdin().read_line(&mut s).expect("Error reading");
                //shadow
                let lane_width: f32 = match s.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if lane_width > 0.0 {
                    let mut lanes_count: u8 = 0;
                    if lane_direction == 1 {
                        if lane_width < 7.0 {
                            lanes_count = 1;
                        } else if lane_width < 10.5 {
                            lanes_count = 2;
                        } else if lane_width < 14.0 {
                            lanes_count = 3;
                        } else if lane_width < 17.5 {
                            lanes_count = 4;
                        } else if lane_width < 21.0 {
                            lanes_count = 5;
                        } else if lane_width < 24.5 {
                            lanes_count = 6;
                        } else if lane_width < 28.0 {
                            lanes_count = 7;
                        } else if lane_width < 31.5 {
                            lanes_count = 8;
                        }
                    }
                    if lane_direction == 2 {
                        if lane_width < 14.0 && lane_width >= 7.0 {
                            lanes_count = 2;
                        } else if lane_width < 21.0 {
                            lanes_count = 4;
                        } else if lane_width < 28.0 {
                            lanes_count = 4;
                        } else if lane_width < 35.0 {
                            lanes_count = 8;
                        }
                    }
                    println!("direction is {}, width is {}", lane_direction, lane_width);
                    println!("lanes count is {}", lanes_count);
                    break;
                }
            };
            //query if quit the current query or not
            let ans: bool = queryYesOrNot();
            if ans == true {
                break;
            } else {
                continue;
            }
        }
    };
}
fn queryShockCoefficient() {
    let result = loop {
        println!("please enter the frequency");
        let mut s = String::new();
        // println!("the number you guess is {}", guess);
        io::stdin().read_line(&mut s).expect("Error reading");
        //shadow
        let frequency: f32 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if frequency > 0.0 {
            let mut shock_eff: f32 = 0.0;
            if frequency < 1.5 {
                shock_eff = 0.05;
            } else if frequency < 14.0 {
                shock_eff = 0.1767 * frequency.ln() - 0.0157;
            } else {
                shock_eff = 0.45;
            }
            println!("the shock efficient is {}", shock_eff);
            println!("the (1+miu) is {}", 1.0 + shock_eff);
            println!("the 0.7/(1+mui) is {}", 0.7 / (1.0 + shock_eff));
            println!("the 0.4/(1+mui) is {}", 0.4 / (1.0 + shock_eff));
            //query if quit the current query or not
            let ans: bool = queryYesOrNot();
            if ans == true {
                break;
            } else {
                continue;
            }
        }
    };
}
fn queryYesOrNot() -> bool {
    loop {
        println!("Are you sure to close the query?[y/n]");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Error reading");
        let ans: String = match ans.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let my_vec: Vec<char> = ans.chars().collect();
        if my_vec.len() > 0 && my_vec[0] == 'y' {
            break true;
        } else {
            break false;
        }
    }
}
fn queryGradientTemperature() {
    let structure_type = loop {
        println!("please enter the structure type");
        println!("[1]混凝土铺装");
        println!("[2]50mm沥青混凝土铺装");
        println!("[3]100mm沥青混凝土铺装");
        let mut s = String::new();
        // println!("the number you guess is {}", guess);
        io::stdin().read_line(&mut s).expect("Error reading");
        //shadow
        let s: u8 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if s == 1 || s == 2 || s == 3 {
            match s {
                1 => println!("T1 is {}, T2 is {}", 25, 6.7),
                2 => println!("T1 is {}, T2 is {}", 20, 6.7),
                3 => println!("T1 is {}, T2 is {}", 14, 5.5),
                _ => (),
            }
            //query if quit the current query or not
            let ans: bool = queryYesOrNot();
            if ans == true {
                break;
            } else {
                continue;
            }
        }
    };
}

fn queryGradientTemperature2() {
    let structure_type = loop {
        println!("please enter the temerature 1:");
        let mut s1 = String::new();
        // println!("the number you guess is {}", guess);
        io::stdin().read_line(&mut s1).expect("Error reading");
        //shadow
        let s1: f32 = match s1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("please enter the temerature 2:");
        let mut s2 = String::new();
        // println!("the number you guess is {}", guess);
        io::stdin().read_line(&mut s2).expect("Error reading");
        //shadow
        let s2: f32 = match s2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("please enter the height(mm):");
        let mut s3 = String::new();
        // println!("the number you guess is {}", guess);
        io::stdin().read_line(&mut s3).expect("Error reading");
        //shadow
        let s3: f32 = match s3.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if s1>0.0 & s2>0.0{
            if s3<100{

            }

            //query if quit the current query or not
            let ans: bool = queryYesOrNot();
            if ans == true {
                break;
            } else {
                continue;
            }
        }
    };
}
fn queryCrowdLoad() {
    let result = loop {
        let mut s = String::new();
        let mut w1 = 0.0;
        let mut w2 = 0.0;
        let mut w3 = 0.0;
        let mut wp = 0.0;
        let mut B = 0.0;
        println!("请输入加载长度:");
        io::stdin().read_line(&mut s).expect("Error reading");
        let span: f32 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if span > 0.0 {
            println!("请输入人行道宽度;");
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Error reading");
            let wp: f32 = match s.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if span < 20.0 {
                w1 = 4.5 * (20.0 - wp) / 20.0;
                w2 = 5.0 * (20.0 - wp) / 20.0;
            } else {
                w1 = (4.5 - 2.0 * (span - 20.0) / 80.0) * (20.0 - wp) / 20.0;
                w2 = (5.0 - 2.0 * (span - 20.0) / 80.0) * (20.0 - wp) / 20.0;
            }
            if w1 < 2.4 {
                w1 = 2.4
            }
            if w2 < 2.4 {
                w2 = 2.4
            }
            if wp / 2.0 > 4.0 {
                let mut B = 4.0;
            }
            if span <= 20.0 {
                w3 = 5.0 * (20.0 - B) / 20.0;
            } else {
                w3 = (5.0 - 2.0 * (span - 20.0) / 80.0) * (20.0 - B) / 20.0;
            }
            println!(
                "城市桥梁人群荷载为{}，专用人行桥人群荷载为{},城市人行天桥人群荷载为{}",
                w1, w2, w3
            );
            //query if quit the current query or not
            let ans: bool = queryYesOrNot();
            if ans == true {
                break;
            } else {
                continue;
            }
        };
    };
}
fn queryLongtermGrowthFactor() {
    let mut grade = 0.0;
    let mut f = 0.0;
    let mut flong = 0.0;
    let mut lita = 0.0;
    let result = loop {
        println!("请输入砼等级");
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Error reading");
        let grade: f32 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if grade > 0.0 {
            println!("请输入短期效应组合的挠度值:");
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Error reading");
            let f: f32 = match s.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if grade < 40.0 {
                lita = 1.6;
            } else if grade <= 80.0 {
                lita = 1.55 - 0.0025 * grade;
            } else {
                lita = 1.55;
            }
            flong = f * lita / 0.95;
            println!("长期挠度为{}", flong);
            //query if quit the current query or not
            let ans: bool = queryYesOrNot();
            if ans == true {
                break;
            } else {
                continue;
            }
        }
    };
}
fn queryConcentratedLoadValue() {
    let span = loop {
        println!("请输入桥梁的计算跨径");
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Error reading");
        let span: f32 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if span > 0.0 {
            let mut Pk: f32 = 0.0;
            if span <= 5.0 {
                Pk = 270.0;
            } else if span < 50.0 {
                Pk = 2.0 * (span + 130.0);
            } else {
                Pk = 360.0;
            }
            println!("集中荷载标准值Pk={}", Pk);
            //query if quit the current query or not
            let ans: bool = queryYesOrNot();
            if ans == true {
                break;
            } else {
                continue;
            }
        }
    };
}

fn main() {
    // let x: f32 = 0.2342;
    // println!("abs is {}", x.ln());

    let choose = loop {
        println!("please choose the query");
        println!("[1]桥涵设计车道数");
        println!("[2]冲击系数");
        println!("[3]正温差梯度温度基数");
        println!("[4]人群荷载");
        println!("[5]长期增长系数");
        println!("[6]集中荷载标准值");
        let mut s = String::new();
        // println!("the number you guess is {}", guess);
        io::stdin().read_line(&mut s).expect("Error reading");
        //shadow
        let s: u8 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match s {
            1 => queryLanesCount(),
            2 => queryShockCoefficient(),
            3 => queryGradientTemperature(),
            4 => queryCrowdLoad(),
            5 => queryLongtermGrowthFactor(),
            6 => queryConcentratedLoadValue(),
            _ => continue,
        }
    };
    // let secret_num = rand::thread_rng().gen_range(1..101);
    // println!("the number is {}", secret_num);
}
