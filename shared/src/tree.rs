use std::collections::VecDeque;

pub struct TreeNode<T: PartialEq> {
    pub data: T,
    pub index: usize,
    pub parent_index: Option<usize>,
    pub children: Vec<usize>
}

pub struct Tree<T: PartialEq> {
    nodes: Vec<TreeNode<T>>
}

impl <T: PartialEq>Tree<T> {
    // Create a new tree with a single root node
    pub fn new(data: T) -> Tree<T> {
        Tree { nodes: vec![TreeNode { data: data, index: 0, parent_index: None, children: Vec::new() }] }
    }

    // Find all node indices that contain the specified value
    pub fn find_all(&self, what: T) -> Vec<usize> {
        let mut frontier = VecDeque::new();
        let mut matches = Vec::new();

        // Start at the root node
        frontier.push_back(0);

        while !frontier.is_empty() {
            // Breadth-first traversal
            let index = frontier.pop_front().unwrap();
            let node = &self.nodes[index];

            // Add children to be searched next
            for child in &node.children {
                frontier.push_back(*child);
            }

            // Found a valid node
            if node.data == what {
                matches.push(node.index);
            }
        }

        // All indices of tree nodes with a matching value
        matches
    }

    // Insert data below a parent
    pub fn insert(&mut self, parent_index: usize, data: T) -> usize {
        let new_node_index = self.nodes.len();

        // Register with parent node
        let parent = &mut self.nodes[parent_index];
        parent.children.push(new_node_index);

        // Create and save the new node
        let new_node = TreeNode { data: data, index: new_node_index, parent_index: Some(parent_index), children: Vec::new() };
        self.nodes.push(new_node);
        new_node_index
    }

    // Retrieve a node by index
    pub fn get_by_index(&self, index: usize) -> &TreeNode<T> {
        &self.nodes[index]
    }
}