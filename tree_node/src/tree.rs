
/* 子节点 */
pub struct TreeNode<T: Clone> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>
}
impl <T: Clone>TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn from_option_node(val:T, left:Option<Box<TreeNode<T>>>, right: Option<Box<TreeNode<T>>>) -> Self {
        TreeNode {
            val, left, right
        }
    }

    pub fn from(val:T, left: T, right: T) -> TreeNode<T> {
        Self {
            val,
            left: Some(Box::new(TreeNode::new(left))),
            right: Some(Box::new(TreeNode::new(right))),
        }
    }

    // pub fn clone(&self) -> TreeNode<T> {
    //     let left: Option<Box<TreeNode<T>>> = {
    //         if self.left.is_none() {
    //             None
    //         }else {
    //             Some(self.left.unwrap())
    //         }
    //     };
    //     let right: Option<Box<TreeNode<T>>> = {
    //         if self.right.is_none() {
    //             None
    //         }else {
    //             Some(self.right.unwrap())
    //         }
    //     };
    //     let val = self.val;
    //     Self {val, left, right}
    // }

    /* 将自身转化为Box指针 */
    pub fn to_box(self) -> Box<TreeNode<T>> {
        Box::new(self)
    }

    /* 将自身转化为Option */
    pub fn to_option(self) -> Option<Box<TreeNode<T>>> {
        Some(self.to_box())
    }

    pub fn set_left(&mut self, node: Option<Box<TreeNode<T>>>) -> () {
        self.left = node;
    }

    pub fn set_right(&mut self, node: Option<Box<TreeNode<T>>>) -> () {
        self.right = node;
    }

    pub fn get_left(&mut self) -> &Option<Box<TreeNode<T>>> {
        &self.left
    }

    pub fn get_right(&mut self) -> &Option<Box<TreeNode<T>>> {
        &self.right
    }
}
pub struct Tree<T: Clone> {
    pub root: Box<TreeNode<T>>,
    pub len: u32, // 节点个数
    pub depth: u32 // 最大深度
}
pub fn is_none<T>(option: &Option<T>) -> bool {
    match option {
        None => return true,
        _ => false
    }
}
fn build<T: Clone>(nodes: &Vec<Option<T>>, index: usize) -> Option<Box<TreeNode<T>>> {
    if index > nodes.len() {
        return None
    }
    let node_option = &nodes[index as usize];
    match node_option {
        None => return None,
        Some(val) => {
            let mut node = TreeNode::new(val.clone());
            let left_index = 2 * index + 1;
            let right_index = 2 * index + 2;
            /* 设置左节点 */
            if left_index > nodes.len() - 1 {
                return None;
            } else {
                let left_node_option = build(nodes, left_index);
                if left_node_option.is_none() {
                    node.set_left(None);
                }else {
                    let left_node = left_node_option.unwrap();
                    node.set_left(left_node.to_option());
                }
            }
            /* 设置右节点 */
            if right_index > nodes.len() - 1 {
                return None
            } else {
                let right_node_option = build(nodes, right_index);
                if right_node_option.is_none() {
                    node.set_right(None);
                } else {
                    let right_node = right_node_option.unwrap();
                    node.set_right(right_node.to_option());
                }
            }
            return node.to_option();
        }
    }
}
impl <T: Clone> Tree<T> {
    pub fn from(nodes: &Vec<Option<T>>) -> Tree<T> {
        let root = build(nodes, 0).unwrap();
        Tree {
            root, depth: 0, len: 0
        }
    }
}