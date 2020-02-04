impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut getx : i32 = x;
        let mut solv : i64 = 0;
        let (mut signed, mut digits, mut revsize) : (bool, u32, u32) = (false,0,0);
        let mut rev : Vec<i32> = Vec::new();

        if x < 0 {
            signed = true;
            getx = -getx;
        }
        while getx > 0 {
            rev.push(getx%10);
            getx = getx/10;
        }
        
        revsize = rev.len() as u32;
        while digits < revsize {
            solv = solv + (rev.pop().unwrap() as i64) * (10_i64.pow(digits));
            digits = digits+1;
        }

        if signed {
            solv = -solv;
        }

        if (solv < (i32::min_value() as i64)) || (solv > (i32::max_value() as i64)){
            0
        }
        else {
            solv as i32
        }
    }
}