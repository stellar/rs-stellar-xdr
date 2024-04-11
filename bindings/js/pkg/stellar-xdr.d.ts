/* tslint:disable */
/* eslint-disable */
/**
* Returns a list of XDR types.
* @returns {(string)[]}
*/
export function types(): (string)[];
/**
* Returns the JSON Schema for an XDR type.
*
* JSON Schema version Draft 7 is returned.
* @param {string} type_variant
* @returns {string}
*/
export function schema(type_variant: string): string;
/**
* Identifies which XDR types the given XDR can decode to completely.
*
* Supports single XDR values only, not arrays, streams, or framed streams.
* @param {string} xdr_base64
* @returns {(string)[]}
*/
export function guess(xdr_base64: string): (string)[];
/**
* Decodes the XDR into JSON.
*
* Accepts a XDR base64 string.
*
* Returns a JSON string.
*
* Unstable: The API of this function is unstable and will likely be changed to
* return a JsValue instead of a JSON string.
* @param {string} type_variant
* @param {string} xdr_base64
* @returns {string}
*/
export function decode(type_variant: string, xdr_base64: string): string;
/**
* Encodes to XDR from JSON.
*
* Accepts a JSON string.
*
* Returns an XDR base64 string.
*
* Unstable: The API of this function is unstable and will likely be changed to
* accept a JsValue instead of a JSON string.
* @param {string} type_variant
* @param {string} json
* @returns {string}
*/
export function encode(type_variant: string, json: string): string;
