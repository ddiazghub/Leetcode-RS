struct Distribution(u32);

impl Distribution {
    const COOKIE_BIT_SIZE: u32 = 3;

    pub fn child_idx(&self, n: u32) -> usize {
        ((self.0 >> (n * Self::COOKIE_BIT_SIZE)) & 0b111) as usize
    }

    pub fn unfairness(&self, cookies: &Vec<i32>, children: u32) -> u32 {
        let mut counts = vec![0; children as usize];

        for (i, &bag) in cookies.into_iter().enumerate() {
            let child_index = self.child_idx(i as u32);
            counts[child_index as usize] += bag as u32;
        }

        counts
            .into_iter()
            .max()
            .unwrap()
    }

    pub fn child_indices<'a>(&'a self, cookie_count: u32) -> impl Iterator<Item = usize> + 'a {
        (0..cookie_count)
            .map(|i| self.child_idx(i))
    }
}

pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
    let children = k as u32;
    let cookie_count = cookies.len() as u32;

    (0 as u32..1 << (Distribution::COOKIE_BIT_SIZE * cookie_count))
        .filter_map(|bitmask| {
            let distribution = Distribution(bitmask);

            if distribution.child_indices(cookie_count).into_iter().all(|index| (index as u32) < children) {
                Some(distribution.unfairness(&cookies, children))
            } else {
                None
            }
        })
        .min()
        .map(|unfairness| unfairness as i32)
        .unwrap()
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

