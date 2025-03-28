// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// } 

use std::collections::HashSet;
use std::{cell::RefCell, rc::Rc};

struct FindElements {
    included: HashSet<usize>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![(root, 0)];
        let mut included = HashSet::<usize>::new();

        while let Some((node_option, index)) = stack.pop() {
            if let Some(node_rc) = node_option {
                let node = node_rc.borrow();

                included.insert(index);
                stack.push((node.left.clone(), 2 * index + 1));
                stack.push((node.right.clone(), 2 * index + 2));
            }
        }

        Self { included }
    }

    fn find(&self, target: i32) -> bool {
        self.included.contains(&(target as usize))
    }
}