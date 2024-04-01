// use std::error::Error;
//
// enum FileState {
//     Open,
//     Closed,
// }
//
//
// struct MemoryFile {
//     name: String,
//     state: FileState,
//
// }
//
// impl MemoryFile {
//     fn new(name: &str) -> MemoryFile {
//         MemoryFile {
//             name: String::from(name),
//             state: FileState::Closed,
//         }
//     }
//     fn open(&mut self) -> Result<(), Box<dyn Error>> {
//         match self.state {
//             FileState::Closed => {
//                 self.state = FileState::Open;
//                 Ok(())
//             },
//             FileState::Open => Err("File must be closed before it can be opened".into()),
//         }
//     }
//     fn close(&mut self) -> Result<(), Box<dyn Error>> {
//         match self.state {
//             FileState::Open => {
//                 self.state = FileState::Closed;
//                 Ok(())
//             },
//             FileState::Closed => Err("File must be open before it can be closed".into()),
//         }
//     }
//     fn read(&self) -> Result<String,Box<dyn Error>> {
//         match self.state {
//             FileState::Open => Ok(self.name.clone()),
//             FileState::Closed => Err("File must be open before it can be read".into()),
//         }
//     }
// }
//
// fn main() {
//     let mut f = MemoryFile::new("my_file.txt");
//     match f.open() {
//         Ok(_) => {
//             println!("File opened successfully");
//             match f.read() {
//                 Ok(contents) => println!("File contents: {}", contents),
//                 Err(e) => println!("Error reading file: {}", e),
//             }
//             match f.close() {
//                 Ok(_) => println!("File closed successfully"),
//                 Err(e) => println!("Error closing file: {}", e),
//             }
//         }
//
//         Err(e) => println!("Error opening file: {}", e),
//     }
//
// }

struct OpenState;
struct ClosedState;


struct MemoryFile<State> {
    name: String,
    state: std::marker::PhantomData<State>,
}


impl MemoryFile<OpenState> {
    fn close(&self) -> MemoryFile<ClosedState> {
        MemoryFile::<ClosedState> {
            name: self.name.clone(),
            state: std::marker::PhantomData,
        }

    }
    fn read(&self) -> String {
        self.name.clone()
    }
}

impl MemoryFile<ClosedState> {
    fn new(name: &str) -> Self {
        MemoryFile::<ClosedState> {
            name: String::from(name),
            state: std::marker::PhantomData,
        }
    }
    fn open(&self) -> MemoryFile<OpenState> {
        MemoryFile {
            name: self.name.clone(),
            state: std::marker::PhantomData,
        }
    }
}

fn main() {
    let f = MemoryFile::<ClosedState>::new("my_file.txt");
    let f = f.open();
    let contents = f.read();
    println!("File contents: {}", contents);
    let _f = f.close();
}