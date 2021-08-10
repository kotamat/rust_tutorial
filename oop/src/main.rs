mod oo_design_patterns;
mod trait_object;

fn main() {
    example_averaged_collection();

    trait_object::main();
    oo_design_patterns::main();
}

fn example_averaged_collection() {
    let mut ac = AveragedCollection {
        list: vec![1, 2, 3],
        average: 2.0,
    };

    ac.add(10);
    println!("average = {}", ac.average())
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            }
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64
    }
}
