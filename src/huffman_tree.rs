use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
pub struct HuffNode {
    pub symbol: u8,
    pub freq: usize,
}

impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}

impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HuffTreeNode {
    pub data: HuffNode,
    left: Option<Box<HuffTreeNode>>,
    right: Option<Box<HuffTreeNode>>,
}

impl PartialOrd for HuffTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data).map(|c| c.reverse())
    }
}

impl Ord for HuffTreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp(&other.data).reverse()
    }
}

impl HuffTreeNode {
    pub fn new(data: HuffNode) -> Self {
        HuffTreeNode {
            data,
            left: None,
            right: None,
        }
    }

    fn new_branch(a: Self, b: Self) -> Self {
        let branch = HuffTreeNode {
            data: HuffNode {
                freq: a.data.freq + b.data.freq,
                symbol: 0,
            },
            left: Some(Box::new(a)),
            right: Some(Box::new(b)),
        };

        branch
    }
}

#[derive(Debug)]
pub struct HuffTree {
    head: HuffTreeNode,
}

impl HuffTree {
    pub fn new() -> Self {
        HuffTree {
            head: HuffTreeNode::new(HuffNode { freq: 0, symbol: 0 }),
        }
    }

    pub fn from_pqueue(bheap: BinaryHeap<HuffNode>) -> Self {
        let mut pqueue: BinaryHeap<HuffTreeNode> = bheap
            .into_iter()
            .map(|node| HuffTreeNode::new(node))
            .collect();

        while pqueue.len() > 1 {
            let tree = pqueue.pop().unwrap();
            let node = pqueue.pop().unwrap();

            let new_node = HuffTreeNode::new_branch(node, tree);
            pqueue.push(new_node);
        }

        HuffTree {
            head: pqueue.pop().expect("the tree needs a root"),
        }
    }

    pub fn pre_traverse<F: Fn(&HuffTreeNode)>(&self, f: F) {
        HuffTree::pre_traverse_node(&self.head, &f);
    }

    fn pre_traverse_node<F: Fn(&HuffTreeNode)>(node: &HuffTreeNode, f: &F) {
        f(node);

        node.left
            .as_ref()
            .map(|l| HuffTree::pre_traverse_node(l.as_ref(), f));

        node.right
            .as_ref()
            .map(|r| HuffTree::pre_traverse_node(r.as_ref(), f));
    }

    fn search(node: &HuffTreeNode, symbol: u8, code: String) -> Option<String> {
        if node.data.symbol != 0 && node.data.symbol == symbol {
            return Some(code);
        }

        if node.left.is_some() {
            let left = node.left.as_ref().unwrap();
            let code = HuffTree::search(left, symbol, code.clone() + "0");
            if code.is_some() {
                return code;
            }
        }

        if node.right.is_some() {
            let right = node.right.as_ref().unwrap();
            let code = HuffTree::search(right, symbol, code.clone() + "1");
            if code.is_some() {
                return code;
            }
        }

        return None;
    }

    pub fn get_code_map(&self) -> HashMap<u8, String> {
        let mut code_map = HashMap::new();
        for i in 0u8..255u8 {
            let code = HuffTree::search(&self.head, i, String::new());
            if code.is_some() {
                code_map.insert(i, code.unwrap());
            }
        }
        println!();
        code_map
    }
}
