pub struct Logger {
    messages: std::collections::HashMap<String, i32>,
}

impl Logger {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            messages: std::collections::HashMap::new(),
        }
    }

    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        let old_time = self.messages.entry(message).or_default();
        if timestamp >= *old_time {
            *old_time = timestamp + 10;
            true
        } else {
            false
        }
    }
}
