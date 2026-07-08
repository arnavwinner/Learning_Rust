#![allow(warnings)]
use text_io::read;


fn int_to_binary(mut n: i32) -> String {

    // 4 -> 0 0 1 -> reverse
    // 100

    // we keep on dividing by two
    let mut s = String::new();
    if n == 0 {
        s += "0";
    }
    while (n > 0) {
        if n % 2 == 0 {
            s += "0 ";
        } else {
            s += "1 ";
        }
        n >>= 1;
    }
    if s != "0" {
        s.pop();
    }
    s.chars().rev().collect()
}

fn main() {


    let p2 = "█ ";
    let p1 = "░ ";

    // we take an input from the user
    let mut N: i32 = read!();;
    // we find the binary value of the input
    let mut answer: String = int_to_binary(N);
    // we showcase the bits
    let mut final_answer = String::new();
    // println!("{answer}");
    let answer_chars: Vec<char> = answer.chars().collect();
    let mut L = answer.len();
    for i in 0..L {
        if answer_chars[i] == '0' {
            final_answer += p1;
        } else if answer_chars[i] == '1' {
            final_answer += p2;
        }
    }
    println!("{answer}");
    println!("{final_answer}");
}
