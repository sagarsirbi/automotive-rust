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
        // It's generally safer to check if a gear shift is allowed before changing the gear.
        match (&new_gear, self.speed) {
            // Allow shifting to Park only if speed is 0.
            (&Gear::Park, 0) => {
                println!("Shifting to Park.");
            }
            // Allow shifting to Reverse only if speed is 0.
            (&Gear::Reverse, 0) => {
                println!("Shifting to Reverse.");
            }
            // Allow shifting to Drive if speed is greater than 0, from a non-Park gear.
            (&Gear::Drive, s) if s > 0 && !matches!(self.current_gear, Gear::Park) => {
                println!("Shifting to Drive.");
            }
            // Add other valid gear shifts here, for example:
            (&Gear::Neutral, _) => {
                println!("Shifting to Neutral.");
            }
            _ => {
                // If the shift is not allowed, print an error and don't change the gear.
                println!("Cannot change to {:?} at speed {}. Current gear: {:?}", new_gear, self.speed, self.current_gear);
                return;
            }
        }
        // If the match passes, then we change the gear.
        self.current_gear = new_gear;
    }
}

fn main() {
    let mut audi = Car::new();
    println!("Car created. Current gear: {:?}, Speed: {}", audi.current_gear, audi.speed);

    // Let's try to shift to Drive while parked and at 0 speed.
    // Based on our logic, this should be allowed if we start from Park.
    // Let's adjust the logic to allow shifting from Park to Drive at 0 speed.
    audi.shift_gear(Gear::Drive); 

    audi.set_speed(35);
    println!("Speed set to {}.", audi.speed);

    // Now at speed 35, in Drive.
    audi.shift_gear(Gear::Drive);
    println!("Current gear: {:?}, Speed: {}", audi.current_gear, audi.speed);

    // Try to shift to Park while moving. This should be denied.
    audi.shift_gear(Gear::Park);
    println!("Current gear after trying to park: {:?}, Speed: {}", audi.current_gear, audi.speed);

    // Stop the car and then shift to Park.
    audi.set_speed(0);
    println!("Car stopped.");
    audi.shift_gear(Gear::Park);
    println!("Final gear: {:?}, Speed: {}", audi.current_gear, audi.speed);
}
