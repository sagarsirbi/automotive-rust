use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct tireData {
    sensor_id: String,
    pressure:i32,
}

fn main() {
    // create channel for publisher and subscriber                                                                                                                          
    let (tx, rx)=mpsc::channel();

    let sensors = vec!["left-front", "right-front", "left-rear", "right-rear"];

    // spawning a thread means creating a new thread for execution concurrent to the main code
    thread::spawn(move || {
        let mut current_pressure = 31.5;
        let tx = tx.clone()

        loop {
            current_pressure += 0.5;

            for sensor in sensors {
            let data = tireData {
                sensor_id: sensor.to_String(),
                pressure: current_pressure
            };

            tx.send(data);


            }

        
        }
    })

    tx.
}
