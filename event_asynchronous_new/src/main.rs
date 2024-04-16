use std::sync::{mpsc,Arc, Mutex};
use std::thread;

struct ResizeEvent {
    width: i32,
    height: i32,
}

type ResizeEventHandler=Arc<dyn Fn(ResizeEvent)->Result<(i32,i32),String>+Send+Sync>;

struct ResizeEventListener {
    events: mpsc::Sender<ResizeEvent>,
}

impl ResizeEventListener {
    fn new(handler:ResizeEventHandler,window:Arc<Mutex<Window>>)->Self {
        let (tx,rx)=mpsc::channel();
        let rx=Arc::new(Mutex::new(rx));
        let handler_clone=Arc::clone(&handler);
        let window_clone=Arc::clone(&window);
        thread::spawn(move || {
            while let Ok(event)=rx.lock().unwrap().recv() {
                let (new_width,new_height)=match handler_clone(event) {
                    Ok((w,h))=>(w,h),
                    Err(_)=>{
                        break;
                    }
                };

                let mut window=window_clone.lock().unwrap();
                window.width=new_width;
                window.height=new_height;

            }
        });

        Self {
            events:tx,
        }
    }

    fn send(&self,event:ResizeEvent) {
        match self.events.send(event) {
            Ok(_)=>{
                println!("Event sent successfully");
            },
            Err(err)=>{
                println!("Failed to send event: {}",err);
            },
        }

    }
}


struct Window {
    listener: Option<ResizeEventListener>,
    title: String,
    width: i32,
    height: i32,
}

impl Window {
    fn new(title:String,width:i32,height:i32,handler:ResizeEventHandler)->Arc<Mutex<Self>> {
        let window=Arc::new(Mutex::new(Self {
            listener:None,
            title,
            width,
            height,
        }));
        window.lock().unwrap().listener=Some(ResizeEventListener::new(handler,Arc::clone(&window)));

        window
    }

    fn resize(&mut self,event:ResizeEvent) {
        if let Some(listener)=&self.listener {
            listener.send(event);
        }
    }

    fn open(&self) {
        println!("Window {} is open with size {}x{}",self.title,self.width,self.height);
    }

    fn close(&self) {
        println!("Window {} is closed",self.title);
    }
}


fn main() {
    let window = Window::new(
        "My Window".to_string(),
        800,
        600,
        Arc::new(|event| {
            println!("Window is resized to {}x{}", event.width, event.height);
            Ok((event.width, event.height))
        }),
    );

    window.lock().unwrap().open();
    window.lock().unwrap().resize(ResizeEvent {
        width: 1024,
        height: 768,
    });
    window.lock().unwrap().resize(ResizeEvent {
        width: 640,
        height: 480,
    });
    thread::sleep(std::time::Duration::from_secs(2));
    window.lock().unwrap().close();
}
