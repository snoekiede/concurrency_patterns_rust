use std::sync::{Arc,Mutex,Condvar};
use std::thread;
use std::fmt;

struct ExpensiveCar {
    brand: String,
    color: String,
}

impl fmt::Display for ExpensiveCar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "brand: {}, color: {}", self.brand, self.color)
    }
}

impl ExpensiveCar {
    fn new(brand: String, color: String) -> ExpensiveCar {
        ExpensiveCar {
            brand,
            color,
        }
    }
}

struct GuardedGarage {
    queue:Mutex<Vec<ExpensiveCar>>,
    guard:Condvar,
}

impl GuardedGarage {
    fn new()->Self {
        GuardedGarage {
            queue:Mutex::new(Vec::new()),
            guard: Condvar::new(),
        }
    }

    fn park(&self,car:ExpensiveCar) {
        let queue_result=self.queue.lock();
        match queue_result {
            Ok(mut queue) => {
                queue.push(car);
                self.guard.notify_one();
            }
            Err(poisoned) => {
                println!("Error occurred: {:?}",poisoned);
            }
        }
    }

    fn get(&self)->Option<ExpensiveCar> {
        let queue_result=self.queue.lock();
        match queue_result {
            Ok(mut queue) => {
                while queue.is_empty() {
                    queue=self.guard.wait(queue).unwrap();
                }
                Some(queue.remove(0))
            }
            Err(poisoned) => {
                println!("Error occurred: {:?}",poisoned);
                None
            }
        }
    }


}


fn main() {
    let guarded_garage=Arc::new(GuardedGarage::new());
    let producer={
        let guarded_garage=Arc::clone(&guarded_garage);
        thread::spawn(move || {
            for i in 0..10 {
                let expensive_car=ExpensiveCar::new(format!("Car {}",i),format!("red{}",i));
                guarded_garage.park(expensive_car);
            }

        })
    };

    let consumer={
        let guarded_garage=Arc::clone(&guarded_garage);
        thread::spawn(move || {
            for _ in 0..10 {
                if let Some(expensive_car)=guarded_garage.get() {
                    println!("got a car: {}",expensive_car);
                }

            }

        })
    };
    let producer_result=producer.join();
    match producer_result {
        Ok(_) => {
            println!("Producer thread finished");
        }
        Err(e) => {
            println!("Error occurred: {:?}",e);
        }
    }
    let consumer_result=consumer.join();
    match consumer_result {
        Ok(_) => {
            println!("Consumer thread finished");
        }
        Err(e) => {
            println!("Error occurred: {:?}",e);
        }
    }
}
