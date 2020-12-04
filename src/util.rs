pub fn nums() -> Vec<i32> {
    let contents = include_str!("input");
    contents
        .lines()
        .map(|line| line.parse().expect("err"))
        .collect()
}
