#![allow(unused)]
use mullvad_proc_macro::{IntoProto, UnwrapProto};

#[derive(Debug)]
struct RelaySettings;
#[derive(Debug)]
struct BridgeSettings;
#[derive(Debug)]
struct BridgeState;
#[derive(Debug)]
struct TunnelOptions;
#[derive(Debug)]
struct SplitTunnelSettings;
#[derive(Debug)]
struct ObfuscationSettings;
#[derive(Debug)]
struct CustomListSettings;
#[derive(Debug)]
struct ApiAccessMethodSettings;
#[derive(Debug)]
struct RelayOverride;

#[derive(UnwrapProto, Debug)]
pub struct Settings {
    relay_settings: Option<RelaySettings>,
    bridge_settings: Option<BridgeSettings>,
    bridge_state: Option<BridgeState>,
    allow_lan: bool,
    block_when_disconnected: bool,
    auto_connect: bool,
    tunnel_options: Option<TunnelOptions>,
    show_beta_releases: bool,
    split_tunnel: Option<SplitTunnelSettings>,
    obfuscation_settings: Option<ObfuscationSettings>,
    custom_lists: Option<CustomListSettings>,
    api_access_methods: Option<ApiAccessMethodSettings>,
    relay_overrides: Vec<RelayOverride>,
}

#[test]
fn unwrap_setting() {
    let settings_proto = Settings {
        relay_settings: Some(RelaySettings),
        bridge_settings: Some(BridgeSettings),
        bridge_state: Some(BridgeState),
        allow_lan: true,
        block_when_disconnected: false,
        auto_connect: true,
        tunnel_options: Some(TunnelOptions),
        show_beta_releases: true,
        split_tunnel: Some(SplitTunnelSettings),
        obfuscation_settings: Some(ObfuscationSettings),
        custom_lists: Some(CustomListSettings),
        api_access_methods: Some(ApiAccessMethodSettings),
        relay_overrides: vec![],
    };

    let settings = SettingsUnwrapped::try_from(settings_proto).expect("Failed to parse setting");
    dbg!(settings);
}

// proto -> real

#[derive(IntoProto, Debug)]
pub struct AppVersionInfo {
    pub supported: bool,
    pub latest_stable: String,
    pub latest_beta: String,
    pub suggested_upgrade: Option<String>,
}

pub type AppVersion = String;

mod proto {
    #[derive(Debug)]
    pub struct AppVersionInfo {
        pub supported: bool,
        pub latest_stable: String,
        pub latest_beta: String,
        pub suggested_upgrade: Option<String>,
    }
}

trait IntoProto<T> {
    fn into_proto(self) -> T;
}
impl<T: IntoProto<S>, S> IntoProto<Option<S>> for Option<T> {
    fn into_proto(self) -> Option<S> {
        self.map(|val| val.into_proto())
    }
}

macro_rules! impl_into_proto_for_value_type {
    ($ty:ty) => {
        impl IntoProto<$ty> for $ty {
            fn into_proto(self) -> $ty {
                self
            }
        }
    };
}

impl_into_proto_for_value_type!(bool);
impl_into_proto_for_value_type!(String);

#[test]
fn test_generate_into_proto() {
    let settings_proto = AppVersionInfo {
        latest_beta: "2025.1-beta2".to_owned(),
        latest_stable: "2025.1".to_owned(),
        suggested_upgrade: Some("2030.1".to_owned()),
        supported: false,
    };

    let settings = settings_proto.into_proto();
    dbg!(settings);
}
