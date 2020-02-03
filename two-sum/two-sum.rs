use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut solv : Vec<i32> = Vec::new();
        let mut index : i32 = 0;
        let (mut beg, mut end):(i32, i32) = (0, 0);
        let mut mmap : HashMap<i32, Vec<i32>> = HashMap::new();
    
        while index < (nums.len() as i32) {
            mmap.entry(nums[index as usize]).or_insert(Vec::new()).push(index);
            index = index+1;
        }
        while beg < (nums.len() as i32) {
            let ktarget = target - nums[beg as usize];
            if mmap.contains_key(&ktarget){
                if mmap[&ktarget][0] != beg {
                    end = mmap[&ktarget][0];
                    break;
                }
                else if mmap[&ktarget].len() > 1 {
                    end = mmap[&ktarget][1];
                    break;
                }
            }
            beg = beg+1;
        }
        solv.push(beg);
        solv.push(end);
        solv.sort();
        solv
    }
}