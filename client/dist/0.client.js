(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../wasm/pkg/unison_wasm.js":
/*!**********************************!*\
  !*** ../wasm/pkg/unison_wasm.js ***!
  \**********************************/
/*! exports provided: lambda, resume, run_sync, enable_logging, enable_logging_with_prefix, info, effects, run, load, __wbindgen_json_parse, __wbindgen_json_serialize, __wbindgen_object_drop_ref, __wbindgen_number_new, __wbindgen_string_new, __wbindgen_is_null, __wbindgen_is_undefined, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbg_debug_cd8a0aad17c8c92f, __wbg_error_7dcc755846c00ef7, __wbg_error_b47ee9a774776bfa, __wbg_info_0c64856d96c69122, __wbg_log_7fc0936bf7223435, __wbg_warn_f88df7e1e2a26187, __wbg_get_9ca243f6a0c3698a, __wbg_new_17534eac4df3cd22, __wbg_push_7114ccbf1c58e41f, __wbg_apply_e7d7fd46ecc6760a, __wbindgen_number_get, __wbindgen_string_get, __wbindgen_boolean_get, __wbindgen_debug_string, __wbindgen_throw, __wbindgen_rethrow */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./unison_wasm_bg.wasm */ \"../wasm/pkg/unison_wasm_bg.wasm\");\n/* harmony import */ var _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./unison_wasm_bg.js */ \"../wasm/pkg/unison_wasm_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"lambda\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"lambda\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"resume\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"resume\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"run_sync\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"run_sync\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"enable_logging\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"enable_logging\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"enable_logging_with_prefix\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"enable_logging_with_prefix\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"info\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"info\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"effects\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"effects\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"run\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"load\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"load\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_json_parse\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_serialize\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_json_serialize\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_number_new\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_number_new\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_string_new\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_null\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_null\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_debug_cd8a0aad17c8c92f\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_debug_cd8a0aad17c8c92f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_7dcc755846c00ef7\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_7dcc755846c00ef7\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_b47ee9a774776bfa\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_b47ee9a774776bfa\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_info_0c64856d96c69122\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_info_0c64856d96c69122\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_7fc0936bf7223435\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_log_7fc0936bf7223435\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_warn_f88df7e1e2a26187\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_warn_f88df7e1e2a26187\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_get_9ca243f6a0c3698a\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_get_9ca243f6a0c3698a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_17534eac4df3cd22\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_17534eac4df3cd22\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_push_7114ccbf1c58e41f\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_push_7114ccbf1c58e41f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_apply_e7d7fd46ecc6760a\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_apply_e7d7fd46ecc6760a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_number_get\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_number_get\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_get\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_string_get\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_boolean_get\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_boolean_get\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_debug_string\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_debug_string\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_rethrow\", function() { return _unison_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_rethrow\"]; });\n\n\n\n\n//# sourceURL=webpack:///../wasm/pkg/unison_wasm.js?");

/***/ }),

/***/ "../wasm/pkg/unison_wasm_bg.js":
/*!*************************************!*\
  !*** ../wasm/pkg/unison_wasm_bg.js ***!
  \*************************************/
