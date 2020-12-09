use std::collections::VecDeque;

#[derive(Debug)]
pub struct TreeNode<T: PartialEq> {
    pub data: T,
    pub index: usize,
    pub parent_index: Option<usize>,
    pub children: Vec<usize>,
    pub depth: usize,
    is_root: bool
}

#[derive(Debug)]
pub struct Tree<T: PartialEq> {
    nodes: Vec<TreeNode<T>>,
    pub max_depth: usize
}

impl <T: PartialEq>Tree<T> {
    // Create a new tree with a single root node
    pub fn new(data: T) -> Tree<T> {
        Tree { nodes: vec![TreeNode { data: data, index: 0, parent_index: None, children: Vec::new(), depth: 0, is_root: true }], max_depth: 0 }
    }

    // Return the number of nodes in the tree
    pub fn len(&self) -> usize {
        self.nodes.len()
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

        let new_node_depth = parent.depth + 1;
        if new_node_depth > self.max_depth {
            self.max_depth = new_node_depth;
        }

        // Create and save the new node
        let new_node = TreeNode { data: data, index: new_node_index, parent_index: Some(parent_index), children: Vec::new(), depth: new_node_depth, is_root: false };
        self.nodes.push(new_node);
        new_node_index
    }

    // Retrieve a node by index
    pub fn get_by_index(&self, index: usize) -> &TreeNode<T> {
        &self.nodes[index]
    }
    
    // Find the index of the root node of this tree
    // Defaults to the first node in the tree if no node has been marked as root
    pub fn find_root_index(&self) -> usize {
        let mut frontier = VecDeque::new();
        frontier.push_back(0);

        while !frontier.is_empty() {
            // Breadth-first traversal
            let index = frontier.pop_front().unwrap();
            let node = &self.nodes[index];

            // Add children to be searched next
            for child in &node.children {
                frontier.push_back(*child);
            }

            // Found the root node
            if node.is_root {
                return node.index;
            }
        }

        // Assume the first node is the root node of the tree
        0
    }
}
