use utils::node::*;

pub mod utils;

const KNOWN_OPERATIONS: [char; 4] = ['+', '-', '*', '/'];

fn main() {
    let mut tree = Node::new(Value::Operation(Op::Add));
    tree.insert_left(Value::Operand(12.0));
    tree.insert_right(Value::Operand(2.0));

    let parsed_str = parse("2*(3+4)".to_owned());
    dbg!(parsed_str);
}

fn parse(expr: String) -> Node {

    let mut expr = expr;

    while expr.starts_with('(') && expr.ends_with(')') {
        let old_expr = expr.clone();
        expr = String::from(&expr[1..(expr.len() - 1)]);
        if expr.find('(') > expr.find(')') {
            expr = old_expr;
            break;
        }
    }

    let mut tree: Node = Node::new(Value::Operand(0.0));

    if expr.contains(|c| { KNOWN_OPERATIONS.contains(&c) } ) {
        // Let's find the first operation
        let mut parenthesis_count = 0;

        for (i, c) in expr.chars().enumerate() {
            match c {
                '(' => parenthesis_count += 1,
                ')' => parenthesis_count -= 1,
                c => {
                    if parenthesis_count == 0 {
                        if KNOWN_OPERATIONS.contains(&c) {
                            let op = match c {
                                '+' => Op::Add,
                                '-' => Op::Sub,
                                '*' => Op::Mul,
                                '/' => Op::Div,
                                _ => Op::Add, // This should never happen
                            };
                            let (expr_left, expr_right) = expr.split_at(i);
                            let expr_left = expr_left.to_owned();
                            let mut expr_right = expr_right.to_owned();
                            expr_right.remove(0);
                            tree = Node::new(Value::Operation(op));
                            tree.left_child = Some(Box::new(parse(expr_left)));
                            tree.right_child = Some(Box::new(parse(expr_right)));
                        }
                    }
                }
            }
        }
    } else {
        let content: f64 = expr.parse::<f64>().unwrap();
        tree = Node::new(Value::Operand(content));
    }

    tree

}