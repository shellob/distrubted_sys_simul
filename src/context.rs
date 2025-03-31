#[derive(Debug)]
pub struct Context {
    pub local_time: u64,
    pub offset: i64,
    pub sync_replies: Vec<u64>,
    pub sync_start_time: Option<u64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            local_time: 0,
            offset: 0,
            sync_replies: Vec::new(),
            sync_start_time: None,
        }
    }

    pub fn tick(&mut self) {
        self.local_time += 1;
    }

    pub fn adjust_time(&mut self, correction: i64) {
        self.offset += correction;
        let corrected = self.local_time as i64 + correction;
        self.local_time = corrected.max(0) as u64;
    }
}