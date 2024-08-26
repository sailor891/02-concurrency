fn main() {
    let nums = Solution::find_array(vec![5, 2, 0, 3, 1]);
    println!("{:?}", nums);
}
struct Solution {}
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut p = 0;
        pref.iter()
            .map(|&num| {
                let t = num ^ p;
                p = num;
                t
            })
            .collect()
    }
}
