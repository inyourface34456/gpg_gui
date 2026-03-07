git filter-branch --force --index-filter \
  'git rm -rf --cached --ignore-unmatch target/' \
  --prune-empty --tag-name-filter cat -- --all

# Clean up filter-branch backups
git for-each-ref --format="delete %(refname)" refs/original/ | git update-ref --stdin
git reflog expire --expire=now --all
git gc --prune=now --aggressive

# Force push all branches
git push origin --force --all
git push origin --force --tags

echo "✅ Done! target/ removed from all history."