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

/// Node which will be used to trace the path from any tree node to the root.
#[derive(Debug)]
struct TraversalNode {
    /// Current tree node
    treenode: Rc<RefCell<TreeNode>>,
    /// Parent node
    parent: Option<Rc<RefCell<TraversalNode>>>
}

impl TraversalNode {
    pub fn new(treenode: Rc<RefCell<TreeNode>>, parent: Option<Rc<RefCell<TraversalNode>>>) -> Self {
        Self {
            treenode,
            parent
        }
    }
}

/// Traces the path from the target treenode to the root node of the tree. Returns None if the
/// target node can't be reached from the root.
fn trace_path(root: Rc<RefCell<TreeNode>>, target: Rc<RefCell<TreeNode>>) -> Option<Vec<Rc<RefCell<TreeNode>>>> {
    let start = TraversalNode::new(root, None);
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if Rc::ptr_eq(&node.treenode, &target) {
            drop(stack);
            let mut path = vec![Rc::clone(&node.treenode)];
            let mut current = node;

            while let Some(parent) = current.parent {
                path.push(Rc::clone(&parent.borrow().treenode));
                current = Rc::try_unwrap(parent).unwrap().into_inner();
            }

            return Some(path);
        }

        let node = Rc::new(RefCell::new(node));

        if let Some(left) = &node.borrow().treenode.borrow().left {
            stack.push(TraversalNode::new(Rc::clone(left), Some(Rc::clone(&node))));
        };

        if let Some(right) = &node.borrow().treenode.borrow().right {
            stack.push(TraversalNode::new(Rc::clone(right), Some(Rc::clone(&node))));
        };
    }

    None
}

/// Given the root of a binary tree, the value of a target node target, and an integer k, return an array of the values of all nodes that have a distance k from the target node.
/// 
/// You can return the answer in any order.
pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
    match (root, target) {
        (None, _) | (_, None) => vec![],
        (Some(root), Some(target)) => {
            if k == 0 {
                return vec![target.borrow().val];
            }

            match trace_path(root, Rc::clone(&target)) {
                None => vec![],
                Some(path) => {
                    let k = k as usize;
                    let mut nodes = Vec::new();

                    if path.len() > k {
                        nodes.push(path[k].borrow().val);
                    }

                    let mut stack = vec![(target, 0)];

                    for (i, pair) in path.windows(2).take(k - 1).enumerate() {
                        let (child, parent) = (&pair[0], &pair[1]);
                        let p = parent.borrow();

                        if let (Some(left), Some(right)) = (&p.left, &p.right) {
                            if Rc::ptr_eq(child, left) {
                                stack.push((Rc::clone(&right), i + 2));
                            } else {
                                stack.push((Rc::clone(&left), i + 2));
                            }
                        };
                    }

                    while let Some((node, distance)) = stack.pop() {
                        if distance == k {
                            nodes.push(node.borrow().val);
                        } else {
                            if let Some(left) = &node.borrow().left {
                                stack.push((Rc::clone(left), distance + 1));
                            };

                            if let Some(right) = &node.borrow().right {
                                stack.push((Rc::clone(right), distance + 1));
                            };
                        }
                    }

                    nodes
                }
            }
        }
    }
}
