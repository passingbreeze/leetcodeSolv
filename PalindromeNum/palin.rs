impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0 {
            false
        }
        else {
            let (mut getx, mut solv) : (i32, i32) = (x, 0);
            while getx > 0 {
                solv = (solv*10) + (getx%10);
                getx /= 10;
            }
            x == solv
        }  
    }
}