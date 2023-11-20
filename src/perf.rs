use std::time::Instant;

// fn get_time() -> f32 {
//     SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f32 / 1000.0
// }

pub struct Perf {
    time: f32,
    pub frames: f32,
    start: Instant,
    pub(crate) text: String
}

impl Perf {
    pub fn new() -> Perf {
        Perf {
            time: 0.0,
            frames: 0.0,
            start: Instant::now(),
            text: "[wait]".to_string()
        }
    }
    pub fn frame(&mut self) {
        const INTERVAL: f32 = 1000.0;
        let cur = Instant::now().duration_since(self.start).as_secs_f32();
        self.frames += 1.0;
        if cur - self.time >= INTERVAL / 1000.0 {
            self.text = format!("{:.3} ms/f, {:.3} fps", INTERVAL / self.frames, self.frames * 1000.0 / INTERVAL);
            self.frames = 0.0;
            self.time += INTERVAL / 1000.0;
        }
    }
}