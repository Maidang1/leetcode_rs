/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let is_negative = x < 0;
        let mut x = x.abs();
        let mut res = 0;

        while x > 0 {
            res = res * 10 + x % 10;
            x = x / 10;
        }
        if is_negative {
            return -res;
        }

        res
    }
}
// @lc code=end
