#![feature(core_intrinsics)]
#![allow(unused)]
#![feature(duration_as_u128)]
use std::collections::HashMap;
use std::str;
use std::cmp::max;
use std::time::Instant;

struct SundaySearcher {
    delta1: Box<[u8; 256]>,
    delta2: Vec<usize>,
    idx: Vec<usize>,
}

fn get_delta1(pat: &str) -> [usize; 256] {
    let mut delta1: [usize; 256] = [0; 256];
    let p = pat.as_bytes();
    for (i, &byte) in p.iter().rev().enumerate(){
        unsafe {
            let entry = delta1.get_unchecked_mut(byte as usize);
            if *entry == 0 {
                *entry = i + 1;
            }
        }
    }
    return delta1;
}

fn kmp(pat: &str) -> Vec<usize> {
    // println!("start");
    let p = pat.as_bytes();
    let n = p.len();
    let mut table = vec![0; n];
    table[0] = 0;
    let mut k = 0;
    for i in 1..n {
        while k > 0 && p[i] != p[k]{
            k = table[k - 1];
            // println!("k: {} ", k);
        }
        if p[i] == p[k] {
            k += 1;
        }
        table[i] = k;
        // println!("here1");

    }
        // println!("here2");
    // println!("{}", pat);
    // table.iter().for_each(|k| print!("{} ", k));
    // println!("done");
    println!("table[0]: {}", table[0]);
    return table;
}

fn get_delta2(pat: &str) -> Vec<usize> {
    let pat = pat.as_bytes();
    let n = pat.len();
    let mut delta2 = vec![1; n];
    let mut shift = 1;
    let check = |i: usize, shift: usize| unsafe { 
        pat.get_unchecked(n - 1 - i) == pat.get_unchecked(n - 1 - i - shift) 
    };
    for i in 0..n {
        shift = 1;
        while shift < n {
            let mut matched = true;
            if (i + shift > n - 1 || check(i, shift) == false) {
                for j in 0..i {
                    if j + shift > n - 1 {
                        break;
                    }
                    if check(j, shift) == false {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    delta2[i] = shift;
                    break;
                }
            }
            shift += 1;
        }
    }
    // delta2.iter().for_each(|k| print!("{} ", k));
    // println!("");
    return delta2;
}

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn naive(text: &String, pat: &String) -> Option<usize>{
    if pat.len() > text.len(){
        return None;
    }
    let n = text.len() - pat.len() + 1;
    (0..n).find(|&i| text[i..].chars().zip(pat.chars()).all(|(a, b)| a == b))
}

fn qskmp(text: &str, pat: &str) -> Option<usize> {
    let t = text.as_bytes();
    let p = pat.as_bytes();
    let delta1 = get_delta1(pat);
    let mut now = Instant::now();
    let delta2 = kmp(pat);
    println!("kmp time: {}", now.elapsed().as_millis());
    now = Instant::now();
    let n = t.len();
    let m = p.len();
    let mut idx = 0;
    unsafe {
        let mut k = 0;
        while idx + m <= n {
            while k < m {
                if t.get_unchecked(idx + k) != p.get_unchecked(k) {
                    break;
                }
                k += 1;
            }
            if k == m {
                println!("search time: {}", now.elapsed().as_millis());
                return Some(idx);
            }
            else if idx + m == n{
                println!("search time: {}", now.elapsed().as_millis());
                return None;
            }
            else {
                let shift1 = *delta1.get_unchecked(*t.get_unchecked(idx + m) as usize);
                if shift1 == 0 {
                    idx += m + 1;
                }
                else {
                    let num_prefix = *delta2.get_unchecked(k - 1);
                    let shift2 = if k > 0 {
                        k - *delta2.get_unchecked(k - 1)
                    }
                    else {
                        1
                    };
                    if shift2 >= shift1 {
                        idx += shift2;
                        k = if k > 0 {
                            *delta2.get_unchecked(k - 1)
                        }
                        else {
                            0
                        };
                    }
                    else {
                        idx += shift1;
                        k = 0;
                    }
                }
            }
        }
        println!("search time: {}", now.elapsed().as_millis());
        return None;
    }
}

fn qsi(text: &str, pat: &str) -> Option<usize> {
    let t = text.as_bytes();
    let p = pat.as_bytes();
    let mut now = Instant::now();
    let delta1 = get_delta1(pat);
    println!("delta1 time: {}", now.elapsed().as_millis());
    now = Instant::now();
    // println!("{}", pat);
    let delta2 = get_delta2(pat);
    println!("delta2 time: {}", now.elapsed().as_millis());
    now = Instant::now();
    let mut idx = 0;
    let n = t.len();
    let m = p.len();
    let mut k = 0;
    while idx <= n - 1 - m + 1 {
        while k < m {
            unsafe {
                if t.get_unchecked(idx + m - 1 - k) != p.get_unchecked(m - 1 - k) {
                    if idx + m == n {
                        println!("search time: {}", now.elapsed().as_millis());
                        return None;
                    }
                    let next_char = *t.get_unchecked(idx + m);
                    let shift1 = *delta1.get_unchecked(next_char as usize);
                    if shift1 == 0 {
                        idx += m + 1;
                        break;
                    }
                    let shift2 = *delta2.get_unchecked(k);
                    if shift2 >= shift1 {
                        if shift2 + k > m {
                            // println!("shift2 + k > m");
                            k = m - shift2;
                        }
                        // println!("shift2 + k <= m");
                        // println!("shift2: {}", shift2);
                        // println!("shift2 + k: {}", shift2 + k);
                        // println!("m: {}", m);
                        idx += shift2;
                    }
                    else {
                        idx += shift1;
                        k = 0;
                    }
                    break;
                }
                k += 1;
            }
        }
        if k == m {
            println!("search time: {}", now.elapsed().as_millis());
            return Some(idx);
        }
    }
    println!("search time: {}", now.elapsed().as_millis());
    return None;
}

fn main() {
    // let text = String::from("你好，爱丝！！爱丽si!爱丽丝!");
    // let pat = String::from("!");
    // // let text = String::from("Hello, Alice!!");
    // // let pat = String::from("Alice");
    // let idx = qsi(text.as_str(), pat.as_str()).unwrap();
    // let len = pat.as_bytes().len();
    // // let res = naive(&text, &pat);
    // let found: &str = str::from_utf8(&(text.as_bytes())[idx..idx + len]).unwrap();
    // println!("{:?}", found);


    let s = "cvijwoekflsdfjlkwenfasdfj".repeat(300);
    // let pat = s.clone().repeat(1) + "bbb";
    // let pat = format!("{}{}", "bbb", s);
    let pat = format!("{}{}", "caaa", s.clone().repeat(1));
    // let pat = pat.repeat(3);
    // let pat = "abba".to_string() + s.clone().as_str();
    let text = s.clone().repeat(10000) + pat.as_str();

    let mut now = Instant::now();
    let idx2 = text.as_str().find(pat.as_str());
    println!("find: {}", now.elapsed().as_millis());

    now = Instant::now();
    let idx1 = qskmp(text.as_str(), pat.as_str());
    println!("qskmp: {}", now.elapsed().as_millis());

    now = Instant::now();
    let idx3 = qsi(text.as_str(), pat.as_str());
    println!("qsi: {}", now.elapsed().as_millis());
    println!("idx1: {:?}\nidx2: {:?}\nidx3: {:?}", idx1, idx2, idx3);

    assert!(idx1 == idx2);
    assert!(idx1 == idx3);

}

