use bitflags::bitflags;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

trait Permission {
    fn from_bits(bits: u64) -> Self;
}

#[derive(Default)]
struct PermissionVisitor<P: Permission> {
    _phantom: std::marker::PhantomData<P>,
}

impl<'de, P: Permission> Visitor<'de> for PermissionVisitor<P> {
    type Value = P;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an integer or long representing the permissions")
    }

    fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E> {
        Ok(P::from_bits(val))
    }
}

bitflags! {
    #[derive(Default)]
    pub struct UserPermission: u64 {
        const NONE = 0x00;
        const ADMIN = 0x01;
        const MANAGE_CHANNEL = 0x02;
    }
}

impl Permission for UserPermission {
    fn from_bits(bits: u64) -> Self {
        UserPermission::from_bits_truncate(bits)
    }
}

impl Serialize for UserPermission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for UserPermission {
    fn deserialize<D>(deserializer: D) -> Result<UserPermission, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(PermissionVisitor::<UserPermission>::default())
    }
}
