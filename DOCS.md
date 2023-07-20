# **Basic Documentation for the Dvigoon library**

# Utils
The library provides simple utils. Its methods and classes can be found in `dvigoon::utils`

## Logger
Logger is simple class to logging.
### Class
Logger has a class' constructor (`dvigoon::utils::Logger::new()`), what accepts provided fields:
| Name                 | Type    | Description                                                                          |
| -------------------- | ------- | ------------------------------------------------------------------------------------ |
|`outputCMD           `|`bool   `| If true, the logger will print logs into the terminal                                |
|`outputCMDFormat     `|`usize  `| Format of data printing (Formats stored in vars in `dvigoon::Logger`)                |
|`outputFile          `|`bool   `| If true, the logger will print logs into a file                                      |
|`outputFilePath      `|`String `| Path to file where to put logs (needs to be filled, if `outputFile` is true)         |
|`outputFileErase     `|`bool   `| If true, erases the log file with print of first log                                 |
|`outputFileFormat    `|`usize  `| Format of data printing (Formats stored in vars in `dvigoon::Logger`)                |
### Functions
* **LogString(`Level`: `&str`, `Message`: `String`)** - Function found in in `dvigoon::Logger::Logger`. `dvigoon::utils` provides pre-made levels:
    * `LoggerLevelINFO`
    * `LoggerLevelWARN`
    * `LoggerLevelERROR`
    * `LoggerLevelFATAL`
* **LogStr()** - Same as `LogString()`, but takes `message` arg as `&str`
* **LogTimer(`timer_ref`: `&dvigoon::Timer::Timer`)** logs it in milliseconds
* **LogTimerTitled(`timer_ref`: `&dvigoon::Timer::Timer`, `title`: `&str`)** logs it in milliseconds with title
* **LogTimerSecs(`timer_ref`: `&dvigoon::Timer::Timer`)** logs it in seconds
* **LogTimerSecsTitled(`timer_ref`: `&dvigoon::Timer::Timer`, `title`: `&str`)** logs it in seconds with title
### Examples
`main.rs`
```rust
use dvigoon::utils::*;
fn main() {
    let mut logger = Logger::new(true, dvigoon::utils::LoggerFormatTEXT, true, "./log.txt".to_string(), true, dvigoon::utils::LoggerFormatJSON);
    logger.LogString(LoggerLevelINFO, "Hello, world!".to_string());
}
```
`console`
```
2023-07-20T06:34:23.675269+00:00 [INFO] Hello, world!
```
`./log.log`
```json
{"time_stamp": "2023-07-20T06:45:37.262987+00:00", "level": "INFO", "message": "Hello, world!"}
```

## Timer
Timer is simple class to measure time eclipsed for some code, based in `std::Instant`
### Class
Timer has a class' constructor (`dvigoon::utils::Timer::new()`), what accepts provided fields:
| Name                 | Type    | Description                                            |
| -------------------- | ------- | ------------------------------------------------------ |
|`Start               `|`Instant`| Start time                                             |
|`End                 `|`Instant`| End time                                               |
|`DurationMilliSecs   `|`u128   `| Eclipsed time (calculates automaticly) in milliseconds |
### Functions
* **Start()** - Function found in in `dvigoon::Timer::Timer`. Saves start time for duration calculation
* **End()** - Function found in in `dvigoon::Timer::Timer`. Saves end time, and calculates duration in `DurationMilliSecs`
* **DurationMilliSecs() -> `u128`** - Returns eclipsed time between call of `Start()` and `End()` functions in milliseconds
* **DurationSecs() -> `f64`** - Returns  eclipsed time between call of `Start()` and `End()` functions in  seconds
### Examples
`main.rs`
```rust
use dvigoon::utils::*;
fn main() {
    let mut logger = Logger::new(true, dvigoon::utils::LoggerFormatTEXT, false, "".to_string(), true, LoggerFormatTEXT);
    let mut timer = Timer::new();

    timer.Start();
    for i in 0..100000000 {
        let _ = i;
    }
    timer.End();

    logger.LogStr(LoggerLevelWARN, "If result is 0ms, don't use --release (it optimizes the code) (use debug build) (that code is for the example).");
    logger.LogTimerTitled(&timer, "For 10.000.000 times loop");
}
```
`console`
```
2023-07-20T06:55:50.379008+00:00 [WARN] If result is 0ms, don't use --release (it optimizes the code) (use debug build) (that code is for the example).
2023-07-20T06:55:50.379095+00:00 [TIMER] For 10.000.000 times loop: 1173 ms
```