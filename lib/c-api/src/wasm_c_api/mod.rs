//! Implementation of the [official WebAssembly C
//! API](https://github.com/WebAssembly/wasm-c-api) for Wasmer.
//!
//! We would like to remind the reader that this official standard can
//! be characterized as a _living standard_. As such, the API is not
//! yet stable, even though it shows maturity over time. The API is
//! described by the `wasm.h` C header, which is included by
//! `wasmer.h` C header file (which contains extension of the
//! standard API, for example to provide WASI or vendor-specific
//! features).
//!
//! # Quick Guide
//!
//! Usually, the user first needs to create an [`engine`] and a
//! [`store`]. Once it's done, the user needs to create a [`module`]
//! and then [instantiate][instance] it. When instantiating the
//! module, the user is able to pass a set of
//! [imports][externals]. With an instance, the user is able to call
//! the [exports][instance::wasm_instance_exports].
//!
//! Every module comes with examples and entry points to guide the
//! discovery of this API.

/// Private Rust macros.
#[macro_use]
mod macros;

/// An engine drives the compilation and the runtime.
///
/// Entry points: A default engine is created with
/// [`wasm_engine_new`][engine::wasm_engine_new] and freed with
/// [`wasm_engine_delete`][engine::wasm_engine_delete].
///
/// # Example
///
/// The simplest way to get a default engine is the following:
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer.h"
/// #
/// int main() {
///     // Create the engine.
///     wasm_engine_t* engine = wasm_engine_new();
///
///     // Check we have a valid engine!
///     assert(engine);
///
///     // Free everything.
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
///
/// To configure the engine, see the [`wasm_config_new`][engine::wasm_config_new].
pub mod engine;

/// cbindgen:ignore
pub mod externals;

/// A WebAssembly instance is a stateful, executable instance of a
/// WebAssembly module.
///
/// Instance objects contain all the exported WebAssembly functions,
/// memories, tables and globals that allow interacting with
/// WebAssembly.
///
/// Entry points: A WebAssembly instance is created with
/// [`wasm_instance_new`][instance::wasm_instance_new] and freed with
/// [`wasm_instance_delete`][instance::wasm_instance_delete].
///
/// # Example
///
/// The simplest way to instantiate a Wasm module is the following:
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer.h"
/// #
/// int main() {
///     // Create the engine and the store.
///     wasm_engine_t* engine = wasm_engine_new();
///     wasm_store_t* store = wasm_store_new(engine);
///
///     // Create a WebAssembly module from a WAT definition.
///     wasm_byte_vec_t wat;
///     wasmer_byte_vec_new_from_string(&wat, "(module)");
///     wasm_byte_vec_t wasm;
///     wat2wasm(&wat, &wasm);
///
///     // Create the module.
///     wasm_module_t* module = wasm_module_new(store, &wasm);
///     assert(module);
///
///     // Instantiate the module.
///     wasm_extern_vec_t imports = WASM_EMPTY_VEC;
///     wasm_trap_t* trap = NULL;
///
///     wasm_instance_t* instance = wasm_instance_new(store, module, &imports, &trap);
///     assert(instance);
///
///     // Now do something with the instance, like calling the
///     // exports with `wasm_instance_exports`.
///
///     // Free everything.
///     wasm_instance_delete(instance);
///     wasm_module_delete(module);
///     wasm_byte_vec_delete(&wasm);
///     wasm_byte_vec_delete(&wat);
///     wasm_store_delete(store);
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
///
/// cbindgen:ignore
pub mod instance;

/// A WebAssembly module contains stateless WebAssembly code that has
/// already been compiled and can be instantiated multiple times.
///
/// Entry points: A WebAssembly module is created with
/// [`wasm_module_new`][module::wasm_module_new] and freed with
/// [`wasm_module_delete`][module::wasm_module_delete].
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer.h"
/// #
/// int main() {
///     // Create the engine and the store.
///     wasm_engine_t* engine = wasm_engine_new();
///     wasm_store_t* store = wasm_store_new(engine);
///
///     // Create a WebAssembly module from a WAT definition.
///     wasm_byte_vec_t wat;
///     wasmer_byte_vec_new_from_string(&wat, "(module)");
///     wasm_byte_vec_t wasm;
///     wat2wasm(&wat, &wasm);
///    
///     // Create the module.
///     wasm_module_t* module = wasm_module_new(store, &wasm);
///
///     // It works!
///     assert(module);
///    
///     // Free everything.
///     wasm_byte_vec_delete(&wasm);
///     wasm_byte_vec_delete(&wat);
///     wasm_module_delete(module);
///     wasm_store_delete(store);
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
///
/// cbindgen:ignore
pub mod module;

