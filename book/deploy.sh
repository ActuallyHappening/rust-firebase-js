set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

git worktree add -f /tmp/book gh-pages
mdbook build
rm -rf /tmp/book/* # this won't delete the .git directory
cp -rp book/* /tmp/book/
cd /tmp/book
git add -A
git commit -m 'deploy new book'
git push origin gh-pages
cd -