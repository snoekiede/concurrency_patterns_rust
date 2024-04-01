use std::sync::{Arc,RwLock};
use std::thread;

#[derive(Debug)]
struct Version {
    version: String,
    content: String,
}

impl Version {
    fn new(version: String, content: String) -> Version {
        Version {
            version,
            content,
        }
    }
}

fn main() {
    let list = Arc::new(RwLock::new(vec![]));
    let mut handles= vec![];

    for counter in 0..10 {
        let list_clone= Arc::clone(&list);
        let handle = thread::spawn(move || {
            let mut list= list_clone.write().unwrap();
            let version = Version::new(format!("v0.{}", counter), format!("content {}", counter*2));
            list.push(version);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}",*list.read().unwrap());
}



