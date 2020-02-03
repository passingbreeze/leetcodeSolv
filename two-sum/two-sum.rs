// impl 키워드는 C++에서 class내에 Method선언부라고 보면 된다. -> implementation
// leetcode에서 rust 답안 제출시 주어지는 template

use std::collections::HashMap; // HashMap 자료구조를 활용하기 위해 선언

impl Solution {
    // 채점시 입력으로 들어오는 데이터는 vector형태로 들어옴, i32는 int 32bit의 약자
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut solv : Vec<i32> = Vec::new(); // 해답을 넣을 Vector공간, vector안에 값을 넣어야하기 때문에 mutable 키워드 필요
        let mut index : i32 = 0;
        let (mut beg, mut end):(i32, i32) = (0, 0);
        let mut mmap : HashMap<i32, Vec<i32>> = HashMap::new();
        // mmap -> key(i32) : vector_i32 형태
    
        while index < (nums.len() as i32) {
            mmap.entry(nums[index as usize]).or_insert(Vec::new()).push(index);
            // hashmap에 key-value로 넣음
            // 만약 입력으로 들어오는 데이터가 [3,2,4]라면
            // key 3이 없으면 empty vector가 value로 입력
            // key 3이 있다면 이미 empty vector가 있으므로 vector안에 값을 삽입
            index = index+1;
            // rust는 for문이 iterator 기반이므로 index값을 다루고 싶다면 while문을 활용해야한다.
        }
        
        while beg < (nums.len() as i32) { 
            // rust에서 standard collection에 해당하는 자료구조에 들어있는 자료수를 구하는 함수인 len()은
            // word size를 다루는 자료형인 usize라는 자료형으로 반환된다. 그러므로 casting이 필요
            let ktarget = target - nums[beg as usize];
            // 주어진 target에서 채점시 입력되는 vector의 beg index에 있는 값을 뺀 나머지를 key로 하여
            // hashmap에 있는지 확인
            if mmap.contains_key(&ktarget){
                // hashmap에서 나머지에 해당되는 key를 찾았는데
                if mmap[&ktarget][0] != beg {
                    // [3,2,4]가 입력되는 경우
                    end = mmap[&ktarget][0];
                    break;
                }
                else if mmap[&ktarget].len() > 1 {
                    // [3,3] 같은 경우가 입력되는 경우
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
