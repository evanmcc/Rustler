/// Exports a given list of functions to a Erlang module.
///
/// This should be called exactly once in every NIF library. It will wrap and export the given rust
/// functions into the Erlang module.
/// 
/// The first argument is a string specifying what Erlang/Elixir module you want the function
/// exported into. In Erlang this will simply be the atom you named your module. In Elixir, all
/// modules are prefixed with `Elixir.<module path>`
/// 
/// The second argument is a list of 3-tuples. Each tuple contains information on a single exported
/// NIF function. The first tuple item is the name you want to export the function into, the second
/// is the arity (number of arguments) of the exported function. The third argument is a
/// indentifier of a rust function. This is where your actual NIF will be implemented.
///
/// The third argument is an `Option<fn(env: &NifEnv, load_info: NifTerm) -> bool>`. If this is
/// `Some`, the function will execute when the NIF is first loaded by the BEAM.
#[macro_export]
macro_rules! rustler_export_nifs {
    ($name:expr, [$( ($nif_name:expr, $nif_arity:expr, $nif_fun:path) ),*], $on_load:expr) => (
        static mut NIF_ENTRY: Option<rustler::wrapper::nif_interface::DEF_NIF_ENTRY> = None;

        #[no_mangle]
        pub extern "C" fn nif_init() -> *const rustler::wrapper::nif_interface::DEF_NIF_ENTRY {
            // TODO: If a NIF ever gets unloaded, there will be a memory leak! Fix this!
            // TODO: If an unwrap ever happens, we will unwind right into C! Fix this!

            extern "C" fn nif_load(
                env: rustler::wrapper::nif_interface::NIF_ENV,
                _priv_data: *mut *mut rustler::codegen_runtime::c_void,
                load_info: rustler::wrapper::nif_interface::NIF_TERM)
                -> rustler::codegen_runtime::c_int {
                    rustler::codegen_runtime::handle_nif_init_call($on_load, env, load_info)
                }

            let fun_entries = [
                $(
                    rustler::wrapper::nif_interface::DEF_NIF_FUNC {
                        name: ::std::ffi::CString::new($nif_name).unwrap().into_raw() as *const u8,
                        arity: $nif_arity,
                        function: {
                            extern "C" fn nif_func(
                                env: rustler::wrapper::nif_interface::NIF_ENV,
                                argc: rustler::codegen_runtime::c_int,
                                argv: *const rustler::wrapper::nif_interface::NIF_TERM)
                                -> rustler::wrapper::nif_interface::NIF_TERM {
                                    rustler::codegen_runtime::handle_nif_call($nif_fun, $nif_arity, env, argc, argv)
                                }
                            nif_func
                        },
                        flags: 0,
                    }
                 ),*
            ];
            let fun_entries_len = fun_entries.len();
            let fun_entries_ptr = Box::into_raw(Box::new(fun_entries));

            let entry = rustler::wrapper::nif_interface::DEF_NIF_ENTRY {
                major: rustler::wrapper::nif_interface::NIF_MAJOR_VERSION,
                minor: rustler::wrapper::nif_interface::NIF_MINOR_VERSION,
                name: ::std::ffi::CString::new($name).unwrap().into_raw() as *const u8,
                num_of_funcs: fun_entries_len as rustler::codegen_runtime::c_int,
                funcs: fun_entries_ptr as *const rustler::wrapper::nif_interface::DEF_NIF_FUNC,
                load: Some(nif_load),
                reload: None,
                upgrade: None,
                unload: None,
                vm_variant: b"beam.vanilla\x00" as *const u8,
                options: 0,
            };
            unsafe { NIF_ENTRY = Some(entry) };

            unsafe { NIF_ENTRY.as_ref().unwrap() }
        }
        );
}
