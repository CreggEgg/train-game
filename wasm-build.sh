rm --r ./out/
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "train-game" \
    ./target/wasm32-unknown-unknown/release/train-game.wasm
cp -r ./assets/ ./out/
echo "
<!doctype html>
<html lang=\"en\" style=\"height: 100%; width: 100%;\">

<body style=\"margin: 0px; height: 100%; width: 100%;\">
  <script type=\"module\">
    import init from \"./train-game.js\"

    init().catch((error) => {
      if (!error.message.startsWith(\"Using exceptions for control flow, don't mind me. This isn't actually an error!\")) {
        throw error;
      }
    });
  </script>
</body>
</html>
" >> ./out/index.html
7za a out.zip  out
