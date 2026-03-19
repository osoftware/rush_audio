pub mod api;
mod frb_generated;

#[cfg(target_os = "android")]
mod init_android_context {
    use jni::{JNIEnv, objects::GlobalRef, objects::JClass, objects::JObject};
    use std::ffi::c_void;
    use std::sync::OnceLock;

    static CTX: OnceLock<GlobalRef> = OnceLock::new();

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_pl_net_orb_rush_1synth_RushSynthPlugin_init_1android(
        env: JNIEnv,
        _class: JClass,
        ctx: JObject,
    ) {
        let global_ref = env.new_global_ref(&ctx).expect("to make global reference");
        let vm = env.get_java_vm().unwrap();
        let vm = vm.get_java_vm_pointer() as *mut c_void;
        unsafe {
            ndk_context::initialize_android_context(vm, global_ref.as_obj().as_raw() as _);
        }
        CTX.get_or_init(|| global_ref);

        flutter_rust_bridge::setup_default_user_utils();
    }
}
