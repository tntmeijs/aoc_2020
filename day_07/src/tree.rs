mod tree {
    use std::collections::VecDeque;

    #[derive(Clone)]
    struct TreeNode {
        name: String,
        children: Vec<TreeNode>
    }

    fn try_insert(root: &mut TreeNode, new_node: &TreeNode) -> bool {
        let mut queue: VecDeque<&mut TreeNode> = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            // Breadth-first tree traversal
            let node = queue.pop_front().unwrap();
            for child in &mut node.children {
                queue.push_back(child);
            }

            if node.name == new_node.name {
                let children_copy = new_node.children.clone();

                for copy in children_copy {
                    // Somehow stop the borrow checker from complaining...
                }
            }
        }

        true
    }
}
