impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        macro_rules! hashmap {
            ($( $key: expr => $val: expr ),*) => {{
                let mut map = ::std::collections::HashMap::new();
                $( map.insert($key, $val); )*
                map
            }}
        }    
        let rintDic = hashmap![
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000    
        ];

        let mut gets : String = s;
        let mut solv : i32 = 0;
        let (mut last, mut lastbef) : (Option<char>, Option<char>);

        while gets.len() > 0 {
            last = gets.pop();
            if gets.len() > 0 {
                lastbef = gets.chars().nth(gets.len()-1);
                // println!("{}, {}", &lastbef.unwrap(), &last.unwrap());
                match (lastbef, last) {
                    (Some('I'), Some('V')) => {
                        solv += 4;
                        gets.pop();
                    },
                    (Some('I'), Some('X')) => {
                        solv += 9;
                        gets.pop();
                    },
                    (Some('X'), Some('L')) => {
                        solv += 40;
                        gets.pop();
                    },
                    (Some('X'), Some('C')) => {
                        solv += 90;
                        gets.pop();
                    },
                    (Some('C'), Some('D')) => {
                        solv += 400;
                        gets.pop();
                    },
                    (Some('C'), Some('M')) => {
                        solv += 900;
                        gets.pop();
                    },
                    _ => solv += rintDic[&last.unwrap()],
                }
            }
            else {
                solv += rintDic[&last.unwrap()];
            }   
        }

        solv
    }
}

/* 속도가 가장 빨랐던 해답 코드

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum: i32 = 0;
        let bytes = s.as_bytes();
        for i in (0..s.len()) {
            let next = if i + 1 == s.len() {
                None 
            } else {
                Some(bytes[i+1])
            };
            sum += match &bytes[i] {
                b'I' => match next {                    
                    Some(b'V') => -1,
                    Some(b'X') => -1,
                    _ => 1
                },
                b'X' => match next {                    
                    Some(b'L') => -10,
                    Some(b'C') => -10,
                    _ => 10
                },
                b'C' => match next {                    
                    Some(b'D') => -100,
                    Some(b'M') => -100,
                    _ => 100
                },
                b'V' => 5,
                b'L' => 50,
                b'D' => 500,
                b'M' => 1000,
                _ => 0
            };
        }
        sum
    }
}
*/