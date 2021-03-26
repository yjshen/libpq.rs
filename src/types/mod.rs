#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Array(crate::Oid),
    Boolean,
    Composite,
    DateTime,
    Enum,
    Geometric,
    Network,
    Numeric,
    Pseudo,
    Range(crate::Oid),
    String,
    Timestamp,
    UserDefined,
    BitString,
    Unknow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    pub oid: crate::Oid,
    pub descr: &'static str,
    pub name: &'static str,
    pub kind: Kind,
}

include!("gen.rs");

impl From<Type> for crate::Oid {
    fn from(ty: Type) -> crate::Oid {
        ty.oid
    }
}
