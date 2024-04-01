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
        let mut queue=self.queue.lock().unwrap();
        queue.push(car);
        self.guard.notify_one();
    }

    fn get(&self)->ExpensiveCar {
        let mut queue=self.queue.lock().unwrap();
        while queue.is_empty() {
            queue=self.guard.wait(queue).unwrap();
        }
        queue.remove(0)
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
                let expensive_car=guarded_garage.get();
                println!("got a car: {}",expensive_car);
            }

        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
}
