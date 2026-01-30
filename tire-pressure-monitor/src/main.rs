use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct tireData {
    sensor_id: String,
    pressure:f32,
}

fn main() {
    // create channel for publisher and subscriber                                                                                                                          
    let (tx, rx)=mpsc::channel();

    let sensors = vec!["left-front", "right-front", "left-rear", "right-rear"];

    let tx_clone = tx.clone();let tx_clone = tx.clone();
    // spawning a thread means creating a new thread for execution concurrently to the main thread running the program
    thread::spawn(move || {
        let mut current_pressure = 31.5;

        loop {
            current_pressure += 0.5;

            for sensor in &sensors {
                let data = tireData {
                    sensor_id: sensor.to_string(),
                    pressure: current_pressure
                };

                if tx_clone.send(data).is_err() {
                    break;
                }

                thread::sleep(Duration::from_secs(1));
            }        
        }
    });

    drop(tx);

    for received in rx {
        println!("[Sensor ID: {}] Pressure: {} PSI", received.sensor_id, received.pressure);
    };
}
