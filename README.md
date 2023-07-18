# dvigoon
2D Game engine in rust

## Usage
Firstly, please read the docs, maybe you can fond something better :) **[Documentation](DOCS.md)**
* Download the `.zip` from the last version. You can find it in **[Releases](https://github.com/LeviiLovie/dvigoon/releases)**
* Unzip the library archive into your project directory. (Usualy third-party libs in rust stored in `./lib/` or `./libs/` directory)
* Add library to your `Cargo.toml`, example (replays `<>` and its content to your project information):
  ```
  [package]
  name = "<YOUR PROJECT NAME>"
  version = "0.1.0"
  edition = "2021"
  
  [dependencies]
  dvigoon = { path = "{<PATH TO UNZIPED LIBRARY FOLDER (ex. "./libs/dvigoon")>}" }
  ```
* Use library in your code, see examples in the **[Documentation](DOCS.md)**
