set -o errexit

trunk build --release

# Copy dist contents to a temp location
rm -rf /tmp/target-cache
mkdir /tmp/target-cache
mv target /tmp/target-cache

rm -rf /tmp/gh-pages-dist
mkdir /tmp/gh-pages-dist
cp dist/* /tmp/gh-pages-dist/

# commit main so i doint acadently wipe miain branch
git add .
git commit --allow-empty -m "$1"
git push

# Switch to gh-pages, wipe it, replace with new build
git checkout gh-pages
rm -rf * target
cp -r /tmp/gh-pages-dist/* .
git add .
git commit --allow-empty -m "$1"
git push origin gh-pages
git checkout main
rm -rf target
mv /tmp/target-cache/target .