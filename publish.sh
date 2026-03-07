set -o errexit

trunk build --release

# Copy dist contents to a temp location
cp dist/* /tmp/gh-pages-dist/

# commit main so i doint acadently wipe miain branch
git add .
git commit --allow-empty -m "Deploy"
git push

# Switch to gh-pages, wipe it, replace with new build
git checkout gh-pages
rm -rf *
cp -r /tmp/gh-pages-dist/* .
git add .
git commit -m "Deploy"
git push origin gh-pages
git checkout main