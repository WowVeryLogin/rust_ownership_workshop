pub struct Receiver {
    last_accessed: std::time::Instant,
    last_data_value: usize,
}

impl Receiver {
    pub fn new() -> Self {
        Self {
            last_accessed: std::time::Instant::now(),
            last_data_value: 0,
        }
    }

    pub fn send_data(&mut self, data: &[usize]) {
        assert!(self.last_accessed.elapsed().as_millis() < 15);
        self.last_accessed = std::time::Instant::now();

        data.iter().all(|&v| v > self.last_data_value);
        self.last_data_value = *data.last().unwrap();
    }

    pub fn keepalive(&mut self) {
        assert!(self.last_accessed.elapsed().as_millis() < 15);
        self.last_accessed = std::time::Instant::now();
    }
}