/// A store represents all global state that can be manipulated by
/// WebAssembly programs. It consists of the runtime representation of
/// all instances of functions, tables, memories, and globals that
/// have been allocated during the lifetime of the abstract machine.
///
/// The store holds the [engine] (that is —amonst many things— used to
/// compile the Wasm bytes into a valid [module] artifact), in addition
/// to extra private types.
///
/// Entry points: A store is created with
/// [`wasm_store_new`][store::wasm_store_new] and freed with
/// [`wasm_store_delete`][store::wasm_store_delete]. To customize the
/// engine the store holds, see
/// [`wasm_config_new`][engine::wasm_config_new].
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer.h"
/// #
/// int main() {
///     // Create the engine.
///     wasm_engine_t* engine = wasm_engine_new();
///
///     // Create the store.
///     wasm_store_t* store = wasm_store_new(engine);
///
///     // It works!
///     assert(store);
///    
///     // Free everything.
///     wasm_store_delete(store);
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
///
/// cbindgen:ignore
pub mod store;

/// A trap represents an error which stores trace message with
/// backtrace.
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer.h"
/// #
/// int main() {
///     // Create an engine and a store.
///     wasm_engine_t* engine = wasm_engine_new();
///     wasm_store_t* store = wasm_store_new(engine);
///
///     // Create the trap message.
///     wasm_message_t message;
///     wasm_name_new_from_string_nt(&message, "foobar");
///
///     // Create the trap with its message.
///     // The backtrace will be generated automatically.
///     wasm_trap_t* trap = wasm_trap_new(store, &message);
///     assert(trap);
///
///     wasm_name_delete(&message);
///
///     // Do something with the trap.
///
///     // Free everything.
///     wasm_trap_delete(trap);
///     wasm_store_delete(store);
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
///
/// Usually, a trap is returned from a host function (an imported
/// function).
///
/// cbindgen:ignore
pub mod trap;

/// cbindgen:ignore
pub mod types;

/// This module contains _unstable non-standard_ C API.
///
/// Use them at your own risks. The API is subject to change or to
/// break without any plan to keep any compatibility :-).
pub mod unstable;

/// Possible runtime values that a WebAssembly module can either
/// consume or produce.
///
/// cbindgen:ignore
pub mod value;

/// Wasmer-specific API to get or query the version of this Wasm C API.
///
/// The `wasmer.h` file provides the `WASMER_VERSION`,
/// `WASMER_VERSION_MAJOR`, `WASMER_VERSION_MINOR`,
/// `WASMER_VERSION_PATCH` and `WASMER_VERSION_PRE`
/// constants. However, in absence of this header file, it is possible
/// to retrieve the same information with their respective functions,
/// namely [`wasmer_version`][version::wasmer_version],
/// [`wasmer_version_major`][version::wasmer_version_major],
/// [`wasmer_version_minor`][version::wasmer_version_minor],
/// [`wasmer_version_patch`][version::wasmer_version_patch], and
/// [`wasmer_version_pre`][version::wasmer_version_pre].
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer.h"
/// #
/// int main() {
///     // Get and print the version.
///     const char* version = wasmer_version();
///     printf("%s", version);
///
///     // No need to free the string. It's statically allocated on
///     // the Rust side.
///
///     return 0;
/// }
/// #    })
/// #    .success()
/// #    .stdout(env!("CARGO_PKG_VERSION"));
/// # }
/// ```
pub mod version;

#[cfg(feature = "wasi")]
pub mod wasi;

/// Wasmer-specific API to transform the WAT format into Wasm bytes.
///
/// It is used mostly for testing or for small program purposes.
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer.h"
/// #
/// int main() {
///     // Our WAT module.
///     wasm_byte_vec_t wat;
///     wasm_byte_vec_new(&wat, 8, "(module)");
///
///     // Our Wasm bytes.
///     wasm_byte_vec_t wasm;
///     wat2wasm(&wat, &wasm);
///
///     // It works!
///     assert(wasm.size > 0);
///
///     // Free everything.
///     wasm_byte_vec_delete(&wasm);
///     wasm_byte_vec_delete(&wat);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
#[cfg(feature = "wat")]
pub mod wat;

