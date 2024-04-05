fn main() {
    tonic_build::configure()
        //.type_attribute("mullvad_daemon.management_interface.UUID", "#[derive(::mullvad_proc_macro::FromProto)]")
        .compile(&["proto/management_interface.proto"], &["proto"])
        .unwrap();
}
