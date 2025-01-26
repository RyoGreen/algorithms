pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, f64)> = position
            .iter()
            .zip(speed.iter())
            .map(|(&p, &s)| (p, (target - p) as f64 / s as f64))
            .collect();
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut result = 0;
        let mut time = 0.0;
        for (_, t) in cars {
            if t > time {
                time = t;
                result += 1;
            }
        }
        result
    }
}
