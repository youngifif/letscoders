

// [347] Top K Frequent Elements
// https://leetcode.com/problems/top-k-frequent-elements/description/


 
#[derive(Default, Debug)]
pub struct Solution;

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;



struct MinHeapPair(i32,i32);


impl PartialEq for MinHeapPair {
    fn eq(&self, other: &Self) -> bool {
       return self.1 == other.1
    }
}

impl Eq for MinHeapPair {}

impl PartialOrd for MinHeapPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.1 > self.1 {
            return Some(Ordering::Greater);
        }else if other.1 < self.1 {
            return Some(Ordering::Less);
        }
        
        
        Some(Ordering::Equal)
        
    }
}

impl Ord for MinHeapPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        
        let mut hashmap = HashMap::with_capacity(k as usize); 
        for num in nums.into_iter() { 
            *hashmap.entry(num).or_insert(0) += 1;
        }


        
 
        let mut min_heap= BinaryHeap::with_capacity(k as usize);
         
         for  num in  hashmap.into_iter(){
            let num_key = num.0;
            let num_cnt  = num.1;
            let new_pair = MinHeapPair(num_key,num_cnt);
            min_heap.push(new_pair);
            
            if min_heap.len()  > k as usize {
                min_heap.pop();
            }
            
         }

        let mut res = Vec::with_capacity(k as usize);
        while let Some(pair) =  min_heap.pop() {
            res.push(pair.0);
        }
        res.reverse();
        res
        
    }
}