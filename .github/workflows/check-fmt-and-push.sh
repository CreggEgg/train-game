
nix-shell --run "cargo fmt"

if ! git diff --exit-code; then
    git add -A
    git commit -m "fixed formatting"
    git push
fi
