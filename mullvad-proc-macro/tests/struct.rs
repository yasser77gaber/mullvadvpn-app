#![allow(unused)]

use crate::proto::Timestamp;
use chrono::DateTime;
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

#[derive(IntoProto, Debug)]
pub struct Device {
    pub created: chrono::DateTime<chrono::Utc>,
}

impl IntoProto<proto::Timestamp> for chrono::DateTime<chrono::Utc> {
    fn into_proto(self) -> proto::Timestamp {
        proto::Timestamp {
            seconds: self.timestamp(),
            nanos: 0,
        }
    }
}

impl FromProto<proto::AppVersionInfo> for AppVersionInfo {
    fn from_proto(other: proto::AppVersionInfo) -> Self {
        AppVersionInfo {
            latest_beta: other.latest_beta,
            latest_stable: other.latest_stable,
            suggested_upgrade: other.suggested_upgrade,
            supported: other.supported,
        }
    }
}

mod proto {
    use super::FromProto;
    use mullvad_proc_macro::{FromProto, IntoProto};

    #[derive(Debug, FromProto)]
    pub struct AppVersionInfo {
        pub supported: bool,
        pub latest_stable: String,
        pub latest_beta: String,
        pub suggested_upgrade: Option<String>,
    }

    #[derive(Debug, FromProto)]
    pub struct Device {
        pub created: Timestamp,
    }

    #[derive(Debug)]
    pub struct Timestamp {
        pub seconds: i64,
        pub nanos: i32,
    }

    mod mullvad_types {
        use crate::FromProto;
        use chrono::TimeZone;

        #[derive(Debug)]
        pub struct AppVersionInfo {
            pub supported: bool,
            pub latest_stable: String,
            pub latest_beta: String,
            pub suggested_upgrade: Option<String>,
        }

        pub struct Device {
            pub created: chrono::DateTime<chrono::Utc>,
        }

        impl FromProto<super::Timestamp> for chrono::DateTime<chrono::Utc> {
            fn from_proto(other: super::Timestamp) -> Self {
                let naive_date_time =
                    chrono::NaiveDateTime::from_timestamp_opt(other.seconds, other.nanos as u32)
                        .unwrap();
                chrono::Utc.from_utc_datetime(&naive_date_time)
            }
        }
    }
}

trait IntoProto<T> {
    fn into_proto(self) -> T;
}

impl<T: Into<S>, S> IntoProto<S> for T {
    fn into_proto(self) -> S {
        self.into()
    }
}

trait FromProto<T> {
    fn from_proto(other: T) -> Self;
}

impl<T: From<S>, S> FromProto<S> for T {
    fn from_proto(other: S) -> Self {
        Self::from(other)
    }
}

#[test]
fn test_generate_into_proto() {
    let settings_proto = AppVersionInfo {
        latest_beta: "2025.1-beta2".to_owned(),
        latest_stable: "2025.1".to_owned(),
        suggested_upgrade: Some("2030.1".to_owned()),
        supported: false,
    };

    let settings: proto::AppVersionInfo = settings_proto.into_proto();
    dbg!(settings);

    let device_proto = Device {
        created: DateTime::default(),
    };

    let settings: proto::Device = device_proto.into_proto();
    dbg!(settings);
}
