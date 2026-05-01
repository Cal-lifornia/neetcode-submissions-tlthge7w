impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut intact: Vec<i32> = vec![];
        asteroids.into_iter().for_each(|asteroid| {
            loop {
                if let Some(last) = intact.last() {
                    if last.is_positive() && asteroid.is_negative() {
                        match asteroid.abs().cmp(&last.abs()) {
                            std::cmp::Ordering::Less => {
                                break;
                            }
                            std::cmp::Ordering::Equal => {
                                intact.pop();
                                break;
                            }
                            std::cmp::Ordering::Greater => {
                                intact.pop();
                            }
                        }
                    } else {
                        intact.push(asteroid);
                        break;
                    }
                } else {
                    intact.push(asteroid);
                    break;
                }
            }
        });
        intact
    }
}
