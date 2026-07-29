pub struct Config {
    pub c_prefix: &'static str,
    pub c_types_prefix: &'static str,
    pub custom_types: &'static [CustomType],
    pub observer: bool,
    pub c_runtime: bool,
    pub blocking_runtime: bool,
    pub sources: &'static [Source],
    pub reexports: &'static [Reexport]
}

pub struct Reexport {
    pub name: &'static str,
    pub dir: &'static str,
    pub c_prefix: &'static str,
    pub c_types_prefix: &'static str,
    pub skip: &'static [&'static str]
}

pub struct Source {
    pub crate_path: &'static str,
    pub dir: &'static str,
    pub type_files: &'static [(&'static str, RemoteKind)],
    pub const_files: &'static [&'static str],
    pub renames: &'static [Rename],
    pub sanitized: &'static [Sanitized],
    pub skip_types: &'static [&'static str]
}

pub struct Rename {
    pub source: &'static str,
    pub target: &'static str
}

pub struct Sanitized {
    pub file: &'static str,
    pub name: &'static str,
    pub skip_fields: &'static [&'static str],
    pub native: Option<&'static str>
}

pub struct CustomType {
    pub alias: &'static str,
    pub target: &'static str,
    pub over: &'static str,
    pub pattern: &'static str,
    pub lower: &'static str,
    pub try_lift: &'static str
}

pub const COW_STR: CustomType = CustomType {
    alias: "StaticStrCow",
    target: "std::borrow::Cow<'static, str>",
    over: "String",
    pattern: "Cow<'static,str>",
    lower: "|value| value.into_owned()",
    try_lift: "|value| Ok(std::borrow::Cow::Owned(value))"
};

#[derive(Clone, Copy)]
pub enum RemoteKind {
    Data,
    Error
}
