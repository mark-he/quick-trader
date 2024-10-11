
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
        
        pub fn cause(&self, error: Box<dyn Error>) -> Self {
            AppError {
                source: Some(error),
                code : self.code,
                message: self.message.clone(),
            }
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
    use std::time::Duration;
    use crossbeam::{channel::{unbounded, Receiver, Sender}, select};
    pub type Rx<T> = Receiver<Option<T>>;
    pub type Tx<T> = Sender<Option<T>>;

    struct Spout<T> {
        filter: Option<Box<dyn Fn(&T)->bool>>,
        sender: Tx<T>,
    }

    unsafe impl <T> Sync for Spout<T> {
    }

    unsafe impl <T> Send for Spout<T> {
        
    }

    impl <T: Clone> Spout<T> {
        fn new(filter: Option<Box<dyn Fn(&T)->bool>>, sender: Sender<Option<T>>) -> Self {
            Spout {
                filter,
                sender,
            }
        }
    }

    pub struct Subscription<T : Clone> {
        receiver: Option<Rx<T>>,
        subscribers: Vec<Spout<T>>,
    }

    impl <T : Clone> Subscription<T> {
        pub fn top() -> Subscription<T> {
            Subscription {
                receiver: None,
                subscribers: vec![],
            }
        }

        pub fn new(rx: Rx<T>) -> Subscription<T> {
            Subscription {
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
        
        pub fn subscribe_with_filter(&mut self, filter: Box<dyn Fn(&T) -> bool>) -> Subscription<T> {
            let (tx, rx) = unbounded::<Option<T>>();
            self.subscribers.push(Spout::new(Some(filter), tx));

            Subscription::new(rx)
        }

        pub fn recv<F>(&self, f:&mut F) -> Option<T>
            where F : FnMut(&Option<T>) {
            let ret = self.receiver.as_ref().unwrap().recv();
            let mut data = None;
            if let Ok(t) = ret {
                data = t;
            }
            f(&data);
            self.send(&data);
            data
        }

        pub fn send(&self, data : &Option<T>) {
            for s in self.subscribers.iter() {
                match &s.filter {
                    Some(f) => {
                        match data {
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