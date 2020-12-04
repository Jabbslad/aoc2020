use crate::util;
use std::collections::HashSet;

/**
 * ThreeSum
*/
pub fn main() {
    let nums = util::nums();
    let mut seen = HashSet::new();
    for x in 0..nums.len() {
        for y in (x + 1)..nums.len() {
            let z = 2020 - nums[x] as i32 - nums[y] as i32;
            if seen.contains(&z) {
                return println!(
                    "{} * {} * {} = {}",
                    nums[x],
                    nums[y],
                    z,
                    nums[x] * nums[y] * z
                );
            } else {
                seen.insert(nums[y] as i32);
            }
        }
    }
}
