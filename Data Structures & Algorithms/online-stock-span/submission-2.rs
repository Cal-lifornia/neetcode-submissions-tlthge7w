pub struct StockSpanner {
    days: Vec<i32>,
}

impl StockSpanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { days: vec![] }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        self.days.push(price);
        let mut current_days = 0;
        for day in self.days.iter().rev() {

            if day <= &price {
                current_days += 1;
            } else {
                break;
            }
        }
        current_days
    }
}

