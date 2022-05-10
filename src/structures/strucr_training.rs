use std::io::*;

#[warn(dead_code)]
pub fn struct_training() {
    let time = Treveltime {
        car_speed: input_changes("Enter speed"),
        distance: input_changes("Enter distance"),
        car_color_red: yes_or_no(),
    };
    println!("Trevel time is: ~{:?}h", time.time_to_trevel());


}

#[warn(dead_code)]
#[derive(Debug)]
struct Treveltime {
    car_speed: u32,
    distance: u32,
    car_color_red: bool
}

#[warn(dead_code)]
impl Treveltime {
    fn time_to_trevel(&self) -> f32 {
        let mut double_speed = 1;
        let mut time: f32 = 0.0;
       
        if self.car_color_red == true {
            double_speed = 2;
        }
        
        if self.distance <= 0 {
            println!("We have already arrived at our destination");
            return time;
        }else if self.car_speed <= 0 {
            println!("We need to move if we want to get somewhere");
            return time;
        }else {
            time = self.distance as f32 / (self.car_speed*double_speed) as f32;
        }
        
        return time;
    }
}

#[warn(dead_code)]
fn input_changes(massage: &str) -> u32 {
    println!("{}", massage);
    
    loop {
        let mut number = String::new();
        stdin().read_line(&mut number).expect("failed to read input.");
        
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter positiv number");
                continue
            }
        };
        return number;
    }; 
}

#[warn(dead_code)]
fn yes_or_no() -> bool {
    println!("Is Car red?");
    println!("Entter: yes or no");

    loop {

        let yes = String::from("yes");
        let no = String::from("no");
    
        let mut condition = String::new();
        stdin().read_line(&mut condition).expect("failed to read input.");
        condition = condition.trim().to_string();

        if condition.eq(&yes) {
            return true
        }else if condition.eq(&no) {
            return false
        }else {
            println!("Please enter: yes or no");
            continue;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::*;
    
    #[test]
    fn test_color_car_true() {
        let time = Treveltime {
            car_speed: 20,
            distance: 100,
            car_color_red: true,
        };
        
        let temp_time = time.time_to_trevel();
        assert_eq!(2.5, temp_time);
        
    }
    
    #[test]
    fn test_color_car_false() {
        let time = Treveltime {
            car_speed: 20,
            distance: 100,
            car_color_red: false,
        };

        let temp_time = time.time_to_trevel();
        assert_eq!(5.0, temp_time);
    }

    #[test]
    fn test_random() {
        for _ in 0..=10 {
            let true_false: bool = thread_rng().gen();
            let time = Treveltime {
                car_speed: thread_rng().gen_range(0..1000),
                distance: thread_rng().gen_range(0..1000),
                car_color_red: true_false,
            };

            let mut double_speed = 1;
            if true_false == true {
               double_speed = 2;
            }
            let expected_time: f32 = time.distance as f32 / (time.car_speed*double_speed) as f32;

            let temp_time = time.time_to_trevel();
            assert_eq!(expected_time, temp_time);
        }
    }
}