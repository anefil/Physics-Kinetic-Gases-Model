import * as wasm from "./mkt_basic_bg.wasm";
import { __wbg_set_wasm } from "./mkt_basic_bg.js";
export * from "./mkt_basic_bg.js";
export function init() { 
    __wbg_set_wasm(wasm); 
    // wasm.__wbindgen_start();
}

