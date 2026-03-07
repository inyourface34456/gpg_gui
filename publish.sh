# Build first
trunk build --release

# Copy dist contents to a temp location
cp -r dist/ /tmp/gh-pages-dist/

# Switch to gh-pages, wipe it, replace with new build\
git add .
git commit -m $1
git push origin main
git checkout gh-pages
cp -r /tmp/gh-pages-dist/* .
git add .
git commit -m "Deploy"
git push origin gh-pages
rm $(ls /tmp/gh-pages-dist)
git checkout main