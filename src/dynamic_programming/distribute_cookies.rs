pub fn distribute(current: usize, cookies: &Vec<i32>, distribution: &mut Vec<i32>) -> i32 {
    if current == cookies.len() {
        return (distribution as &Vec<i32>)
            .into_iter()
            .copied()
            .max()
            .unwrap();
    }

    let bag = cookies[current];
    let mut min = i32::MAX;

    for i in 0..distribution.len() {
        distribution[i] += bag;
        let unfairness = distribute(current + 1, cookies, distribution);
        distribution[i] -= bag;
        min = min.min(unfairness);
    }

    min
}

pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
    let mut distribution = vec![0; k as usize];

    distribute(0, &cookies, &mut distribution)
}

#[cfg(test)]
mod tests {
    use super::distribute_cookies;

    #[test]
    fn test1() {
        let cookies = vec![8,15,10,20,8];
        let result = distribute_cookies(cookies, 2);
        assert_eq!(result, 31);
    }
}
