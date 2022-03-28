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

    pub fn eval(&self) -> f64 {
        let value = match &self.content {
            Value::Operation(op) => match op {
                Op::Add => self.left_child.as_ref().unwrap().eval() + self.right_child.as_ref().unwrap().eval(),
                Op::Sub => &self.left_child.as_ref().unwrap().eval() - &self.right_child.as_ref().unwrap().eval(),
                Op::Mul => &self.left_child.as_ref().unwrap().eval() * &self.right_child.as_ref().unwrap().eval(),
                Op::Div => &self.left_child.as_ref().unwrap().eval() / &self.right_child.as_ref().unwrap().eval(),
            },
            Value::Operand(v) => *v,
        };

        return value;
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