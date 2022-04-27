use crate::spec::{
    crt_objects, LinkArgs, LinkOutputKind, LinkerFlavor, LldFlavor, PanicStrategy, TargetOptions,
};

pub fn opts() -> TargetOptions {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Lld(LldFlavor::Ld), vec![]);

    TargetOptions {
        os: "oceanic".into(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".into()),
        dynamic_linking: true,
        executables: true,
        panic_strategy: PanicStrategy::Abort,
        pre_link_args,
        pre_link_objects: crt_objects::new(&[
            (LinkOutputKind::DynamicNoPicExe, &["crt0.o"]),
            (LinkOutputKind::DynamicPicExe, &["crt0.o"]),
            (LinkOutputKind::StaticNoPicExe, &["crt0.o"]),
            (LinkOutputKind::StaticPicExe, &["crt0.o"]),
        ]),
        position_independent_executables: true,
        has_thread_local: true,
        ..Default::default()
    }
}
