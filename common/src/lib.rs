
pub mod error {
    use std::error::Error;
    use std::fmt;
    
    #[derive(Debug)]
    pub struct AppError {
        pub code: i32,
        pub message: String,
        pub source: Option<Box<dyn Error>>,
    }
    
    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }
    
    impl AppError {
        pub fn new(code: i32, message: &str) -> Self {
            AppError {
                code,
                message: message.to_string(),
                source: None,
            }
        }
        
        pub fn cause(mut self, error: Box<dyn Error>) -> Self {
            self.source = Some(error);
            self
        }
    }
    
    impl Error for AppError {
    }

}

pub mod c {
    use std::ffi::{CStr, CString};
    use std::os::raw::*;
    use encoding_rs::GBK;

    pub fn string_to_c_char<const N: usize>(s: String) -> [c_char; N] {
        let c_str = CString::new(s).unwrap();
        let bytes = c_str.as_bytes_with_nul();
        let mut array: [c_char; N] = [0 as c_char; N];
    
        for (i, &byte) in bytes.iter().enumerate() {
            if i < N {
                array[i] = byte as c_char;
            } else {
                break;
            }
        }
        array
    }

    pub fn c_char_to_string(c_str: *const c_char) -> String {
        let c_str = unsafe { CStr::from_ptr(c_str) };
        let rust_str = c_str.to_str().unwrap().to_owned();
        rust_str
    }

    pub fn c_char_to_gbk_string(c_char_array: *const c_char) -> String {
        unsafe {
            let c_str = CStr::from_ptr(c_char_array);
            let gbk_bytes = c_str.to_bytes();
            GBK.decode(gbk_bytes).0.to_string()
        }
    }
}

pub mod msmc {
    use std::{fmt::Debug, marker::PhantomData, thread, time::Duration};
    use crossbeam::{channel::{unbounded, Receiver, Sender}, select};

    pub type Rx<T> = Receiver<Option<T>>;
    pub type Tx<T> = Sender<Option<T>>;

    pub struct Spout<T: EventTrait> {
        filter: Option<Box<dyn Fn(&T)->bool>>,
        sender: Tx<T>,
    }

    unsafe impl <T: EventTrait> Sync for Spout<T> {
    }

    unsafe impl <T: EventTrait> Send for Spout<T> {
        
    }

    impl <T: EventTrait> Spout<T> {
        fn new(filter: Option<Box<dyn Fn(&T)->bool>>, sender: Sender<Option<T>>) -> Self {
            Spout {
                filter,
                sender,
            }
        }
    }

    pub trait EventTrait : Clone + Send + Debug {
    
    }

    pub struct Subscription<T: EventTrait> {
        _phantom_data: PhantomData<T>,
        pub receiver: Option<Rx<T>>,
        pub subscribers: Vec<Spout<T>>,
    }

    impl <T : EventTrait> Subscription<T> {
        pub fn top() -> Subscription<T> {
            Subscription {
                _phantom_data: PhantomData::default(),
                receiver: None,
                subscribers: vec![],
            }
        }

        pub fn new(rx: Rx<T>) -> Subscription<T> {
            Subscription {
                _phantom_data: PhantomData::default(),
                receiver: Some(rx),
                subscribers: vec![],
            }
        }

        pub fn publish_to_under(&mut self, under: &mut Subscription<T>) {
            let (tx, rx) = unbounded::<Option<T>>();
            self.subscribers.push(Spout::new(None, tx));
            under.receiver = Some(rx);
        }

        pub fn publish_to_under_with_filter(&mut self, under: &mut Subscription<T>, filter: Box<dyn Fn(&T) -> bool>) {
            let (tx, rx) = unbounded::<Option<T>>();
            self.subscribers.push(Spout::new(Some(filter), tx));
            under.receiver = Some(rx);
        }

        pub fn subscribe(&mut self) -> Subscription<T> {
            let (tx, rx) = unbounded::<Option<T>>();
            self.subscribers.push(Spout::new(None, tx));

            Subscription::new(rx)
        }
        
        pub fn subscribe_with_filter<'a, F: 'static>(&mut self, filter: F) -> Subscription<T> 
            where F: Fn(&T) -> bool {
            let (tx, rx) = unbounded::<Option<T>>();
            self.subscribers.push(Spout::new(Some(Box::new(filter)), tx));

            Subscription::new(rx)
        }

        pub fn stream<F>(&self, f: &mut F)
            where F : FnMut(&Option<T>) -> bool {
            loop {
                let ret = self.receiver.as_ref().unwrap().try_recv();
                match ret {
                    Ok(opt) => {
                        let continue_flag = f(&opt);
                        self.send(&opt);

                        if !continue_flag {
                            break;
                        }
                    },
                    Err(_) => {
                        thread::sleep(Duration::from_millis(100));
                    },
                }
            }
        }

        pub fn send(&self, data : &Option<T>) {
            for s in self.subscribers.iter() {
                match &s.filter {
                    Some(f) => {
                        match data.as_ref() {
                            Some(x) => {
                                if f(x) {
                                    let _ = s.sender.send(data.clone());
                                }
                            },
                            None => {
                                let _ = s.sender.send(None);
                            },
                        }
                    },
                    None => {
                        let _ = s.sender.send(data.clone());
                    }
                }
            }
        }

        pub fn recv_timeout<F>(&self, secs: u64, f:&mut F) -> Result<Option<T>, ()>
            where F : FnMut(&Option<T>) {
            let rx = self.receiver.as_ref().unwrap();
            select! {
                recv(rx) -> ret => {
                    let mut data = None;
                    if let Ok(t) = ret {
                        data = t;
                    }
                    f(&data);
                    self.send(&data);
                    Ok(data)
                },
                default(Duration::from_secs(secs)) => {
                    Err(())
                }
            }
        }
    }
}

pub mod thread {
    use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};
    use crossbeam::channel::{unbounded, Receiver, Sender};
    pub type Rx<T> = Receiver<T>;
    pub type Tx<T> = Sender<T>;

    pub struct InteractiveThread {

    }

    pub struct Handler<T> {
        pub join_handler: Arc<Mutex<Option<JoinHandle<T>>>>,
        pub sender: Sender<String>,
    }

    impl InteractiveThread {
        pub fn spawn<F, T>(f: F) -> Handler<T>
        where
            F: FnOnce(Receiver<String>) -> T,
            F: Send + 'static,
            T: Send + 'static, {
            let (sender, receiver) = unbounded::<String>();
            let join_handler = thread::spawn(move || {
                f(receiver)
            });
            Handler {
                join_handler: Arc::new(Mutex::new(Some(join_handler))),
                sender,
            }
        }
    }
}