struct ProductOfNumbers {
    mul: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { mul: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num > 0 {
            self.mul.push((*self.mul.last().unwrap_or(&1)) * num);
        } else {
            self.mul.clear();
            self.mul.push(1);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        if (k as usize) < self.mul.len() {
            self.mul.last().unwrap() / self.mul[self.mul.len() - k as usize - 1]
        } else {
            0
        }
    }
}