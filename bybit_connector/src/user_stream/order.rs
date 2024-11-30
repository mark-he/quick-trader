
use serde_json::error::Category;

use crate::websocket::Stream;

pub struct OrderStream {
    category: Option<Category>,
}

impl OrderStream {
    pub fn new() -> Self {
        Self {
            category: None,
        }
    }
    
    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category.to_owned());
        self
    }
}

impl From<OrderStream> for Stream {
    fn from(stream: OrderStream) -> Stream {
        if stream.category.is_some() {
            Stream::new(&format!("order.{:?}", stream.category.unwrap()))
        } else {
            Stream::new("order")
        }
    }
}
