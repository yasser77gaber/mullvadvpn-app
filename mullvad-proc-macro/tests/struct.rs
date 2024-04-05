use mullvad_proc_macro::UnwrapProto;

#[derive(UnwrapProto)]
struct TestMe {
    name: String,
    middle_name: Option<String>,
}
