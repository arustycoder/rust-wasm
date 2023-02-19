// Import our outputted wasm ES6 module
// which, export default's, an initialization function
import wasmInit from "./pkg/hello_wasm.js";
import {domConsoleLog} from "../demo-utils/domConsole";

const runWasm = async () => {
    // Instantiate our wasm module
    const helloWorld = await wasmInit("./pkg/hello_wasm_bg.wasm");

    // Call the Add function export from wasm, save the result
    const addResult = helloWorld.call_me_from_javascript(3, 4);

    // set the result onto the body
    // document.body.textContent = `Hello Wasm! The addResult ${addResult}`;
    domConsoleLog(addResult);
    domConsoleLog(helloWorld.ADD_CONSTANT);
    domConsoleLog(helloWorld.add_integer_with_constant);
};
runWasm()