import * as wasm from "./stellar-xdr_bg.wasm";
import { __wbg_set_wasm } from "./stellar-xdr_bg.js";
__wbg_set_wasm(wasm);
export * from "./stellar-xdr_bg.js";
