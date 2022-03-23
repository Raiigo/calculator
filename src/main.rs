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

    while expr.starts_with('(') && expr.ends_with(')') { // We have to handle the case were we have a pair of parenthesis from the start to the end
        let old_expr = expr.clone(); // In case the parenthesis are not a pair, we keep track of the old expr
        expr = String::from(&expr[1..(expr.len() - 1)]); // We delete first and last parenthesis
        if expr.find('(') > expr.find(')') { // If true, it means that the parenthesis that we deleted were not a pair, so we set back old expr
            expr = old_expr;
            break;
        }
    }

    let mut tree: Node = Node::new(Value::Operand(0.0)); // Default case (shouldn't be returned except if expr is not well written)

    if expr.contains(|c| { KNOWN_OPERATIONS.contains(&c) } ) { // In this case there is an operation, we have to found were
        // Let's find the first operation
        let mut parenthesis_count = 0; // We want to be out of a parenthesis scope to find the least prioritized operation

        for (i, c) in expr.chars().enumerate() {
            match c {
                '(' => parenthesis_count += 1,
                ')' => parenthesis_count -= 1,
                c => {
                    if parenthesis_count == 0 { // In this case, the char could be the operation we are seeking for
                        if KNOWN_OPERATIONS.contains(&c) { // It is the operation
                            let op = match c { // We match the char to get the Op variant
                                '+' => Op::Add,
                                '-' => Op::Sub,
                                '*' => Op::Mul,
                                '/' => Op::Div,
                                _ => Op::Add, // This should never happen
                            };
                            let (expr_left, expr_right) = expr.split_at(i); // Split expr in two to analyse each one and push it into a child of this node
                            let expr_left = expr_left.to_owned();
                            let mut expr_right = expr_right.to_owned();
                            expr_right.remove(0); // Remove this element, it's the operation sign
                            tree = Node::new(Value::Operation(op)); // Create a node with the operation
                            tree.left_child = Some(Box::new(parse(expr_left))); // Recursively parse left node
                            tree.right_child = Some(Box::new(parse(expr_right))); // Recursively parse right node
                        }
                    }
                }
            }
        }
    } else { // In this cas there is not any operation in the expr, so we can suppose it's an operand and we parse it (recursion basic case)
        let content: f64 = expr.parse::<f64>().unwrap();
        tree = Node::new(Value::Operand(content));
    }

    tree

}