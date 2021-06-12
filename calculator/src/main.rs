use std::io;

fn main() {
    println!("Calculator!");
    loop {
        println!("Input operand 1 : ");
        let operand1 = input_operand();
        println!("Input operator : ");
        let operator = input_operator();
        println!("Input operand 2 : ");
        let operand2 = input_operand();

        // calculate(_, operator, _) -> opeator가 fn calculate로 move함 -> 아래에서 사용 불가능
        match calculate(operand1, &operator, operand2) {
            Ok(result) => println!(
                "Result : {} {} {} = {}",
                operand1, operator, operand2, result
            ),
            Err(msg) => println!("calculate error ; {}", msg),
        }
    }
}

// 'static ???
fn calculate(operand1: i32, operator: &str, operand2: i32) -> Result<i32, &'static str> {
    // Cannot use match for String and &str
    if operator == "+" {
        Ok(operand1 + operand2)
    } else if operator == "-" {
        Ok(operand1 - operand2)
    } else if operator == "*" {
        Ok(operand1 * operand2)
    } else if operator == "/" {
        Ok(operand1 / operand2)
    } else {
        Err("Unsupported operator")
    }
}

fn input_operand() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("read_line error");

    buf.trim().parse::<i32>().expect("parse error")
}

fn input_operator() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("read_line error");

    // trim() -> &str -> cannot return &str ?
    buf.trim().to_string()
}
