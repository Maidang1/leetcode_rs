/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut res = 0;
        let mut inner_x = x;

        while(inner_x > res) {
            res = res * 10 + inner_x % 10;
            inner_x = inner_x / 10;
        }

        return res == inner_x || res / 10 == inner_x;

    }
}
// @lc code=end

