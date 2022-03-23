#[derive(Debug)]
pub struct Node {
    pub content: Value,
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
}

impl Node {
    pub fn new(content: Value) -> Self {
        Self {
            content: content,
            left_child: None,
            right_child: None,
        }
    }

    pub fn insert_left(&mut self, content: Value) {
        self.left_child = Some(Box::new(Node::new(content)));
    }

    pub fn insert_right(&mut self, content: Value) {
        self.right_child = Some(Box::new(Node::new(content)));
    }
}

#[derive(Debug)]
pub enum Value {
    Operation(Op),
    Operand(f64),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}