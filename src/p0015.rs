use super::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        // sort完以后是从小到大排序，比如[-5, -4, -1, 0, 2]
        nums.sort_unstable();

        // 实质上还是三重循环，但是直接三重循环会超时，于是第三重直接binary_search
        'i: for (i, num) in nums.iter().enumerate() {
            for j in i + 1..nums.len() {
                let x = &nums[j + 1..];
                let target = 0 - num - nums[j];
                if target < 0 && nums[j] >= 0 {
                    // 此时前两个数加起来已经大于0了，而且第二个数也大于0，没必要往后找。
                    if j == i + 1 {
                        break 'i;
                    }
                    break;
                }
                match x.binary_search(&target) {
                    Ok(v) if !ret.contains(&vec![*num, nums[j], x[v]]) => {
                        ret.push(vec![*num, nums[j], x[v]])
                    }
                    _ => {}
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_three_sum() {
        let ret1 = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(ret1.len(), 3);

        let ret2 = Solution::three_sum(vec![0, 1, 1]);
        assert!(ret2.is_empty());

        let ret3 = Solution::three_sum(vec![0, 0, 0]);
        assert_eq!(ret3, vec![[0, 0, 0]])
    }
}
