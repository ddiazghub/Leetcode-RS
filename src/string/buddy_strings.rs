use std::collections::HashSet;

pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    let mut found: HashSet<u8> = HashSet::new();
    let mut dupe = false;
    let mut different: Vec<usize> = Vec::new();

    for (i, (ch1, ch2)) in s.bytes().zip(goal.bytes()).enumerate() {
        if !dupe && !found.insert(ch1) {
            dupe = true;
        }

        if ch1 != ch2 {
            different.push(i);
        }
    }
    
    if different.len() == 2 {
        let (first, second) = (different[0], different[1]);

        s.as_bytes()[first] == goal.as_bytes()[second] && goal.as_bytes()[first] == s.as_bytes()[second]
    } else {
        different.len() == 0 && dupe
    }
}
