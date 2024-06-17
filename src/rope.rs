/// Enumeration of possible values in a Rope Data Structure. Each node can be either
/// a Leaf Node which can be represented by a String, or an internal Node. In the second
/// case, the Node will have its Leaf Nodes and a weight which represents the length of
/// the left Leaf Node.
#[derive(Debug)]
pub enum Rope {
    /// The sting in case the rope is a Leaf.
    Leaf(String),
    Node {
        /// the size of the left part of the Node.
        weight: usize,
        /// The left Leaf/Node of the Node.
        left: Box<Rope>,
        /// The right Leaf/Node of the Node.
        right: Box<Rope>,
    },
}

impl Rope {
    /// Creation of a new Leaf node in the  binary tree.
    /// # Arguments:
    ///    * `data` -> the string contained in the leaf.
    ///
    /// # Returns:
    ///    A new leaf Node in the binary tree.
    pub fn new_leaf(data: &str) -> Rope {
        Rope::Leaf(data.to_string())
    }

    /// Creation of a new internal Node in the  binary tree.
    /// # Arguments:
    ///    * `left` -> the left part of the internal node.
    ///    * `right` -> the left part of the internal node.
    ///
    /// # Returns:
    ///    A new internal Node in the binary tree.
    pub fn new_node(left: Rope, right: Rope) -> Rope {
        let weight = left.node_length();
        Rope::Node {
            weight,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Calculates the length of the strings containing in a Node. This is a
    /// recursive funtion. The basic logic is that is that each Node can be either
    /// a leaf Node, which means a String or an internal Node that contains futher
    /// nodes or leafs. In the first case, the length of the string is getting
    /// returned. In the second case, the function calls itself in order until
    /// the right Node results into a leaf Node.
    /// # Arguments:
    ///    * `&self` -> each time represent the node whose length is calculated.
    ///
    /// # Returns:
    ///    The total length of the Node.

    pub fn node_length(&self) -> usize {
        match self {
            Rope::Leaf(data) => data.len(),
            Rope::Node { weight, right, .. } => weight + right.node_length(),
        }
    }

    /// Concatenation of two ropes. In case of a Node concatenating with another,
    /// this automatically means that the Node has to be an internal one. So, the
    /// process has to be simple. The creation of an internal node must be done where
    /// the left rope will be the one calling this function and the right one be the
    /// other argument.
    ///
    /// # Arguments:
    ///    *`rope_to_concat` -> The right rope Node that wants to be
    ///         concatenated.
    ///
    /// # Returns:
    ///    A new Rope internal Node.
    pub fn concatenate_ropes(self, rope_to_concat: Rope) -> Rope {
        Rope::new_node(self, rope_to_concat)
    }

    pub fn split_rope(self, index: usize) -> (Rope, Rope) {
        match self {
            Rope::Leaf(data) => {
                let (left_part, right_part) = data.split_at(index);
                (Rope::new_leaf(left_part), Rope::new_leaf(right_part))
            }
            Rope::Node {
                weight,
                left,
                right,
            } => {
                if index < weight {
                    let (l_node_of_left, r_node_of_left) = left.split_rope(index);
                    (l_node_of_left, Rope::new_node(r_node_of_left, *right))
                } else {
                    let (r_node_of_left, r_node_of_right) = right.split_rope(index);
                    (Rope::new_node(*left, r_node_of_left), r_node_of_right)
                }
            }
        }
    }

    pub fn get_a_substring(&self, start: usize, end: usize) -> String {
        // Returns a substring from a Node existing in the tree.
        let mut result = String::new();
        self.collect_substring(start, end, &mut result);
        result
    }

    pub fn collect_substring(&self, start: usize, end: usize, result: &mut String) {
        match self {
            Rope::Leaf(data) => {
                if start < data.len() {
                    result.push_str(&data[start..end.min(data.len())])
                }
            }
            Rope::Node {
                weight,
                left,
                right,
            } => {
                if start < *weight {
                    left.collect_substring(start, end.min(*weight), result);
                }
                if end > *weight {
                    right.collect_substring(
                        start.saturating_sub(*weight),
                        end.saturating_sub(*weight),
                        result,
                    );
                }
            }
        }
    }
}
