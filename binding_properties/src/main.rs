use std::sync::{Arc, Mutex};
use std::thread;

pub struct Person {
    name: String,
    age: i32,
    observers: Vec<Arc<Mutex<dyn Fn(&str,&str) + Send + Sync>>>,
}

impl Person {
    pub fn new(name: String, age: i32) -> Person {
        Person {
            name,
            age,
            observers: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name.clone();
        self.notify_observers("name",&name.as_str());
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }

    pub fn set_age(&mut self, age: i32) {
        self.age = age;
        self.notify_observers("age",&age.to_string().as_str());
    }

    pub fn subscribe<F>(&mut self, f: F)
    where
        F: Fn(&str,&str) + 'static + Send + Sync,
    {
        self.observers.push(Arc::new(Mutex::new(f)));
    }

    fn notify_observers(&self, property_name: &str,property_value:&str) {
        for observer in &self.observers {
            if let Ok(observer) = observer.lock() {
                observer(property_name,property_value);
            }
        }
    }
}

fn main() {
    let person = Arc::new(Mutex::new(Person::new("Test".to_string(), 55)));
    let person_clone = Arc::clone(&person);
    let handle = thread::spawn(move || {
        let mut person = person_clone.lock().unwrap();
        person.subscribe(move |property_name,property_value| {
            println!("subthread: {} changed to {}", property_name,property_value);
        });
        person.set_name("Jane".to_string());
        person.set_age(21);
    });
    person.lock().unwrap().subscribe(move |property_name,property_value| {
        println!("main thread: {} changed to {}", property_name,property_value);
    });
    person.lock().unwrap().set_name("John".to_string());
    handle.join().unwrap();
}