/// Wasmer-specific API to use CUDA driver interface to support GPU
///
/// # Example
///
/// ```c
/// #include <stdio.h>
/// #include <string.h>
/// #include <malloc.h>
/// #include "wasmer.h"
///
/// #define ERROR 0
/// #define OK 1
/// #define own
/// #define WASMER_WASI_ENABLED
///
/// static const char *wasm_path = "/home/yb/code/wasmer/lib/wasmer-cuda/tests/test_bin/";
///
///
/// // Assert that a `wasm_name_t` equals something.
/// void wasmer_assert_name(const wasm_name_t *name, const char *expected) {
///   assert(name->size == strlen(expected) &&
///          strncmp(name->data, expected, name->size) == 0);
/// }
///
/// // Use the last_error API to retrieve error messages
/// void print_wasmer_error()
/// {
///     int error_len = wasmer_last_error_length();
///     printf("Error len: `%d`\n", error_len);
///     char *error_str = malloc(error_len);
///     wasmer_last_error_message(error_str, error_len);
///     printf("Error str: `%s`\n", error_str);
/// }
///
/// int load_binary(wasm_byte_vec_t *binary, const char *file_name) {
///     FILE *file = fopen(file_name, "rb");
///     if (!file) {
///         printf("> file open error: %s\n", file_name);
///         return ERROR;
///     }
///     fseek(file, 0L, SEEK_END);
///     size_t file_size = ftell(file);
///     fseek(file, 0L, SEEK_SET);
///
///     wasm_byte_vec_new_uninitialized(binary, file_size);
///     if (fread(binary->data, file_size, 1, file) != 1) {
///         printf("file read binary error!\n");
///         return ERROR;
///     }
///     fclose(file);
///     return OK;
/// }
///
///
/// int run(const char *wasm_name, const char *run_arg) {
///     printf("\n\n\n--------------------------test %s------------------\n\n", wasm_name);
///     char *wasm_file = (char *) malloc(100);
///     strcat(strcpy(wasm_file, wasm_path), wasm_name);
///
///     // load binary
///     wasm_byte_vec_t binary;
///     if (load_binary(&binary, wasm_file) != OK) {
///         printf("load binary fail\n");
///         return ERROR;
///     }
///
///     free(wasm_file);
///
///     // Initialize.
///     wasm_engine_t *engine = wasm_engine_new();
///     wasm_store_t *store = wasm_store_new(engine);
///
///     // compile module
///     own wasm_module_t *module = wasm_module_new(store, &binary);
///     if ( !module ) {
///         printf("compile module error!\n");
///         return ERROR;
///     }
///
///     wasm_byte_vec_delete(&binary);
///
///     // init wasi_config
///     wasi_config_t *wasi_config = wasi_config_new(wasm_name);
///     wasi_config_arg(wasi_config, run_arg);
///
///     // init wasi_env
///     wasi_env_t *wasi_env = wasi_env_new(wasi_config);
///     if (!wasi_env) {
///         printf("build wasi env error!\n");
///         print_wasmer_error();
///         return ERROR;
///     }
///
///     // init cuda_env
///     cuda_env_t *cuda_env = cuda_env_new();
///     if (!cuda_env) {
///         printf("build cuda env error\n");
///         print_wasmer_error();
///         return ERROR;
///     }
///
///     // get imports size
///     wasm_importtype_vec_t import_types;
///     wasm_module_imports(module, &import_types);
///
///     // alloc memory for imports
///     wasm_extern_vec_t imports;
///     wasm_extern_vec_new_uninitialized(&imports, import_types.size+1000);
///     wasm_importtype_vec_delete(&import_types);
///
///     // init imports
///     bool get_import_result = cuda_wasi_get_imports(store, module, cuda_env, wasi_env, &imports);
///     if (!get_import_result) {
///         printf("get wasi imports error\n");
///         print_wasmer_error();
///         return ERROR;
///     }
///
///     // instantiate
///     wasm_instance_t *instance = wasm_instance_new(store, module, &imports, NULL);
///     if (!instance) {
///         printf("instantiating module\n");
///         print_wasmer_error();
///         return ERROR;
///     }
///
///     // extract export
///     own wasm_extern_vec_t exports;
///     wasm_instance_exports(instance, &exports);
///     if (exports.size == 0) {
///         printf("access exports error\n");
///         return 1;
///     }
///
///     // get func from export
///     wasm_func_t *run_func = wasi_get_start_function(instance);
///     if (run_func == NULL) {
///         printf("extract start from exports error\n");
///         print_wasmer_error();
///         return 1;
///     }
///
///     wasm_module_delete(module);
///     wasm_instance_delete(instance);
///
///     // call
///     wasm_val_vec_t args = WASM_EMPTY_VEC;
///     wasm_val_vec_t res = WASM_EMPTY_VEC;
///
///     if (wasm_func_call(run_func, &args, &res)) {
///         printf("call function error\n");
///         return ERROR;
///     }
///
///     wasm_extern_vec_delete(&exports);
///     wasm_extern_vec_delete(&imports);
///
///     wasm_func_delete(run_func);
///     wasi_env_delete(wasi_env);
///     cuda_env_delete(cuda_env);
///     wasm_store_delete(store);
///     wasm_engine_delete(engine);
///
///     return OK;
/// }
///
/// int main(int argc, const char *argv[]) {
///     if (run("device.wasm","") != OK) {
///         printf("test device.wasm fail\n");
///     }
///     if (run("mem.wasm", "") != OK) {
///         printf("test mem.wasm fail\n");
///     }
///     if (run("sumArray.wasm", "") != OK) {
///         printf("test sumArray.wasm fail\n");
///     }
///     if (run("matrixMulCuda.wasm", "1024") != OK) {
///         printf("test matrixMulCuda.wasm fail\n");
///     }
///     return 0;
/// }
/// ```
pub mod cuda;
