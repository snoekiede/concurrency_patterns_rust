use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone)]
enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Clone)]
struct LogMessage {
    level: LogLevel,
    message: String,
}

impl LogMessage {
    fn new(level: LogLevel, message: &str) -> LogMessage {
        LogMessage {
            level,
            message: message.to_string(),
        }
    }
}

struct ActiveLogger {
    sender: mpsc::Sender<QueueMessage>,
}

enum QueueMessage {
    Run((Box<dyn FnOnce(LogMessage) + Send>, LogMessage)),
    Terminate,
}

impl ActiveLogger {
    fn new() -> ActiveLogger {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                match message {
                    QueueMessage::Run((f, m)) => f(m.clone()),
                    QueueMessage::Terminate => break,
                }
            }
        });
        ActiveLogger { sender }
    }
    fn run<F>(&self, f: F, message: LogMessage)
    where
        F: FnOnce(LogMessage) + Send + 'static,
    {
        match self.sender.send(QueueMessage::Run((Box::new(f), message))) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    fn terminate(&self) {
        println!("Terminating...");
        self.sender.send(QueueMessage::Terminate).unwrap();
    }
}
fn main() {
    let logger = ActiveLogger::new();
    let message = LogMessage::new(LogLevel::Info, "Hello, world!");
    logger.run(
        |mes| {
            println!("{:?}:{}", mes.level, mes.message);
        },
        message,
    );
    let errormessage = LogMessage::new(LogLevel::Error, "Error!");
    logger.run(
        |mes| {
            println!("{:?}:{}", mes.level, mes.message);
        },
        errormessage,
    );
    logger.terminate();
}
