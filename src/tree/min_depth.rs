use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(root) => {
            let mut min_depth = i32::MAX;
            let mut stack = vec![(root, 1)];

            while let Some((node, depth)) = stack.pop() {
                match (&node.borrow().left, &node.borrow().right) {
                    (None, None) => min_depth = min_depth.min(depth),
                    (Some(left), Some(right)) => {
                        stack.push((Rc::clone(left), depth + 1));
                        stack.push((Rc::clone(right), depth + 1));
                    },
                    (Some(left), None) => stack.push((Rc::clone(left), depth + 1)),
                    (None, Some(right)) => stack.push((Rc::clone(right), depth + 1)),
                }
            }

            min_depth
        },
        None => 0
    }
}
