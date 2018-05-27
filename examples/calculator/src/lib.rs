const OPS: &[char] = &['-', '+', '*', '/'];

pub struct RpnCalculator {
    stack: Vec<f64>,
}

impl RpnCalculator {
    pub fn new() -> RpnCalculator {
        RpnCalculator {
            stack: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.stack.clear();
    }

    pub fn push<S: AsRef<str>>(&mut self, arg: S) {
        let arg = arg.as_ref();
        let first_char = arg.chars().next();

        if arg.chars().count() == 1 && OPS.contains(&first_char.unwrap()) {
            let operator = first_char.unwrap();
            let y = self.remove_last();
            let x = if self.stack.is_empty() { 0f64 } else { self.remove_last() };
            let val = match operator {
                '-' => x - y,
                '+' => x + y,
                '*' => x * y,
                '/' => x / y,
                _ => panic!("unexpected operator: {}", operator),
            };
            self.stack.push(val);
        } else {
            let num = arg.parse::<f64>().unwrap();
            self.stack.push(num);
        }
    }

    fn remove_last(&mut self) -> f64 {
        let last_elem_index = self.stack.len() - 1;
        self.stack.remove(last_elem_index)
    }

    pub fn pi(&mut self) {
        self.stack.push(std::f64::consts::PI);
    }

    pub fn value(&self) -> f64 {
        self.stack.last().unwrap().clone()
    }
}
