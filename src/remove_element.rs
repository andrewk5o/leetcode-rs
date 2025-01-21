// 27. Remove element
pub struct Solution;

impl Solution {
    // pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    //     nums.retain(|&x| x != val);
    //     i32::try_from(nums.len()).unwrap()
    // }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[j] = nums[j];
                i += 1;
            }
        }
        return i32::try_from(i).unwrap();
    }
}