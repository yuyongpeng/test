#![feature(proc_macro_hygiene)]

use pm::{
    hwx, HelloWorld,
};
use std::sync::Arc;
use std::borrow::BorrowMut;

trait HelloWorld {
    fn hw(&self) -> ();
}

#[derive(HelloWorld)]
struct Test;


fn overwrite<T: Copy>(covarianced: &mut T, short: &mut T) {
    *covarianced = *short;
}


fn main() {
    test_raw_pointers();
//     let input:Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
//     let tokens = format!("\
// {{
//         let mut hm =
//             ::std::collections::HashMap::with_capacity({});
//             {}
//             hm
//         }}
//     ",5, input.iter().map(|n| format!("{}",n)).collect::<String>());
//     print!("{}",tokens);
//     println!("Hello, world!");
//
//     hwx!();
//
//     let t: Test = Test {};
//     t.hw();
}

fn test_raw_pointers() {

    let mut num = 1;
    // 将引用转为裸指针
    let num_raw_point = &mut num as *mut i32;
    unsafe {
        *num_raw_point = 100;
        println!("{} {} {:p} {:?}", num, *num_raw_point, &num, num_raw_point);
        // Output: 100 100 0x8d8c6ff6bc
    }

    let address = num_raw_point as usize;
    // 将一个 usize 对象，转化为 裸指针
    let raw = address as *mut i32;
    unsafe {
        *raw = 200;
        println!("{} {} {:p} {}", num, *num_raw_point, &num, address);
        // Output: 200 200 0x8d8c6ff6bc 607946536636
    }
}

