# clean
rm -r dist

cargo build --release

mkdir dist
cp target/release/licenser dist
cp -r templates dist/templates

cp LICENSE dist
cp README.md dist