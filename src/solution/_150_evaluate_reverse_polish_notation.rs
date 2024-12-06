#[derive(Default, Debug)]
pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = Vec::new();
        for str in tokens.into_iter() {
            if str == "+" || str == "-" || str == "/" || str == "*" {
                let a = stack.pop().unwrap().parse::<i32>().unwrap();
                let b = stack.pop().unwrap().parse::<i32>().unwrap();

                let res = match str.as_str() {
                    "+" => b + a,
                    "-" => b - a,
                    "*" => b * a,
                    "/" => b / a,

                    _ => unreachable!(),
                };

                stack.push(res.to_string());
            } else {
                stack.push(str);
            }
        }
        return stack[0].parse::<i32>().unwrap();
    }
}
