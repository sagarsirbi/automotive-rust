#[derive(Debug)]
enum Gear {
    Park,
    Reverse,
    Neutral,
    Drive,
    Low,
}

struct Car {
    current_gear: Gear,
    speed:u32,
}

impl Car {
    fn new() -> Car {
        Car {
            current_gear: Gear::Park,
            speed: 0,
        }
    }

    fn set_speed(&mut self, new_speed:u32) {
        self.speed = new_speed;
    }

    fn shift_gear(&mut self, new_gear: Gear) {
        match (&new_gear, self.speed) {
            (&Gear::Park, 0) => {
                println!("Shifting to Park.");
            }
            (&Gear::Reverse, 0) => {
                println!("Shifting to Reverse.");
            }
            (&Gear::Drive, s) if s > 0 && !matches!(self.current_gear, Gear::Park) => {
                println!("Shifting to Drive.");
            }       
            (&Gear::Neutral, _) => {
                println!("Shifting to Neutral.");
            }
            _ => {
                println!("Cannot change to {:?} at speed {}. Current gear: {:?}", new_gear, self.speed, self.current_gear);
                return;
            }
        }
        self.current_gear = new_gear;
    }
}

fn main() {
    let mut audi = Car::new();
    println!("Car created. Current gear: {:?}, Speed: {}", audi.current_gear, audi.speed);
    audi.shift_gear(Gear::Drive); 
    audi.set_speed(35);
    println!("Speed set to {}.", audi.speed);
    audi.shift_gear(Gear::Drive);
    println!("Current gear: {:?}, Speed: {}", audi.current_gear, audi.speed);
    audi.shift_gear(Gear::Park);
    println!("Current gear after trying to park: {:?}, Speed: {}", audi.current_gear, audi.speed);
    audi.set_speed(0);
    println!("Car stopped.");
    audi.shift_gear(Gear::Park);
    println!("Final gear: {:?}, Speed: {}", audi.current_gear, audi.speed);
}
