use std::cmp;

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode { val, left: None, right: None }
    }

    fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
        match root {
            Some(node) => {
                let left_depth = TreeNode::max_depth(&node.left);
                let right_depth = TreeNode::max_depth(&node.right);
                cmp::max(left_depth, right_depth) + 1
            }
            None => 0,
        }
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));

    println!("Maximum depth of the binary tree: {}", TreeNode::max_depth(&Some(Box::new(root))));
}
