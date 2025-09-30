#[allow(dead_code)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

// #[allow(dead_code)]
enum Expression {
    Op {op:Operation, left: Box<Expression>, right:Box<Expression>},
    Value(i64)
}

// #[allow(dead_code)]
impl Expression {
    fn eval(e: Expression) -> i64 {
        match e {
            Expression::Op { op, left, right } => {
                let l = Expression::eval(*left);
                let r = Expression::eval(*right);
                match op {
                    Operation::Add => { l + r}
                    Operation::Sub => { l - r}
                    Operation::Mul => { l * r}
                    Operation::Div => { if r == 0 {0} else {l / r} } // I made it my policy so that if you were to divide any number by 0 it would return 0
                }
            }
            Expression::Value(n) => n
        }
    }
}

fn main() {
    // (1 + 1) * 2, Expected result: 4, Future note: It worked me happy :D
    let test = Expression::Op { 
        op: (Operation::Mul), 
        left: (Box::new(Expression::Op { 
            op: (Operation::Add), 
            left: (Box::new(Expression::Value(1))), 
            right: (Box::new(Expression::Value(1))) 
        })), 
        right: (Box::new(Expression::Value(2))) 
    };
    println!("Result of (1 + 1) * 2: {}", Expression::eval(test))
}
