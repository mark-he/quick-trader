
use serde_json::error::Category;

use crate::websocket::Stream;

pub struct PositionStream {
    category: Option<Category>,
}

impl PositionStream {
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

impl From<PositionStream> for Stream {
    fn from(stream: PositionStream) -> Stream {
        if stream.category.is_some() {
            Stream::new(&format!("position.{:?}", stream.category.unwrap()))
        } else {
            Stream::new("position")
        }
    }
}
