use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Debug, Clone)]
struct Stock {
    name: String,
    price: f64,
}

impl Stock {
    fn new(name: &str, price: f64) -> Self {
        Stock {
            name: name.to_string(),
            price,
        }
    }
    fn update_price(&mut self, new_price: f64) {
        self.price = new_price;
    }

    fn get_price(&self) -> f64 {
        self.price
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}
struct Monitor {
    value: Mutex<Stock>,
    stock_signal: Condvar,
}

impl Monitor {
    fn new(initial_value: Stock) -> Self {
        Monitor {
            value: Mutex::new(initial_value.clone()),
            stock_signal: Condvar::new(),
        }
    }

    fn update_price(&self, new_price: f64) {
        let mut stock = self.value.lock().unwrap();
        println!(
            "Updating price from {} to {} for stock {}",
            stock.get_price(),
            new_price,
            stock.get_name()
        );
        stock.update_price(new_price);
        self.stock_signal.notify_one();
    }

    fn wait_for_release(&self) {
        let limit = 120.0;
        let mut stock = self.value.lock().unwrap();
        while stock.get_price() < limit {
            println!(
                "Price is below {}, current is {}, waiting for release",
                limit,
                stock.get_price()
            );
            stock = self.stock_signal.wait(stock).unwrap();
        }
        println!("Price is now above {}", limit);
    }
}

fn main() {
    let monitor = Arc::new(Monitor::new(Stock::new("MSFT", 100.0)));
    let threads: Vec<_> = (0..10)
        .map(|counter| {
            let monitor = monitor.clone();
            thread::spawn(move || {
                monitor.update_price(110.0 + 2.0 * (counter as f64));
            })
        })
        .collect();
    let mut threads = Vec::new();
    for counter in 0..10 {
         let monitor = monitor.clone();
         threads.push(thread::spawn(move || {
             monitor.update_price(110.0 + counter as f64);
         }));
    }

    monitor.wait_for_release();
    for thread in threads {
        thread.join().unwrap();
    }
    monitor.update_price(200.0);
    let final_value = monitor.value.lock().unwrap();
    println!("Stock is now for {:?}", final_value);
}
