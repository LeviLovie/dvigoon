use dvigoon::*;

fn main() {
    let mut timer = Timer::Timer::new();
    let mut logger = Logger::Logger::new(true, Logger::FormatTEXT, false, "".to_string(), true, Logger::FormatTEXT);

    timer.Start();
    for i in 0..100000000 {
        let _ = i;
    }
    timer.End();

    logger.LogStr(Logger::LevelWARN , "If zero, then it's optimized out for the release build (try debug build) (that's for example)");
    logger.LogTimerTitled(&timer, "For 10.000.000 iterations");
}
