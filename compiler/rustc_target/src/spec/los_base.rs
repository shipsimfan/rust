use crate::spec::{
    crt_objects, LinkOutputKind, PanicStrategy, RelocModel, RelroLevel, StackProbeType,
    TargetOptions,
};

use super::{LinkArgs, LinkerFlavor, LldFlavor};

pub fn opts() -> TargetOptions {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(
        LinkerFlavor::Lld(LldFlavor::Ld),
        vec!["-z".to_string(), "max-page-size=4096".to_string()],
    );

    let pre_link_objects = crt_objects::new(&[
        (LinkOutputKind::DynamicNoPicExe, &["crt0.o", "crti.o"]),
        (LinkOutputKind::DynamicPicExe, &["crt0.o", "crti.o"]),
        (LinkOutputKind::StaticNoPicExe, &["crt0.o", "crti.o"]),
        (LinkOutputKind::StaticPicExe, &["crt0.o", "crti.o"]),
        (LinkOutputKind::DynamicDylib, &["crt0.o", "crti.o"]),
        (LinkOutputKind::StaticDylib, &["crt0.o", "crti.o"]),
    ]);

    let post_link_objects = crt_objects::all("crtn.o");

    let mut late_link_args = LinkArgs::new();
    late_link_args
        .insert(LinkerFlavor::Lld(LldFlavor::Ld), vec!["-lc".to_string(), "-lkernel".to_string()]);

    TargetOptions {
        is_builtin: true,
        os: "los".to_string(),
        vendor: "".to_string(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        lld_flavor: LldFlavor::Ld,
        pre_link_args,
        pre_link_objects,
        post_link_objects,
        late_link_args,
        executables: true,
        relocation_model: RelocModel::Static,
        exe_suffix: ".app".to_string(),
        abi_return_struct_as_int: true,
        position_independent_executables: false,
        static_position_independent_executables: false,
        needs_plt: false,
        relro_level: RelroLevel::None,
        has_elf_tls: true,
        panic_strategy: PanicStrategy::Abort,
        crt_static_allows_dylibs: false,
        crt_static_default: true,
        crt_static_respected: true,
        stack_probes: StackProbeType::Call,
        singlethread: false,
        no_builtins: true,
        ..Default::default()
    }
}
