use std::collections::{HashSet, VecDeque};

struct SnakeGame {
    height: i32,
    width: i32,
    food: Vec<(i32, i32)>,
    current_food: (i32, i32),
    points: i32,
    snake_head: (i32, i32),
    snake_length: usize,
    prev_pos: Vec<(i32, i32)>,
}

impl SnakeGame {
    pub fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        let mut food = food
            .iter()
            .map(|item| (item[0], item[1]))
            .collect::<Vec<_>>();
        food.reverse();
        let current_food = food.pop().unwrap();
        Self {
            height,
            width,
            food,
            current_food,
            points: 0,
            snake_head: (0, 0),
            snake_length: 0,
            prev_pos: vec![],
        }
    }

    pub fn make_move(&mut self, direction: String) -> i32 {
        self.prev_pos.push(self.snake_head);
        match direction.as_str() {
            "R" => {
                self.snake_head.1 += 1;
                if self.snake_head.1 > self.width - 1 {
                    return -1;
                }
            }
            "L" => {
                self.snake_head.1 -= 1;
                if self.snake_head.1 < 0 {
                    return -1;
                }
            }
            "D" => {
                self.snake_head.0 += 1;
                if self.snake_head.0 > self.height - 1 {
                    return -1;
                }
            }
            "U" => {
                self.snake_head.0 -= 1;
                if self.snake_head.0 < 0 {
                    return -1;
                }
            }
            _ => unreachable!(),
        }

        if self.snake_length > 3 {
            let prev_turns = &self.prev_pos[self.prev_pos.len() - self.snake_length..];
            if prev_turns.iter().any(|body| &self.snake_head == body) {
                return -1;
            }
        }

        if self.snake_head == self.current_food {
            self.points += 1;
            self.snake_length += 1;
            self.current_food = self.food.pop().unwrap_or((-1, -1));
        }

        self.points
    }
}
