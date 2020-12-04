use std::collections::HashSet;

fn two_sum(total: i32, nums: Vec<i32>) -> Result<(i32, i32), String> {
    let mut seen = HashSet::new();
    for num in nums {
        let find = total - num;
        if seen.contains(&find) {
            return Ok((num, find));
        } else {
            seen.insert(num);
        }
    }
    Err("could not find".to_owned())
}

pub fn main() {
    let contents = include_str!("one/input");
    let nums: Vec<i32> = contents
        .lines()
        .map(|line| line.parse().expect("err"))
        .collect();

    match two_sum(2020, nums) {
        Ok((x, y)) => println!("{} * {} = {}", x, y, x * y),
        Err(msg) => println!("{}", msg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_sum_find() {
        assert_eq!(two_sum(2020, vec![1000, 1020]), Ok((1020, 1000)));
    }

    #[test]
    fn two_sum_not_find() {
        assert!(two_sum(2020, vec![1000, 1021]).is_err());
    }
}
