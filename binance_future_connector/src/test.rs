use std::sync::{Arc, Mutex};
use std::thread;

struct MyStruct {
    closure: Arc<Mutex<Box<dyn Fn() + Send + 'static>>>
}

impl MyStruct {
    fn new(closure: impl Fn() + Send + 'static) -> Self {
        Self {
            closure: Arc::new(Mutex::new(Box::new(closure)))
        }
    }

    fn execute_in_thread(&self) {
        let closure_clone = self.closure.clone();
        thread::spawn(move || {
            let mut closure = closure_clone.lock().unwrap();
            (*closure)();
        });
    }
}

fn main() {
    let my_struct = MyStruct::new(|| {
        println!("Executing closure in thread!");
    });

    my_struct.execute_in_thread();
}