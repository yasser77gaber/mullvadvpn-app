use crate::types::proto;

trait IntoProto<T> {
    fn into_proto(self) -> T;
}

impl IntoProto<proto::AppVersionInfo> for mullvad_types::version::AppVersionInfo {
    fn into_proto(self) -> proto::AppVersionInfo {
        proto::AppVersionInfo {
            supported: self.supported,
            latest_stable: self.latest_stable,
            latest_beta: self.latest_beta,
            suggested_upgrade: self.suggested_upgrade,
        }
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

impl From<mullvad_types::version::AppVersionInfo> for proto::AppVersionInfo {
    fn from(version_info: mullvad_types::version::AppVersionInfo) -> Self {
        version_info.into_proto()
    }
}

trait FromProto<T> {
    fn from_proto(value: T) -> Self;
}

impl FromProto<proto::AppVersionInfo> for mullvad_types::version::AppVersionInfo {
    fn from_proto(value: proto::AppVersionInfo) -> mullvad_types::version::AppVersionInfo {
        Self {
            supported: value.supported,
            latest_stable: value.latest_stable,
            latest_beta: value.latest_beta,
            suggested_upgrade: value.suggested_upgrade,
        }
    }
}

impl From<proto::AppVersionInfo> for mullvad_types::version::AppVersionInfo {
    fn from(version_info: proto::AppVersionInfo) -> Self {
        mullvad_types::version::AppVersionInfo::from_proto(version_info)
    }
}
