use std::collections::BinaryHeap;

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
    data: HuffNode,
    left: Box<Option<HuffTreeNode>>,
    right: Box<Option<HuffTreeNode>>,
}

impl PartialOrd for HuffTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Ord for HuffTreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

impl HuffTreeNode {
    pub fn new(data: HuffNode) -> Self {
        HuffTreeNode {
            data,
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    fn new_branch(a: Self, b: Self) -> Self {
        let mut branch = HuffTreeNode {
            data: HuffNode {
                freq: a.data.freq + b.data.freq,
                symbol: 0,
            },
            left: Box::new(None),
            right: Box::new(None),
        };

        if a.data > b.data {
            branch.left = Box::new(Some(a));
            branch.right = Box::new(Some(b));
        } else {
            branch.left = Box::new(Some(b));
            branch.right = Box::new(Some(a));
        }
        branch
    }
}

#[derive(Debug)]
pub struct HuffTree {
    head: Option<HuffTreeNode>,
}

impl HuffTree {
    pub fn new() -> Self {
        HuffTree { head: None }
    }

    pub fn from_pqueue(bheap: BinaryHeap<HuffNode>) -> Self {
        let mut pqueue: BinaryHeap<HuffTreeNode> = bheap
            .into_iter()
            .map(|node| HuffTreeNode::new(node))
            .collect();

        let mut tree = Some(pqueue.pop().expect("pqueue was empty"));
        let mut node = pqueue.pop();
        while tree != None && node != None {
            let new_node = HuffTreeNode::new_branch(tree.unwrap(), node.unwrap());
            node = pqueue.pop();
            tree = pqueue.pop();

            pqueue.push(new_node);
        }

        HuffTree { head: pqueue.pop() }
    }

    // Blank is None value
    //             ┌────────┬──┐
    // s: {[np][p][P][c][c][C][C][c][c][ ][ ]}
    //       └──┼──┘  │  │        │  │
    //          └─────┼──┘        │  │
    //                └───────────┴──┘
    //
    // a: {[p][c][c][c][c]}
    //      └──┼──┘  │  │
    //         └─────┴──┘
    // b: {[P][C][C]}
    //      └──┴──┘
}
