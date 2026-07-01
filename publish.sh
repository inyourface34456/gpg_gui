set -o errexit

find . -name '*.rs' -exec sed -i 's/[ \t]*$//' {} +
cargo fmt
trunk build --release --minify

CACHE_DIR="../.build-cache"
mkdir -p "$CACHE_DIR"
rm -rf "$CACHE_DIR/target"
mv target "$CACHE_DIR/"

rm -rf /tmp/gh-pages-dist
mkdir /tmp/gh-pages-dist
cp -r dist/* /tmp/gh-pages-dist/

git add .
git commit --allow-empty -m "$1"
git push

git checkout gh-pages
rm -rf * target
cp -r /tmp/gh-pages-dist/* .
git add .
git commit --allow-empty -m "$1"
git push origin gh-pages

git checkout main
mv "$CACHE_DIR/target" .

rm -rf /tmp/gh-pages-dist