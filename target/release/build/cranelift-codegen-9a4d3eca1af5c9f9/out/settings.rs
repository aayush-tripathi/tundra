#[derive(Clone, Hash)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:426
/// Flags group `shared`.
pub struct Flags {
    bytes: [u8; 12], // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:429
}
impl Flags {
    /// Create flags shared settings group.
    #[allow(unused_variables)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:26
    pub fn new(builder: Builder) -> Self {
        let bvec = builder.state_for("shared"); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:28
        let mut shared = Self { bytes: [0; 12] }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:29
        debug_assert_eq!(bvec.len(), 12); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:35
        shared.bytes[0..12].copy_from_slice(&bvec); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:40
        shared // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:61
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> + use<> {
        let mut bytes = [0; 12]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:71
        bytes.copy_from_slice(&self.bytes[0..12]); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:72
        DESCRIPTORS.iter().filter_map(move |d| {
            let values = match &d.detail {
                detail::Detail::Preset => return None, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:75
                detail::Detail::Enum { last, enumerators } => Some(TEMPLATE.enums(*last, *enumerators)), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:76
                _ => None // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:77
            }
            ; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:79
            Some(Value { name: d.name, detail: d.detail, values, value: bytes[d.offset as usize] }) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:80
        }
        ) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:82
    }
}
/// Values for `shared.regalloc_algorithm`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:144
pub enum RegallocAlgorithm {
    /// `backtracking`.
    Backtracking, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
}
impl RegallocAlgorithm {
    /// Returns a slice with all possible [RegallocAlgorithm] values. // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:89
    pub fn all() -> &'static [RegallocAlgorithm] {
        &[ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:95
            Self::Backtracking, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
        ] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:101
    }
}
impl fmt::Display for RegallocAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Backtracking => "backtracking", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
        }
        ) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:116
    }
}
impl core::str::FromStr for RegallocAlgorithm {
    type Err = (); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:122
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "backtracking" => Ok(Self::Backtracking), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            _ => Err(()), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:128
        }
    }
}
/// Values for `shared.opt_level`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:144
pub enum OptLevel {
    /// `none`.
    None, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `speed`.
    Speed, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `speed_and_size`.
    SpeedAndSize, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
}
impl OptLevel {
    /// Returns a slice with all possible [OptLevel] values. // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:89
    pub fn all() -> &'static [OptLevel] {
        &[ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:95
            Self::None, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::Speed, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::SpeedAndSize, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
        ] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:101
    }
}
impl fmt::Display for OptLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::None => "none", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::Speed => "speed", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::SpeedAndSize => "speed_and_size", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
        }
        ) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:116
    }
}
impl core::str::FromStr for OptLevel {
    type Err = (); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:122
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "speed" => Ok(Self::Speed), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "speed_and_size" => Ok(Self::SpeedAndSize), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            _ => Err(()), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:128
        }
    }
}
/// Values for `shared.tls_model`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:144
pub enum TlsModel {
    /// `none`.
    None, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `elf_gd`.
    ElfGd, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `macho`.
    Macho, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `coff`.
    Coff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
}
impl TlsModel {
    /// Returns a slice with all possible [TlsModel] values. // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:89
    pub fn all() -> &'static [TlsModel] {
        &[ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:95
            Self::None, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::ElfGd, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::Macho, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::Coff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
        ] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:101
    }
}
impl fmt::Display for TlsModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::None => "none", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::ElfGd => "elf_gd", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::Macho => "macho", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::Coff => "coff", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
        }
        ) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:116
    }
}
impl core::str::FromStr for TlsModel {
    type Err = (); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:122
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "elf_gd" => Ok(Self::ElfGd), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "macho" => Ok(Self::Macho), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "coff" => Ok(Self::Coff), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            _ => Err(()), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:128
        }
    }
}
/// Values for `shared.stack_switch_model`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:144
pub enum StackSwitchModel {
    /// `none`.
    None, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `basic`.
    Basic, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `update_windows_tib`.
    UpdateWindowsTib, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
}
impl StackSwitchModel {
    /// Returns a slice with all possible [StackSwitchModel] values. // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:89
    pub fn all() -> &'static [StackSwitchModel] {
        &[ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:95
            Self::None, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::Basic, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::UpdateWindowsTib, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
        ] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:101
    }
}
impl fmt::Display for StackSwitchModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::None => "none", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::Basic => "basic", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::UpdateWindowsTib => "update_windows_tib", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
        }
        ) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:116
    }
}
impl core::str::FromStr for StackSwitchModel {
    type Err = (); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:122
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "basic" => Ok(Self::Basic), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "update_windows_tib" => Ok(Self::UpdateWindowsTib), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            _ => Err(()), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:128
        }
    }
}
/// Values for `shared.libcall_call_conv`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:144
pub enum LibcallCallConv {
    /// `isa_default`.
    IsaDefault, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `fast`.
    Fast, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `cold`.
    Cold, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `system_v`.
    SystemV, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `windows_fastcall`.
    WindowsFastcall, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `apple_aarch64`.
    AppleAarch64, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `probestack`.
    Probestack, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
}
impl LibcallCallConv {
    /// Returns a slice with all possible [LibcallCallConv] values. // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:89
    pub fn all() -> &'static [LibcallCallConv] {
        &[ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:95
            Self::IsaDefault, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::Fast, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::Cold, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::SystemV, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::WindowsFastcall, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::AppleAarch64, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::Probestack, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
        ] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:101
    }
}
impl fmt::Display for LibcallCallConv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::IsaDefault => "isa_default", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::Fast => "fast", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::Cold => "cold", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::SystemV => "system_v", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::WindowsFastcall => "windows_fastcall", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::AppleAarch64 => "apple_aarch64", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::Probestack => "probestack", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
        }
        ) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:116
    }
}
impl core::str::FromStr for LibcallCallConv {
    type Err = (); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:122
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "isa_default" => Ok(Self::IsaDefault), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "fast" => Ok(Self::Fast), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "cold" => Ok(Self::Cold), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "system_v" => Ok(Self::SystemV), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "windows_fastcall" => Ok(Self::WindowsFastcall), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "apple_aarch64" => Ok(Self::AppleAarch64), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "probestack" => Ok(Self::Probestack), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            _ => Err(()), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:128
        }
    }
}
/// Values for `shared.probestack_strategy`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:144
pub enum ProbestackStrategy {
    /// `outline`.
    Outline, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
    /// `inline`.
    Inline, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:148
}
impl ProbestackStrategy {
    /// Returns a slice with all possible [ProbestackStrategy] values. // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:89
    pub fn all() -> &'static [ProbestackStrategy] {
        &[ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:95
            Self::Outline, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
            Self::Inline, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:98
        ] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:101
    }
}
impl fmt::Display for ProbestackStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Outline => "outline", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
            Self::Inline => "inline", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:113
        }
        ) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:116
    }
}
impl core::str::FromStr for ProbestackStrategy {
    type Err = (); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:122
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "outline" => Ok(Self::Outline), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            "inline" => Ok(Self::Inline), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:126
            _ => Err(()), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:128
        }
    }
}
/// User-defined settings.
#[allow(dead_code)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:206
impl Flags {
    /// Get a view of the boolean predicates.
    pub fn predicate_view(&self) -> crate::settings::PredicateView {
        crate::settings::PredicateView::new(&self.bytes[9..]) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:212
    }
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[9 + p / 8] & (1 << (p % 8)) != 0 // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:223
    }
    /// Algorithm to use in register allocator.
    ///
    /// Supported options:
    ///
    /// - `backtracking`: A backtracking allocator with range splitting; more expensive
    ///                   but generates better code.
    ///
    /// Note that the `single_pass` option is currently disabled because it does not
    /// have adequate support for the kinds of allocations required by exception
    /// handling (https://github.com/bytecodealliance/regalloc2/issues/217).
    pub fn regalloc_algorithm(&self) -> RegallocAlgorithm {
        match self.bytes[0] {
            0 => {
                RegallocAlgorithm::Backtracking
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Optimization level for generated code.
    ///
    /// Supported levels:
    ///
    /// - `none`: Minimise compile time by disabling most optimizations.
    /// - `speed`: Generate the fastest possible code
    /// - `speed_and_size`: like "speed", but also perform transformations aimed at reducing code size.
    pub fn opt_level(&self) -> OptLevel {
        match self.bytes[1] {
            0 => {
                OptLevel::None
            }
            1 => {
                OptLevel::Speed
            }
            2 => {
                OptLevel::SpeedAndSize
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Defines the model used to perform TLS accesses.
    pub fn tls_model(&self) -> TlsModel {
        match self.bytes[2] {
            3 => {
                TlsModel::Coff
            }
            1 => {
                TlsModel::ElfGd
            }
            2 => {
                TlsModel::Macho
            }
            0 => {
                TlsModel::None
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Defines the model used to performing stack switching.
    ///
    /// This determines the compilation of `stack_switch` instructions. If
    /// set to `basic`, we simply save all registers, update stack pointer
    /// and frame pointer (if needed), and jump to the target IP.
    /// If set to `update_windows_tib`, we *additionally* update information
    /// about the active stack in Windows' Thread Information Block.
    pub fn stack_switch_model(&self) -> StackSwitchModel {
        match self.bytes[3] {
            1 => {
                StackSwitchModel::Basic
            }
            0 => {
                StackSwitchModel::None
            }
            2 => {
                StackSwitchModel::UpdateWindowsTib
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// Defines the calling convention to use for LibCalls call expansion.
    ///
    /// This may be different from the ISA default calling convention.
    ///
    /// The default value is to use the same calling convention as the ISA
    /// default calling convention.
    ///
    /// This list should be kept in sync with the list of calling
    /// conventions available in isa/call_conv.rs.
    pub fn libcall_call_conv(&self) -> LibcallCallConv {
        match self.bytes[4] {
            5 => {
                LibcallCallConv::AppleAarch64
            }
            2 => {
                LibcallCallConv::Cold
            }
            1 => {
                LibcallCallConv::Fast
            }
            0 => {
                LibcallCallConv::IsaDefault
            }
            6 => {
                LibcallCallConv::Probestack
            }
            3 => {
                LibcallCallConv::SystemV
            }
            4 => {
                LibcallCallConv::WindowsFastcall
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// The log2 of the size of the stack guard region.
    ///
    /// Stack frames larger than this size will have stack overflow checked
    /// by calling the probestack function.
    ///
    /// The default is 12, which translates to a size of 4096.
    pub fn probestack_size_log2(&self) -> u8 {
        self.bytes[5] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:187
    }
    /// Controls what kinds of stack probes are emitted.
    ///
    /// Supported strategies:
    ///
    /// - `outline`: Always emits stack probes as calls to a probe stack function.
    /// - `inline`: Always emits inline stack probes.
    pub fn probestack_strategy(&self) -> ProbestackStrategy {
        match self.bytes[6] {
            1 => {
                ProbestackStrategy::Inline
            }
            0 => {
                ProbestackStrategy::Outline
            }
            _ => {
                panic!("Invalid enum value")
            }
        }
    }
    /// The log2 of the size to insert dummy padding between basic blocks
    ///
    /// This is a debugging option for stressing various cases during code
    /// generation without requiring large functions. This will insert
    /// 0-byte padding between basic blocks of the specified size.
    ///
    /// The amount of padding inserted two raised to the power of this value
    /// minus one. If this value is 0 then no padding is inserted.
    ///
    /// The default for this option is 0 to insert no padding as it's only
    /// intended for testing and development.
    pub fn bb_padding_log2_minus_one(&self) -> u8 {
        self.bytes[7] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:187
    }
    /// The log2 of the minimum alignment of functions
    /// The bigger of this value and the default alignment will be used as actual alignment.
    pub fn log2_min_function_alignment(&self) -> u8 {
        self.bytes[8] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:187
    }
    /// Enable the symbolic checker for register allocation.
    ///
    /// This performs a verification that the register allocator preserves
    /// equivalent dataflow with respect to the original (pre-regalloc)
    /// program. This analysis is somewhat expensive. However, if it succeeds,
    /// it provides independent evidence (by a carefully-reviewed, from-first-principles
    /// analysis) that no regalloc bugs were triggered for the particular compilations
    /// performed. This is a valuable assurance to have as regalloc bugs can be
    /// very dangerous and difficult to debug.
    pub fn regalloc_checker(&self) -> bool {
        self.numbered_predicate(0) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable verbose debug logs for regalloc2.
    ///
    /// This adds extra logging for regalloc2 output, that is quite valuable to understand
    /// decisions taken by the register allocator as well as debugging it. It is disabled by
    /// default, as it can cause many log calls which can slow down compilation by a large
    /// amount.
    pub fn regalloc_verbose_logs(&self) -> bool {
        self.numbered_predicate(1) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Do redundant-load optimizations with alias analysis.
    ///
    /// This enables the use of a simple alias analysis to optimize away redundant loads.
    /// Only effective when `opt_level` is `speed` or `speed_and_size`.
    pub fn enable_alias_analysis(&self) -> bool {
        self.numbered_predicate(2) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Run the Cranelift IR verifier at strategic times during compilation.
    ///
    /// This makes compilation slower but catches many bugs. The verifier is always enabled by
    /// default, which is useful during development.
    pub fn enable_verifier(&self) -> bool {
        self.numbered_predicate(3) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable proof-carrying code translation validation.
    ///
    /// This adds a proof-carrying-code mode. Proof-carrying code (PCC) is a strategy to verify
    /// that the compiler preserves certain properties or invariants in the compiled code.
    /// For example, a frontend that translates WebAssembly to CLIF can embed PCC facts in
    /// the CLIF, and Cranelift will verify that the final machine code satisfies the stated
    /// facts at each intermediate computed value. Loads and stores can be marked as "checked"
    /// and their memory effects can be verified as safe.
    pub fn enable_pcc(&self) -> bool {
        self.numbered_predicate(4) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable Position-Independent Code generation.
    pub fn is_pic(&self) -> bool {
        self.numbered_predicate(5) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Use colocated libcalls.
    ///
    /// Generate code that assumes that libcalls can be declared "colocated",
    /// meaning they will be defined along with the current function, such that
    /// they can use more efficient addressing.
    pub fn use_colocated_libcalls(&self) -> bool {
        self.numbered_predicate(6) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable the use of floating-point instructions.
    ///
    /// Disabling use of floating-point instructions is not yet implemented.
    pub fn enable_float(&self) -> bool {
        self.numbered_predicate(7) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable NaN canonicalization.
    ///
    /// This replaces NaNs with a single canonical value, for users requiring
    /// entirely deterministic WebAssembly computation. This is not required
    /// by the WebAssembly spec, so it is not enabled by default.
    pub fn enable_nan_canonicalization(&self) -> bool {
        self.numbered_predicate(8) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable the use of the pinned register.
    ///
    /// This register is excluded from register allocation, and is completely under the control of
    /// the end-user. It is possible to read it via the get_pinned_reg instruction, and to set it
    /// with the set_pinned_reg instruction.
    pub fn enable_pinned_reg(&self) -> bool {
        self.numbered_predicate(9) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable the use of atomic instructions
    pub fn enable_atomics(&self) -> bool {
        self.numbered_predicate(10) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable safepoint instruction insertions.
    ///
    /// This will allow the emit_stack_maps() function to insert the safepoint
    /// instruction on top of calls and interrupt traps in order to display the
    /// live reference values at that point in the program.
    pub fn enable_safepoints(&self) -> bool {
        self.numbered_predicate(11) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable various ABI extensions defined by LLVM's behavior.
    ///
    /// In some cases, LLVM's implementation of an ABI (calling convention)
    /// goes beyond a standard and supports additional argument types or
    /// behavior. This option instructs Cranelift codegen to follow LLVM's
    /// behavior where applicable.
    ///
    /// Currently, this applies only to Windows Fastcall on x86-64, and
    /// allows an `i128` argument to be spread across two 64-bit integer
    /// registers. The Fastcall implementation otherwise does not support
    /// `i128` arguments, and will panic if they are present and this
    /// option is not set.
    pub fn enable_llvm_abi_extensions(&self) -> bool {
        self.numbered_predicate(12) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable support for sret arg introduction when there are too many ret vals.
    ///
    /// When there are more returns than available return registers, the
    /// return value has to be returned through the introduction of a
    /// return area pointer. Normally this return area pointer has to be
    /// introduced as `ArgumentPurpose::StructReturn` parameter, but for
    /// backward compatibility reasons Cranelift also supports implicitly
    /// introducing this parameter and writing the return values through it.
    ///
    /// **This option currently does not conform to platform ABIs and the
    /// used ABI should not be assumed to remain the same between Cranelift
    /// versions.**
    ///
    /// This option is **deprecated** and will be removed in the future.
    ///
    /// Because of the above issues, and complexities of native ABI support
    /// for the concept in general, Cranelift's support for multiple return
    /// values may also be removed in the future (#9510). For the most
    /// robust solution, it is recommended to build a convention on top of
    /// Cranelift's primitives for passing multiple return values, for
    /// example by allocating a stackslot in the caller, passing it as an
    /// explicit StructReturn argument, storing return values in the callee,
    /// and loading results in the caller.
    pub fn enable_multi_ret_implicit_sret(&self) -> bool {
        self.numbered_predicate(13) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Generate unwind information.
    ///
    /// This increases metadata size and compile time, but allows for the
    /// debugger to trace frames, is needed for GC tracing that relies on
    /// libunwind (such as in Wasmtime), and is unconditionally needed on
    /// certain platforms (such as Windows) that must always be able to unwind.
    pub fn unwind_info(&self) -> bool {
        self.numbered_predicate(14) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Preserve frame pointers
    ///
    /// Preserving frame pointers -- even inside leaf functions -- makes it
    /// easy to capture the stack of a running program, without requiring any
    /// side tables or metadata (like `.eh_frame` sections). Many sampling
    /// profilers and similar tools walk frame pointers to capture stacks.
    /// Enabling this option will play nice with those tools.
    pub fn preserve_frame_pointers(&self) -> bool {
        self.numbered_predicate(15) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Generate CFG metadata for machine code.
    ///
    /// This increases metadata size and compile time, but allows for the
    /// embedder to more easily post-process or analyze the generated
    /// machine code. It provides code offsets for the start of each
    /// basic block in the generated machine code, and a list of CFG
    /// edges (with blocks identified by start offsets) between them.
    /// This is useful for, e.g., machine-code analyses that verify certain
    /// properties of the generated code.
    pub fn machine_code_cfg_info(&self) -> bool {
        self.numbered_predicate(16) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable the use of stack probes for supported calling conventions.
    pub fn enable_probestack(&self) -> bool {
        self.numbered_predicate(17) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable the use of jump tables in generated machine code.
    pub fn enable_jump_tables(&self) -> bool {
        self.numbered_predicate(18) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable Spectre mitigation on heap bounds checks.
    ///
    /// This is a no-op for any heap that needs no bounds checks; e.g.,
    /// if the limit is static and the guard region is large enough that
    /// the index cannot reach past it.
    ///
    /// This option is enabled by default because it is highly
    /// recommended for secure sandboxing. The embedder should consider
    /// the security implications carefully before disabling this option.
    pub fn enable_heap_access_spectre_mitigation(&self) -> bool {
        self.numbered_predicate(19) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable Spectre mitigation on table bounds checks.
    ///
    /// This option uses a conditional move to ensure that when a table
    /// access index is bounds-checked and a conditional branch is used
    /// for the out-of-bounds case, a misspeculation of that conditional
    /// branch (falsely predicted in-bounds) will select an in-bounds
    /// index to load on the speculative path.
    ///
    /// This option is enabled by default because it is highly
    /// recommended for secure sandboxing. The embedder should consider
    /// the security implications carefully before disabling this option.
    pub fn enable_table_access_spectre_mitigation(&self) -> bool {
        self.numbered_predicate(20) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Enable additional checks for debugging the incremental compilation cache.
    ///
    /// Enables additional checks that are useful during development of the incremental
    /// compilation cache. This should be mostly useful for Cranelift hackers, as well as for
    /// helping to debug false incremental cache positives for embedders.
    ///
    /// This option is disabled by default and requires enabling the "incremental-cache" Cargo
    /// feature in cranelift-codegen.
    pub fn enable_incremental_compilation_cache_checks(&self) -> bool {
        self.numbered_predicate(21) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
}
static DESCRIPTORS: [detail::Descriptor; 31] = [ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:262
    detail::Descriptor {
        name: "regalloc_algorithm", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Algorithm to use in register allocator.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Enum { last: 0, enumerators: 0 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:283
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "opt_level", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Optimization level for generated code.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Enum { last: 2, enumerators: 1 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:283
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "tls_model", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Defines the model used to perform TLS accesses.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 2, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Enum { last: 3, enumerators: 4 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:283
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "stack_switch_model", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Defines the model used to performing stack switching.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 3, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Enum { last: 2, enumerators: 8 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:283
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "libcall_call_conv", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Defines the calling convention to use for LibCalls call expansion.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 4, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Enum { last: 6, enumerators: 11 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:283
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "probestack_size_log2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "The log2 of the size of the stack guard region.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 5, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Num, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:291
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "probestack_strategy", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Controls what kinds of stack probes are emitted.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 6, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Enum { last: 1, enumerators: 18 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:283
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "bb_padding_log2_minus_one", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "The log2 of the size to insert dummy padding between basic blocks", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 7, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Num, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:291
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "log2_min_function_alignment", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "The log2 of the minimum alignment of functions", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 8, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Num, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:291
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "regalloc_checker", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable the symbolic checker for register allocation.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 0 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "regalloc_verbose_logs", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable verbose debug logs for regalloc2.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 1 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_alias_analysis", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Do redundant-load optimizations with alias analysis.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 2 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_verifier", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Run the Cranelift IR verifier at strategic times during compilation.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 3 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_pcc", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable proof-carrying code translation validation.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 4 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "is_pic", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable Position-Independent Code generation.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 5 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "use_colocated_libcalls", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Use colocated libcalls.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 6 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_float", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable the use of floating-point instructions.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 7 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_nan_canonicalization", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable NaN canonicalization.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 0 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_pinned_reg", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable the use of the pinned register.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 1 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_atomics", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable the use of atomic instructions", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 2 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_safepoints", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable safepoint instruction insertions.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 3 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_llvm_abi_extensions", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable various ABI extensions defined by LLVM's behavior.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 4 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_multi_ret_implicit_sret", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable support for sret arg introduction when there are too many ret vals.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 5 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "unwind_info", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Generate unwind information.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 6 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "preserve_frame_pointers", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Preserve frame pointers", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 7 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "machine_code_cfg_info", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Generate CFG metadata for machine code.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 11, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 0 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_probestack", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable the use of stack probes for supported calling conventions.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 11, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 1 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_jump_tables", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable the use of jump tables in generated machine code.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 11, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 2 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_heap_access_spectre_mitigation", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable Spectre mitigation on heap bounds checks.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 11, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 3 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_table_access_spectre_mitigation", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable Spectre mitigation on table bounds checks.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 11, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 4 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "enable_incremental_compilation_cache_checks", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Enable additional checks for debugging the incremental compilation cache.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 11, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 5 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:313
static ENUMERATORS: [&str; 20] = [ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:316
    "backtracking", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "none", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "speed", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "speed_and_size", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "none", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "elf_gd", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "macho", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "coff", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "none", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "basic", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "update_windows_tib", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "isa_default", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "fast", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "cold", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "system_v", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "windows_fastcall", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "apple_aarch64", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "probestack", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "outline", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
    "inline", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:319
]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:322
static HASH_TABLE: [u16; 64] = [ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:332
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    2, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    11, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    29, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    13, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    24, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    30, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    20, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    18, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    7, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    6, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    3, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    14, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    22, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    26, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    8, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    17, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    12, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    19, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    21, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    23, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    5, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    28, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    25, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    27, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    15, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    4, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    16, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:348
static PRESETS: [(u8, u8); 0] = [ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:351
]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:368
static TEMPLATE: detail::Template = detail::Template {
    name: "shared", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:383
    descriptors: &DESCRIPTORS, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:384
    enumerators: &ENUMERATORS, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:385
    hash_table: &HASH_TABLE, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:386
    defaults: &[0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x8c, 0x44, 0x1c], // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:387
    presets: &PRESETS, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:388
}
; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:391
/// Create a `settings::Builder` for the shared settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:398
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[shared]")?; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:407
        for d in &DESCRIPTORS {
            if !d.detail.is_preset() {
                write!(f, "{} = ", d.name)?; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:410
                TEMPLATE.format_toml_value(d.detail, self.bytes[d.offset as usize], f)?; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:411
                writeln!(f)?; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:415
            }
        }
        Ok(()) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:418
    }
}
