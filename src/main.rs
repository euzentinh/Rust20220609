use std::{io, process::id};
// use rand::Rng;
use regex::Regex;

fn main() {

    // Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? 
    // (yêu cầu đúng thứ tự của các phần tử)
    check_sub_array();

    // Bài tập 2 : Cho 1 chuỗi str Slice như dưới đây. 
    // Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho. 
    count_word();
}

fn check_sub_array() {
let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6,8,10];

    let mut is_child = false;
    let mut i = 0;
    loop {
        if i == org_arr.len() {
            break;
        }
        let mut j = 0;
        loop {
            if org_arr[i + j] != sub_arr[j]{
                break;
            }
            else if j == sub_arr.len() - 1 {
                is_child = true;
                break;
            }
            else {
                j +=1;
            }
        }

        if is_child {
            break;
        }
        i += 1;
        
    }

    println!("sub_arr is org_arr's child: {}", is_child);
}

fn count_word() {
    let paragraph = "This is a regular paragraph with the default style of Normal. 
    This is a regular paragraph with the default style of Normal. 
    This is a regular paragraph with the default style of Normal. 
    This is a regular paragraph with the default style of Normal. 
    This is a regular paragraph with the default style of Normal.";

    let mut word = String::new();
    println!("enter word you want to count: ");
    io::stdin().read_line(&mut word).unwrap();

    word.pop();
    println!("counting word: {}", word);

    let mut childgraph = paragraph;
    let mut count = 0;
    loop {
        let result = childgraph.find(&word);
        if result.is_none() {
            break;
        }
        count += 1;
        let idx = result.unwrap();
        childgraph = &childgraph[idx + word.len()..];
    }
    println!("number of word \"{}\" in paragraph : {:?}", word, count);
}

fn find_word_regex() {
    let paragraph = "This is a regular paragraph with the default style of Normal. 
    This is a regular paragraph with the default style of Normal. 
    This is a regular paragraph with the default style of Normal. 
    This is a regular paragraph with the default style of Normal. 
    This is a regular paragraph with the default style of Normal.";

    let mut word = String::new();
    println!("enter regex you want: ");
    io::stdin().read_line(&mut word).unwrap();

    word.pop();
    println!("regex: {}", word);

    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    
    // println!("number of word \"{}\" in paragraph : {:?}", word, count);
}
