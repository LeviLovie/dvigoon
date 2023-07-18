use std::time::Instant;

pub struct Timer {
    pub Start: Instant,
    pub End: Instant,
    pub DurationMilliSecs: u128,
}
impl Timer {
    pub fn new() -> Timer {
        return Timer {
            Start: Instant::now(),
            End: Instant::now(),
            DurationMilliSecs: 0,
        }
    }

    pub fn Start(&mut self) {self.Start = Instant::now();}
    pub fn End(&mut self) {
        self.End = Instant::now();
        self.DurationMilliSecs = self.End.duration_since(self.Start).as_millis();
    }
    pub fn DurationMilliSecs(&self) -> u128 {return self.DurationMilliSecs;}
    pub fn DurationSecs(&self) -> f64 {return self.DurationMilliSecs as f64 / 1000.0;}
}