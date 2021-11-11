while [ 1 ]
do
    clear
    wasm-pack build --target web --release
    cp pkg/mcpiano_bg.wasm pkg/mcpiano.js dist/
    ls -l dist/mcpiano_bg.wasm
    read
done


