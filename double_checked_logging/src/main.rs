use std::sync::{Arc, Mutex, Once};

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ExpensiveCar {
    name: String,
    price: u32,
}

impl ExpensiveCar {
    fn default() -> Self {
        ExpensiveCar {
            name: "default".to_string(),
            price: 100,
        }
    }
}

#[derive(Debug)]
pub struct Lazy<T> {
    // The `Once` type ensures that the initialization code is run at most once.
    once: Once,
    // We use `Mutex` to safely mutate the value across threads.
    // `Option` is used to fill in the value later since it starts as `None`.
    value: Mutex<Option<Arc<T>>>,
}

impl<T> Lazy<T> {
    // Constructor for the lazy value.
    pub fn new() -> Self {
        Lazy {
            once: Once::new(),
            value: Mutex::new(None),
        }
    }

    // This function provides access to the lazily initialized value.
    pub fn get_or_init<F>(&self, init: F) -> Arc<T>
    where
        F: FnOnce() -> T,
    {
        // Fast path: if the value is already initialized, return it.
        // We do this by attempting to lock the mutex and seeing if the value is present.
        if let Some(value) = self.value.lock().unwrap().as_ref() {
            return value.clone();
        }

        // Slow path: call `call_once` to ensure that the initialization code is run at most once.
        // The provided closure will initialize the value if it hasn't been initialized already.
        self.once.call_once(|| {
            let mut value = self.value.lock().unwrap();
            *value = Some(Arc::new(init()));
        });

        // By this point, the value is guaranteed to be initialized.
        // We lock the mutex again (it's fine if this is blocking, since we know the value is there).
        self.value.lock().unwrap().as_ref().unwrap().clone()
    }
}
fn main() {
    let locked_value = Lazy::new();
    {
        locked_value.get_or_init(|| ExpensiveCar::default());
        let result_value = locked_value.value.lock().unwrap();
        match result_value.as_ref() {
            Some(value) => println!("value is {:?}", value),
            None => println!("value is None"),
        }
    }
}
