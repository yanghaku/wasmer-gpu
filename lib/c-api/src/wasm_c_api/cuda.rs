use wasmer_cuda::env::CudaEnv;
use wasmer_cuda::add_cuda_to_import;
use crate::wasm_c_api::wasi::wasi_env_t;
use crate::wasm_c_api::store::wasm_store_t;
use crate::wasm_c_api::module::wasm_module_t;
use crate::wasm_c_api::externals::wasm_extern_vec_t;
use crate::error::CApiError;
use wasmer::{imports, ImportObject, Extern, Store, NamedResolver};
use wasmer_wasi::{get_wasi_version, generate_import_object_from_env};
use crate::wasm_c_api::unstable::wasi::wasmer_named_extern_t;
use crate::wasm_c_api::unstable::wasi::wasmer_named_extern_vec_t;

#[allow(non_camel_case_types)]
pub struct cuda_env_t {
    pub(super) inner: CudaEnv,
}

/// Create a new WASI environment
#[no_mangle]
pub extern "C" fn cuda_env_new() -> Option<Box<cuda_env_t>> {
    Some(Box::new(cuda_env_t {
        inner: CudaEnv::new(),
    }))
}

/// Delete a `cuda_env_t`
#[no_mangle]
pub extern "C" fn cuda_env_delete(_x: Option<Box<cuda_env_t>>) {}

/// return an Ordered imports vec for the module
#[no_mangle]
pub unsafe extern "C" fn cuda_get_imports(
    store: Option<&wasm_store_t>,
    module: Option<&wasm_module_t>,
    cuda_env: Option<&cuda_env_t>,
    imports: &mut wasm_extern_vec_t,
) -> bool {
    cuda_get_imports_inner(store, module, cuda_env, imports).is_some()
}

fn cuda_get_imports_inner(
    store: Option<&wasm_store_t>,
    module: Option<&wasm_module_t>,
    cuda_env: Option<&cuda_env_t>,
    imports: &mut wasm_extern_vec_t,
) -> Option<()> {
    let store = store?;
    let module = module?;
    let cuda_env = cuda_env?;

    let store = &store.inner;

    let mut import_object = imports! {};
    add_cuda_to_import(store, cuda_env.inner.clone(), &mut import_object);

    map_to_ordered_imports(imports, module, import_object, store)
}

/// return an Ordered imports vec for the module
/// not only cuda_imports, but also wasi_imports
#[no_mangle]
pub unsafe extern "C" fn cuda_wasi_get_imports(
    store: Option<&wasm_store_t>,
    module: Option<&wasm_module_t>,
    cuda_env: Option<&cuda_env_t>,
    wasi_env: Option<&wasi_env_t>,
    imports: &mut wasm_extern_vec_t,
) -> bool {
    cuda_wasi_get_imports_inner(store, module, cuda_env, wasi_env, imports).is_some()
}

fn cuda_wasi_get_imports_inner(
    store: Option<&wasm_store_t>,
    module: Option<&wasm_module_t>,
    cuda_env: Option<&cuda_env_t>,
    wasi_env: Option<&wasi_env_t>,
    imports: &mut wasm_extern_vec_t,
) -> Option<()> {
    let store = store?;
    let module = module?;
    let cuda_env = cuda_env?;
    let wasi_env = wasi_env?;

    let store = &store.inner;

    let version = c_try!(
        get_wasi_version(&module.inner, false).ok_or_else(|| CApiError{
            msg: "could not detect a WASI version on this module".to_string(),
        })
    );

    let mut import_object = generate_import_object_from_env(store, wasi_env.inner.clone(), version);
    add_cuda_to_import(store, cuda_env.inner.clone(), &mut import_object);

    map_to_ordered_imports(imports, module, import_object, store)
}


fn map_to_ordered_imports(imports: &mut wasm_extern_vec_t, module: &wasm_module_t,
                          import_object: ImportObject, store: &Store) -> Option<()> {

    *imports = module.inner.imports().map(
        |import_type| {
            let export = c_try!(import_object.resolve_by_name(
                import_type.module(), import_type.name())
                .ok_or_else(|| CApiError {
                    msg: format!(
                        "failed to resolve import \"{}\" \"{}\" !!",
                        import_type.module(),
                        import_type.name()
                    ),
                }));
            let inner = Extern::from_vm_export(store, export);
            Some(Box::new(inner.into()))
        }
    ).collect::<Option<Vec<_>>>()?.into();

    Some(())
}

/// get the unordered imports for cuda
#[no_mangle]
pub unsafe extern "C" fn cuda_get_unordered_imports(
    store: Option<&wasm_store_t>,
    cuda_env: Option<&cuda_env_t>,
    unordered_imports: &mut wasmer_named_extern_vec_t,
) -> bool {
    cuda_get_unordered_imports_inner(store, cuda_env, unordered_imports).is_some()
}

fn cuda_get_unordered_imports_inner(
    store: Option<&wasm_store_t>,
    cuda_env: Option<&cuda_env_t>,
    unordered_imports: &mut wasmer_named_extern_vec_t,
) -> Option<()> {
    let store = store?;
    let cuda_env = cuda_env?;

    let store = &store.inner;

    let mut import_object = imports! {};
    add_cuda_to_import(store, cuda_env.inner.clone(), &mut import_object);

    *unordered_imports = import_object.into_iter()
        .map(|((module, name), export)| {
            let module = module.into();
            let name = name.into();
            let extern_inner = Extern::from_vm_export(store, export);

            Box::new(wasmer_named_extern_t {
                module,
                name,
                r#extern: Box::new(extern_inner.into()),
            })
        })
        .collect::<Vec<_>>()
        .into();

    Some(())
}

