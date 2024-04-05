#[allow(unused)]
use mullvad_proc_macro::UnwrapProto;

#[derive(UnwrapProto, Debug, PartialEq, Eq)]
struct TestMe {
    name: String,
    middle_name: Option<String>,
    company: Option<String>,
    age: u8,
}

#[test]
fn test_generate_unwrapped() {
    let me = TestMe {
        name: "Sebastian".into(),
        middle_name: Some("Ludvig".into()),
        company: Some("Mullvad VPN".into()),
        age: 26,
    };
    println!("me: {me:#?}");

    let unwrapped_me = TestMeUnwrapped::try_from(me).unwrap();
    println!("unwrapped_me: {unwrapped_me:#?}");

    assert_eq!(
        unwrapped_me,
        TestMeUnwrapped {
            name: "Sebastian".to_owned(),
            middle_name: "Ludvig".to_owned(),
            company: "Mullvad VPN".to_owned(),
            age: 26,
        }
    );
}
