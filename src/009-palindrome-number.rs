impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
    
        if x < 0 {
            return false;
        }
        
        if (0..10).contains(&x) {
            return true;
        }
        
        let y = x.to_string();
        
        let mut fp = 0;
        let mut ep = y.len() - 1;
        
        while fp <= ep {
            if y.chars().nth(fp).unwrap() != y.chars().nth(ep).unwrap() {
                return false;
            } 
            fp +=1 ;
            ep -=1 ;
        }
        
        return true;
    }
}

/* 
+------------------------------------+
| No need for the leetcode           |
+------------------------------------+
*/

struct Solution;

fn main(){
    let a = Solution::is_palindrome(122);
    println!("{:?}", a)
}