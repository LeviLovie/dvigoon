/*#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_variables)]*/

use dvigoon::Logger::*;

fn main() {
    let mut logger = Logger::new(true, FormatTEXT, true, "./log.log".to_string(), true, FormatJSON);

    logger.LogStr(LevelINFO, "Information");
    logger.LogStr(LevelWARN, "Warning");
    logger.LogStr(LevelERROR, "There is a mistake in your code!");
    logger.LogStr(LevelFATAL, "My code again isn't working!!!");
}
