cargo watch -i .gitignore -i "pkg/*" -c -q -s "cd core && wasm-pack build --target nodejs && cd ../interface && npm test"
