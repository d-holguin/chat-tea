pub struct FpsCounter {
    pub frame_count: u64,
    pub last_tick: std::time::Instant,
    pub fps: u64,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_tick: std::time::Instant::now(),
            fps: 0,
        }
    }
    pub fn tick(&mut self) {
        self.frame_count += 1;
        if self.last_tick.elapsed().as_secs() >= 1 {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_tick = std::time::Instant::now();
        }
    }
}
