const bc = await Deno.readFile("../target/wasm32-unknown-unknown/debug/simple.wasm");
const instance = await WebAssembly.instantiate(bc, {});
const got = (instance.instance.exports.plus1 as Function)(3);
console.log(got);
