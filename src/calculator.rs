
enum Operator {
    Sum,
    Subtraction,
    Product,
    Division
}

struct Statement {
    left: i32,
    op: Operator,
    right: i32,
    result: i32
}

impl Statement {
    fn execute(&mut self) {
        match self.op {
            Operator::Sum => self.result = self.left + self.right,
            Operator::Subtraction => self.result = self.left - self.right,
            Operator::Product => self.result = self.left * self.right,
            Operator::Division => self.result = self.left / self.right,
        }
        self.left = self.result;
        self.right = 0;
    }
}

pub fn start() {
    let mut statement = Statement {
        left: 0,
        op: Operator::Sum,
        right: 0,
        result: 0
    };

    loop {
        println!("Type a number: ");
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).expect("Failed to read line");
        if statement.left == 0 {
            statement.left = number.trim().parse::<i32>().expect("Not a number")
        } else {
            statement.right = number.trim().parse::<i32>().expect("Not a number");
            statement.execute();
        }

        println!("Type and operator: + - * / [~ to show result]");
        let mut op = String::new();
        std::io::stdin().read_line(&mut op).expect("Failed to read line");
        match op.trim() {
            "+" => statement.op = Operator::Sum,
            "-" => statement.op = Operator::Subtraction,
            "*" => statement.op = Operator::Product,
            "/" => statement.op = Operator::Division,
            "~" => {
                println!("Result: {}", statement.result);
                break
            }
            _ => panic!("Invalid operator")
        }
    }
}