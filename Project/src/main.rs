mod leetcode;
mod models;
use crate::leetcode::longestzigzagpathinabinarytree;
use crate::leetcode::maximumwidthofbinarytree;
use crate::models::treenode::TreeNode;
fn main() {
    let mut s = String::from("Hello, world!");

    // 將字符串轉換為字符向量
    let mut chars: Vec<char> = s.chars().collect();

    // 將第一個字符替換為 'X'
    chars[0] = 'X';
    println!("{}", chars[0]);
    // 將字符向量轉換回字符串
    s = chars.iter().collect();

    println!("{}", s);
}



pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.iter().max().unwrap();
    let mut result = vec![false;candies.len()];
    for index in 0..candies.len(){
        if candies[index]+extra_candies>=*max{
            result[index]=true;
        }
    }
    return result;
}

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut index1 = 0;
    let mut index2 = 0;
    let mut result = String::new();

    let word1vec: Vec<char> = word1.chars().collect();
    let word2vec: Vec<char> = word2.chars().collect();
    while index1 < word1.len() || index2 < word2.len() {
        if index1 < word1.len() {
            result.push(word1vec[index1]);
            index1 += 1;
        }
        if index2 < word2.len() {
            result.push(word2vec[index2]);
            index2 += 1;
        }
    }
    return result;
}
