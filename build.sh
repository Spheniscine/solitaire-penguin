rm -r ./docs
dx bundle --release
mv ./target/dx/solitaire-penguin/release/web/public ./docs