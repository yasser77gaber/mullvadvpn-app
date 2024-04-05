#![allow(unused)]
use mullvad_proc_macro::UnwrapProto;

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
fn test_generate_unwrapped() {
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
