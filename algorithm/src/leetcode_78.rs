impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(nums: &[i32], result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>) {
            result.push(temp.to_vec());

            for (i, &val) in nums.iter().enumerate() {
                temp.push(val);
                backtracking(&nums[i + 1..], result, temp);
                temp.pop();
            }
        }

        let mut temp = vec![];
        let mut result = vec![];
        backtracking(&nums, &mut result, &mut temp);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let res = Solution::subsets([1, 2, 3].into());
        println!("【 res 】==> {:?}", res);
    }
}

pub struct Solution;
