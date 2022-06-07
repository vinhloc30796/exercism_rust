#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
pub fn pop_two(stack: &mut Vec<i32>) -> Option<(i32, i32)> {
    match (stack.pop(), stack.pop()) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                let (a, b) = pop_two(&mut stack)?;
                match (a, b) {
                    (a, b) => stack.push(a + b),
                }
            }
            CalculatorInput::Subtract => {
                let (a, b) = pop_two(&mut stack)?;
                match (a, b) {
                    (a, b) => stack.push(b - a),
                }
            }
            CalculatorInput::Multiply => {
                let (a, b) = pop_two(&mut stack)?;
                match (a, b) {
                    (a, b) => stack.push(a * b),
                }
            }
            CalculatorInput::Divide => {
                let (a, b) = pop_two(&mut stack)?;
                match (a, b) {
                    (a, b) => stack.push(b / a),
                }   
            }
            CalculatorInput::Value(n) => stack.push(*n),
        }
    }
    if stack.len() == 1 {
        Some(stack.pop().unwrap())
    } else {
        None
    }
}
