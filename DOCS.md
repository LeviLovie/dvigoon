# Basic Documentation for the Dvigoon library
## Logger
The library provides a simple logger, it located in the `dvigoon::Logger`.
It conteins public class Logger (`dvigoon::Logger::Logger`)

Logger has a constructor (`dvigoon::Logger::Logger::new()`), what accepts provided fields:
| Name                 | Type    | Description                                                                          |
| -------------------- | ------- | ------------------------------------------------------------------------------------ |
|`outputCMD           `|`bool   `| If true, the logger will print logs into the terminal                                |
|`outputCMDFormat     `|`usize  `| Format of data printing (Formats stored in vars in `dvigoon::Logger`)                |
|`outputFile          `|`bool   `| If true, the logger will print logs into a file                                      |
|`outputFilePath      `|`String `| Path to file where to put logs (needs to be filled, if `outputFile` is true)         |
|`outputFileErase     `|`bool   `| If true, erases the log file with print of first log                                 |
|`outputFileFormat    `|`usize  `| Format of data printing (Formats stored in vars in `dvigoon::Logger`)                |

Functions, Logger provides:

**LogString()** - Function found in in `dvigoon::Logger::Logger`. Takes 2 args:
* `Level`: `&str` - String meaning level of logging, defaults can be found as vars in `dvigoon::Looger`
    * `LevelINFO`
    * `LevelWARN`
    * `LevelERROR`
    * `LevelFATAL`
* `Message`: "String" - Message, which logger will print

**LogStr()** - Same as `LogString()`, but takes `message` arg as `&str`

### Examples
`main.rs`
```rust
use dvigoon::Logger::*;

fn main() {
    let mut logger = Logger::new(true, FormatTEXT, true, "./log.log".to_string(), true, FormatJSON);

    logger.LogStr(LevelINFO, "Information");
    logger.LogStr(LevelWARN, "Warning");
    logger.LogStr(LevelERROR, "There is a mistake in your code!");
    logger.LogStr(LevelFATAL, "My code again isn't working again..");
}
```
`console`
```
2023-07-17T16:33:15.813645+00:00 [INFO] Information
2023-07-17T16:33:15.813832+00:00 [WARN] Warning
2023-07-17T16:33:15.819121+00:00 [ERROR] There is a mistake in your code!
2023-07-17T16:33:15.819364+00:00 [FATAL] My code again isn't working again..
```
`./log.log`
```json
{"time_stamp": "2023-07-17T16:33:15.813721+00:00", "level": "INFO", "message": "Information"}
{"time_stamp": "2023-07-17T16:33:15.813847+00:00", "level": "WARN", "message": "Warning"}
{"time_stamp": "2023-07-17T16:33:15.819181+00:00", "level": "ERROR", "message": "There is a mistake in your code!"}
{"time_stamp": "2023-07-17T16:33:15.819495+00:00", "level": "FATAL", "message": "My code again isn't working again.."}
```