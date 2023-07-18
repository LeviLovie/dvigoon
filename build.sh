cargo build --release

mkdir -p dvigoon
mkdir -p dvigoon/src
cp -R src dvigoon/
cp Cargo.toml dvigoon/Cargo.toml

zip -r dvigoon.zip dvigoon
rm -R dvigoon