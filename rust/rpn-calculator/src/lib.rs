#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for i in inputs {
        match i {
            CalculatorInput::Value(val) => {
                stack.push(*val)
            },
            _ => {
                if stack.len() < 2 {
                    return None;
                } else {
                    let val_one = stack.pop().unwrap();
                    let val_two = stack.pop().unwrap();
                    match i {
                        CalculatorInput::Add => stack.push(val_two + val_one),
                        CalculatorInput::Subtract => stack.push(val_two - val_one),
                        CalculatorInput::Multiply => stack.push(val_two * val_one),
                        CalculatorInput::Divide => stack.push(val_two / val_one),
                        _ => (),
                    }
                }
            },
        }
    }
    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}