/*! exports provided: lambda, resume, run_sync, enable_logging, enable_logging_with_prefix, info, effects, run, load, __wbindgen_json_parse, __wbindgen_json_serialize, __wbindgen_object_drop_ref, __wbindgen_number_new, __wbindgen_string_new, __wbindgen_is_null, __wbindgen_is_undefined, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbg_debug_cd8a0aad17c8c92f, __wbg_error_7dcc755846c00ef7, __wbg_error_b47ee9a774776bfa, __wbg_info_0c64856d96c69122, __wbg_log_7fc0936bf7223435, __wbg_warn_f88df7e1e2a26187, __wbg_get_9ca243f6a0c3698a, __wbg_new_17534eac4df3cd22, __wbg_push_7114ccbf1c58e41f, __wbg_apply_e7d7fd46ecc6760a, __wbindgen_number_get, __wbindgen_string_get, __wbindgen_boolean_get, __wbindgen_debug_string, __wbindgen_throw, __wbindgen_rethrow */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"lambda\", function() { return lambda; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"resume\", function() { return resume; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"run_sync\", function() { return run_sync; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"enable_logging\", function() { return enable_logging; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"enable_logging_with_prefix\", function() { return enable_logging_with_prefix; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"info\", function() { return info; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"effects\", function() { return effects; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return run; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"load\", function() { return load; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return __wbindgen_json_parse; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_serialize\", function() { return __wbindgen_json_serialize; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_number_new\", function() { return __wbindgen_number_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return __wbindgen_string_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_null\", function() { return __wbindgen_is_null; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_debug_cd8a0aad17c8c92f\", function() { return __wbg_debug_cd8a0aad17c8c92f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_7dcc755846c00ef7\", function() { return __wbg_error_7dcc755846c00ef7; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_b47ee9a774776bfa\", function() { return __wbg_error_b47ee9a774776bfa; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_info_0c64856d96c69122\", function() { return __wbg_info_0c64856d96c69122; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_log_7fc0936bf7223435\", function() { return __wbg_log_7fc0936bf7223435; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_warn_f88df7e1e2a26187\", function() { return __wbg_warn_f88df7e1e2a26187; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_get_9ca243f6a0c3698a\", function() { return __wbg_get_9ca243f6a0c3698a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_17534eac4df3cd22\", function() { return __wbg_new_17534eac4df3cd22; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_push_7114ccbf1c58e41f\", function() { return __wbg_push_7114ccbf1c58e41f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_apply_e7d7fd46ecc6760a\", function() { return __wbg_apply_e7d7fd46ecc6760a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_number_get\", function() { return __wbindgen_number_get; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_get\", function() { return __wbindgen_string_get; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_boolean_get\", function() { return __wbindgen_boolean_get; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_debug_string\", function() { return __wbindgen_debug_string; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_rethrow\", function() { return __wbindgen_rethrow; });\n/* harmony import */ var _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./unison_wasm_bg.wasm */ \"../wasm/pkg/unison_wasm_bg.wasm\");\nfunction _typeof(obj) { \"@babel/helpers - typeof\"; if (typeof Symbol === \"function\" && typeof Symbol.iterator === \"symbol\") { _typeof = function _typeof(obj) { return typeof obj; }; } else { _typeof = function _typeof(obj) { return obj && typeof Symbol === \"function\" && obj.constructor === Symbol && obj !== Symbol.prototype ? \"symbol\" : typeof obj; }; } return _typeof(obj); }\n\n\nvar lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\nvar cachedTextDecoder = new lTextDecoder('utf-8', {\n  ignoreBOM: true,\n  fatal: true\n});\ncachedTextDecoder.decode();\nvar cachegetUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n  if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n    cachegetUint8Memory0 = new Uint8Array(_unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n  }\n\n  return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nvar heap = new Array(32).fill(undefined);\nheap.push(undefined, null, true, false);\nvar heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n  if (heap_next === heap.length) heap.push(heap.length + 1);\n  var idx = heap_next;\n  heap_next = heap[idx];\n  heap[idx] = obj;\n  return idx;\n}\n\nfunction getObject(idx) {\n  return heap[idx];\n}\n\nvar WASM_VECTOR_LEN = 0;\nvar lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\nvar cachedTextEncoder = new lTextEncoder('utf-8');\nvar encodeString = typeof cachedTextEncoder.encodeInto === 'function' ? function (arg, view) {\n  return cachedTextEncoder.encodeInto(arg, view);\n} : function (arg, view) {\n  var buf = cachedTextEncoder.encode(arg);\n  view.set(buf);\n  return {\n    read: arg.length,\n    written: buf.length\n  };\n};\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n  if (realloc === undefined) {\n    var buf = cachedTextEncoder.encode(arg);\n\n    var _ptr = malloc(buf.length);\n\n    getUint8Memory0().subarray(_ptr, _ptr + buf.length).set(buf);\n    WASM_VECTOR_LEN = buf.length;\n    return _ptr;\n  }\n\n  var len = arg.length;\n  var ptr = malloc(len);\n  var mem = getUint8Memory0();\n  var offset = 0;\n\n  for (; offset < len; offset++) {\n    var code = arg.charCodeAt(offset);\n    if (code > 0x7F) break;\n    mem[ptr + offset] = code;\n  }\n\n  if (offset !== len) {\n    if (offset !== 0) {\n      arg = arg.slice(offset);\n    }\n\n    ptr = realloc(ptr, len, len = offset + arg.length * 3);\n    var view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n    var ret = encodeString(arg, view);\n    offset += ret.written;\n  }\n\n  WASM_VECTOR_LEN = offset;\n  return ptr;\n}\n\nvar cachegetInt32Memory0 = null;\n\nfunction getInt32Memory0() {\n  if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n    cachegetInt32Memory0 = new Int32Array(_unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n  }\n\n  return cachegetInt32Memory0;\n}\n\nfunction dropObject(idx) {\n  if (idx < 36) return;\n  heap[idx] = heap_next;\n  heap_next = idx;\n}\n\nfunction takeObject(idx) {\n  var ret = getObject(idx);\n  dropObject(idx);\n  return ret;\n}\n\nfunction isLikeNone(x) {\n  return x === undefined || x === null;\n}\n\nvar cachegetFloat64Memory0 = null;\n\nfunction getFloat64Memory0() {\n  if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n    cachegetFloat64Memory0 = new Float64Array(_unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n  }\n\n  return cachegetFloat64Memory0;\n}\n\nfunction debugString(val) {\n  // primitive types\n  var type = _typeof(val);\n\n  if (type == 'number' || type == 'boolean' || val == null) {\n    return \"\".concat(val);\n  }\n\n  if (type == 'string') {\n    return \"\\\"\".concat(val, \"\\\"\");\n  }\n\n  if (type == 'symbol') {\n    var description = val.description;\n\n    if (description == null) {\n      return 'Symbol';\n    } else {\n      return \"Symbol(\".concat(description, \")\");\n    }\n  }\n\n  if (type == 'function') {\n    var name = val.name;\n\n    if (typeof name == 'string' && name.length > 0) {\n      return \"Function(\".concat(name, \")\");\n    } else {\n      return 'Function';\n    }\n  } // objects\n\n\n  if (Array.isArray(val)) {\n    var length = val.length;\n    var debug = '[';\n\n    if (length > 0) {\n      debug += debugString(val[0]);\n    }\n\n    for (var i = 1; i < length; i++) {\n      debug += ', ' + debugString(val[i]);\n    }\n\n    debug += ']';\n    return debug;\n  } // Test for built-in\n\n\n  var builtInMatches = /\\[object ([^\\]]+)\\]/.exec(toString.call(val));\n  var className;\n\n  if (builtInMatches.length > 1) {\n    className = builtInMatches[1];\n  } else {\n    // Failed to match the standard '[object ClassName]'\n    return toString.call(val);\n  }\n\n  if (className == 'Object') {\n    // we're a user defined class or Object\n    // JSON.stringify avoids problems with cycles, and is generally much\n    // easier than looping through ownProperties of `val`.\n    try {\n      return 'Object(' + JSON.stringify(val) + ')';\n    } catch (_) {\n      return 'Object';\n    }\n  } // errors\n\n\n  if (val instanceof Error) {\n    return \"\".concat(val.name, \": \").concat(val.message, \"\\n\").concat(val.stack);\n  } // TODO we could test for more things here, like `Set`s and `Map`s.\n\n\n  return className;\n}\n\nvar cachegetUint32Memory0 = null;\n\nfunction getUint32Memory0() {\n  if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n    cachegetUint32Memory0 = new Uint32Array(_unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n  }\n\n  return cachegetUint32Memory0;\n}\n\nfunction passArrayJsValueToWasm0(array, malloc) {\n  var ptr = malloc(array.length * 4);\n  var mem = getUint32Memory0();\n\n  for (var i = 0; i < array.length; i++) {\n    mem[ptr / 4 + i] = addHeapObject(array[i]);\n  }\n\n  WASM_VECTOR_LEN = array.length;\n  return ptr;\n}\n/**\n* @param {number} env_id\n* @param {any} partial\n* @param {any} arg\n* @param {any[]} raw_handlers\n* @returns {any}\n*/\n\n\nfunction lambda(env_id, partial, arg, raw_handlers) {\n  var ptr0 = passArrayJsValueToWasm0(raw_handlers, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  var ret = _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"lambda\"](env_id, addHeapObject(partial), addHeapObject(arg), ptr0, len0);\n  return takeObject(ret);\n}\n/**\n* @param {number} env_id\n* @param {any} kont\n* @param {any} arg\n* @param {any[]} raw_handlers\n* @returns {any}\n*/\n\nfunction resume(env_id, kont, arg, raw_handlers) {\n  var ptr0 = passArrayJsValueToWasm0(raw_handlers, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  var ret = _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"resume\"](env_id, addHeapObject(kont), addHeapObject(arg), ptr0, len0);\n  return takeObject(ret);\n}\n/**\n* @param {number} env_id\n* @param {string} term\n* @param {any[]} args\n* @param {any[]} raw_handlers\n* @returns {any}\n*/\n\nfunction run_sync(env_id, term, args, raw_handlers) {\n  var ptr0 = passStringToWasm0(term, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  var ptr1 = passArrayJsValueToWasm0(args, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n  var len1 = WASM_VECTOR_LEN;\n  var ptr2 = passArrayJsValueToWasm0(raw_handlers, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n  var len2 = WASM_VECTOR_LEN;\n  var ret = _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"run_sync\"](env_id, ptr0, len0, ptr1, len1, ptr2, len2);\n  return takeObject(ret);\n}\n/**\n*/\n\nfunction enable_logging() {\n  _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"enable_logging\"]();\n}\n/**\n* @param {string} prefix\n*/\n\nfunction enable_logging_with_prefix(prefix) {\n  var ptr0 = passStringToWasm0(prefix, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"enable_logging_with_prefix\"](ptr0, len0);\n}\n/**\n* @param {number} env_id\n* @param {string} term\n* @returns {any}\n*/\n\nfunction info(env_id, term) {\n  var ptr0 = passStringToWasm0(term, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  var ret = _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"info\"](env_id, ptr0, len0);\n  return takeObject(ret);\n}\n/**\n* @param {number} env_id\n* @param {string} term\n* @returns {any}\n*/\n\nfunction effects(env_id, term) {\n  var ptr0 = passStringToWasm0(term, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  var ret = _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"effects\"](env_id, ptr0, len0);\n  return takeObject(ret);\n}\n/**\n* @param {number} env_id\n* @param {string} term\n* @param {any[]} args\n* @param {any[]} raw_handlers\n* @returns {any}\n*/\n\nfunction run(env_id, term, args, raw_handlers) {\n  var ptr0 = passStringToWasm0(term, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  var ptr1 = passArrayJsValueToWasm0(args, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n  var len1 = WASM_VECTOR_LEN;\n  var ptr2 = passArrayJsValueToWasm0(raw_handlers, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n  var len2 = WASM_VECTOR_LEN;\n  var ret = _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"run\"](env_id, ptr0, len0, ptr1, len1, ptr2, len2);\n  return takeObject(ret);\n}\n/**\n* @param {string} data\n* @returns {number}\n*/\n\nfunction load(data) {\n  var ptr0 = passStringToWasm0(data, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  var ret = _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"load\"](ptr0, len0);\n  return ret >>> 0;\n}\n\nfunction handleError(f) {\n  return function () {\n    try {\n      return f.apply(this, arguments);\n    } catch (e) {\n      _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n    }\n  };\n}\n\nvar __wbindgen_json_parse = function __wbindgen_json_parse(arg0, arg1) {\n  var ret = JSON.parse(getStringFromWasm0(arg0, arg1));\n  return addHeapObject(ret);\n};\nvar __wbindgen_json_serialize = function __wbindgen_json_serialize(arg0, arg1) {\n  var obj = getObject(arg1);\n  var ret = JSON.stringify(obj === undefined ? null : obj);\n  var ptr0 = passStringToWasm0(ret, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  getInt32Memory0()[arg0 / 4 + 1] = len0;\n  getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\nvar __wbindgen_object_drop_ref = function __wbindgen_object_drop_ref(arg0) {\n  takeObject(arg0);\n};\nvar __wbindgen_number_new = function __wbindgen_number_new(arg0) {\n  var ret = arg0;\n  return addHeapObject(ret);\n};\nvar __wbindgen_string_new = function __wbindgen_string_new(arg0, arg1) {\n  var ret = getStringFromWasm0(arg0, arg1);\n  return addHeapObject(ret);\n};\nvar __wbindgen_is_null = function __wbindgen_is_null(arg0) {\n  var ret = getObject(arg0) === null;\n  return ret;\n};\nvar __wbindgen_is_undefined = function __wbindgen_is_undefined(arg0) {\n  var ret = getObject(arg0) === undefined;\n  return ret;\n};\nvar __wbg_new_59cb74e423758ede = function __wbg_new_59cb74e423758ede() {\n  var ret = new Error();\n  return addHeapObject(ret);\n};\nvar __wbg_stack_558ba5917b466edd = function __wbg_stack_558ba5917b466edd(arg0, arg1) {\n  var ret = getObject(arg1).stack;\n  var ptr0 = passStringToWasm0(ret, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  getInt32Memory0()[arg0 / 4 + 1] = len0;\n  getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\nvar __wbg_error_4bb6c2a97407129a = function __wbg_error_4bb6c2a97407129a(arg0, arg1) {\n  try {\n    console.error(getStringFromWasm0(arg0, arg1));\n  } finally {\n    _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n  }\n};\nvar __wbg_debug_cd8a0aad17c8c92f = function __wbg_debug_cd8a0aad17c8c92f(arg0, arg1, arg2, arg3) {\n  console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));\n};\nvar __wbg_error_7dcc755846c00ef7 = function __wbg_error_7dcc755846c00ef7(arg0) {\n  console.error(getObject(arg0));\n};\nvar __wbg_error_b47ee9a774776bfa = function __wbg_error_b47ee9a774776bfa(arg0, arg1, arg2, arg3) {\n  console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));\n};\nvar __wbg_info_0c64856d96c69122 = function __wbg_info_0c64856d96c69122(arg0, arg1, arg2, arg3) {\n  console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));\n};\nvar __wbg_log_7fc0936bf7223435 = function __wbg_log_7fc0936bf7223435(arg0, arg1, arg2, arg3) {\n  console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));\n};\nvar __wbg_warn_f88df7e1e2a26187 = function __wbg_warn_f88df7e1e2a26187(arg0, arg1, arg2, arg3) {\n  console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));\n};\nvar __wbg_get_9ca243f6a0c3698a = function __wbg_get_9ca243f6a0c3698a(arg0, arg1) {\n  var ret = getObject(arg0)[arg1 >>> 0];\n  return addHeapObject(ret);\n};\nvar __wbg_new_17534eac4df3cd22 = function __wbg_new_17534eac4df3cd22() {\n  var ret = new Array();\n  return addHeapObject(ret);\n};\nvar __wbg_push_7114ccbf1c58e41f = function __wbg_push_7114ccbf1c58e41f(arg0, arg1) {\n  var ret = getObject(arg0).push(getObject(arg1));\n  return ret;\n};\nvar __wbg_apply_e7d7fd46ecc6760a = handleError(function (arg0, arg1, arg2) {\n  var ret = getObject(arg0).apply(getObject(arg1), getObject(arg2));\n  return addHeapObject(ret);\n});\nvar __wbindgen_number_get = function __wbindgen_number_get(arg0, arg1) {\n  var obj = getObject(arg1);\n  var ret = typeof obj === 'number' ? obj : undefined;\n  getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;\n  getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);\n};\nvar __wbindgen_string_get = function __wbindgen_string_get(arg0, arg1) {\n  var obj = getObject(arg1);\n  var ret = typeof obj === 'string' ? obj : undefined;\n  var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  getInt32Memory0()[arg0 / 4 + 1] = len0;\n  getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\nvar __wbindgen_boolean_get = function __wbindgen_boolean_get(arg0) {\n  var v = getObject(arg0);\n  var ret = typeof v === 'boolean' ? v ? 1 : 0 : 2;\n  return ret;\n};\nvar __wbindgen_debug_string = function __wbindgen_debug_string(arg0, arg1) {\n  var ret = debugString(getObject(arg1));\n  var ptr0 = passStringToWasm0(ret, _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unison_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n  var len0 = WASM_VECTOR_LEN;\n  getInt32Memory0()[arg0 / 4 + 1] = len0;\n  getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\nvar __wbindgen_throw = function __wbindgen_throw(arg0, arg1) {\n  throw new Error(getStringFromWasm0(arg0, arg1));\n};\nvar __wbindgen_rethrow = function __wbindgen_rethrow(arg0) {\n  throw takeObject(arg0);\n};\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../../client/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../wasm/pkg/unison_wasm_bg.js?");

/***/ }),

/***/ "../wasm/pkg/unison_wasm_bg.wasm":
/*!***************************************!*\
  !*** ../wasm/pkg/unison_wasm_bg.wasm ***!
  \***************************************/
/*! exports provided: memory, lambda, resume, run_sync, enable_logging, enable_logging_with_prefix, info, effects, run, load, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free, __wbindgen_exn_store */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./unison_wasm_bg.js */ \"../wasm/pkg/unison_wasm_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../wasm/pkg/unison_wasm_bg.wasm?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);