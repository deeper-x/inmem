pub struct Redis {
    pub uri: &'static str,
}

impl Redis {
    pub fn new() -> Redis {
        Redis {
            uri: "redis://127.0.0.1:6379",
        }
    }
}
