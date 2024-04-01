use std::thread;
use std::sync::{Arc,Mutex};
fn main() {
    let data = Arc::new(Mutex::new(vec![]));

    let data_clone1=Arc::clone(&data);
    let handle1= thread::spawn(move || {
        let result:i32=(0..1000).sum();
        let mut data = data_clone1.lock().unwrap();
        data.push(result);
    });

    let data_clone2=Arc::clone(&data);
    let handle2= thread::spawn(move || {
        let result:i32=(1..10).product();
        let mut data = data_clone2.lock().unwrap();
        data.push(result);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let data = data.lock().unwrap();
    println!("Result: {:?}", data);
}

