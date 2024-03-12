# `csv_to_usv` as [Extism](https://github.com/extism/extism) plugin

Check out the Wasm code at: https://modsurfer.dylibso.com/module?hash=a28e7322a6fde92cc27344584b5e86c211dbd5a345fe6ec95f1389733c325541

![csv_to_usv badge](https://cdn.modsurfer.dylibso.com/api/v1/module/a28e7322a6fde92cc27344584b5e86c211dbd5a345fe6ec95f1389733c325541/badge.png)

## Run from other languages

1. Install an Extism Host SDK in 15+ languages: https://github.com/extism/extism

2. Load the `csv_to_usv` plugin and call `csv_to_usv` export function:

```js
// Simple example to run this in your browser! But will work in Go, PHP, Ruby, Java, Python, etc...
const extism = await import("https://esm.sh/@extism/extism");
        
const plugin = await extism.createPlugin(
    "https://cdn.modsurfer.dylibso.com/api/v1/module/a28e7322a6fde92cc27344584b5e86c211dbd5a345fe6ec95f1389733c325541.wasm",
    { useWasi: false }
);

let out = await plugin.call("csv_to_usv", "a,b,c");
console.log(out.text());
```

3. ???

4. Profit