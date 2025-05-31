#[derive(Clone, Hash)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:426
/// Flags group `x86`.
pub struct Flags {
    bytes: [u8; 5], // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:429
}
impl Flags {
    /// Create flags x86 settings group.
    #[allow(unused_variables)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:26
    pub fn new(shared: &settings::Flags, builder: &Builder) -> Self {
        let bvec = builder.state_for("x86"); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:28
        let mut x86 = Self { bytes: [0; 5] }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:29
        debug_assert_eq!(bvec.len(), 3); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:35
        x86.bytes[0..3].copy_from_slice(&bvec); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:40
        // Precompute #17.
        if x86.has_avx() {
            x86.bytes[2] |= 1 << 1; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #18.
        if x86.has_avx() && x86.has_avx2() {
            x86.bytes[2] |= 1 << 2; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #19.
        if x86.has_avx512bitalg() {
            x86.bytes[2] |= 1 << 3; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #20.
        if x86.has_avx512dq() {
            x86.bytes[2] |= 1 << 4; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #21.
        if x86.has_avx512f() {
            x86.bytes[2] |= 1 << 5; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #22.
        if x86.has_avx512vbmi() {
            x86.bytes[2] |= 1 << 6; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #23.
        if x86.has_avx512vl() {
            x86.bytes[2] |= 1 << 7; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #24.
        if x86.has_bmi1() {
            x86.bytes[3] |= 1 << 0; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #25.
        if x86.has_bmi2() {
            x86.bytes[3] |= 1 << 1; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #26.
        if x86.has_cmpxchg16b() {
            x86.bytes[3] |= 1 << 2; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #27.
        if x86.has_avx() && x86.has_fma() {
            x86.bytes[3] |= 1 << 3; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #28.
        if x86.has_lzcnt() {
            x86.bytes[3] |= 1 << 4; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #29.
        if x86.has_popcnt() && x86.has_sse42() {
            x86.bytes[3] |= 1 << 5; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #30.
        if x86.has_sse41() {
            x86.bytes[3] |= 1 << 6; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #31.
        if x86.has_sse41() && x86.has_sse42() {
            x86.bytes[3] |= 1 << 7; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        // Precompute #32.
        if x86.has_ssse3() {
            x86.bytes[4] |= 1 << 0; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:51
        }
        x86 // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:61
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> + use<> {
        let mut bytes = [0; 3]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:71
        bytes.copy_from_slice(&self.bytes[0..3]); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:72
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
/// User-defined settings.
#[allow(dead_code)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:206
impl Flags {
    /// Get a view of the boolean predicates.
    pub fn predicate_view(&self) -> crate::settings::PredicateView {
        crate::settings::PredicateView::new(&self.bytes[0..]) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:212
    }
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[0 + p / 8] & (1 << (p % 8)) != 0 // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:223
    }
    /// Has support for SSE3.
    /// SSE3: CPUID.01H:ECX.SSE3[bit 0]
    pub fn has_sse3(&self) -> bool {
        self.numbered_predicate(0) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for SSSE3.
    /// SSSE3: CPUID.01H:ECX.SSSE3[bit 9]
    pub fn has_ssse3(&self) -> bool {
        self.numbered_predicate(1) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for CMPXCHG16b.
    /// CMPXCHG16b: CPUID.01H:ECX.CMPXCHG16B[bit 13]
    pub fn has_cmpxchg16b(&self) -> bool {
        self.numbered_predicate(2) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for SSE4.1.
    /// SSE4.1: CPUID.01H:ECX.SSE4_1[bit 19]
    pub fn has_sse41(&self) -> bool {
        self.numbered_predicate(3) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for SSE4.2.
    /// SSE4.2: CPUID.01H:ECX.SSE4_2[bit 20]
    pub fn has_sse42(&self) -> bool {
        self.numbered_predicate(4) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for AVX.
    /// AVX: CPUID.01H:ECX.AVX[bit 28]
    pub fn has_avx(&self) -> bool {
        self.numbered_predicate(5) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for AVX2.
    /// AVX2: CPUID.07H:EBX.AVX2[bit 5]
    pub fn has_avx2(&self) -> bool {
        self.numbered_predicate(6) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for FMA.
    /// FMA: CPUID.01H:ECX.FMA[bit 12]
    pub fn has_fma(&self) -> bool {
        self.numbered_predicate(7) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for AVX512BITALG.
    /// AVX512BITALG: CPUID.07H:ECX.AVX512BITALG[bit 12]
    pub fn has_avx512bitalg(&self) -> bool {
        self.numbered_predicate(8) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for AVX512DQ.
    /// AVX512DQ: CPUID.07H:EBX.AVX512DQ[bit 17]
    pub fn has_avx512dq(&self) -> bool {
        self.numbered_predicate(9) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for AVX512VL.
    /// AVX512VL: CPUID.07H:EBX.AVX512VL[bit 31]
    pub fn has_avx512vl(&self) -> bool {
        self.numbered_predicate(10) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for AVX512VMBI.
    /// AVX512VBMI: CPUID.07H:ECX.AVX512VBMI[bit 1]
    pub fn has_avx512vbmi(&self) -> bool {
        self.numbered_predicate(11) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for AVX512F.
    /// AVX512F: CPUID.07H:EBX.AVX512F[bit 16]
    pub fn has_avx512f(&self) -> bool {
        self.numbered_predicate(12) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for POPCNT.
    /// POPCNT: CPUID.01H:ECX.POPCNT[bit 23]
    pub fn has_popcnt(&self) -> bool {
        self.numbered_predicate(13) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for BMI1.
    /// BMI1: CPUID.(EAX=07H, ECX=0H):EBX.BMI1[bit 3]
    pub fn has_bmi1(&self) -> bool {
        self.numbered_predicate(14) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for BMI2.
    /// BMI2: CPUID.(EAX=07H, ECX=0H):EBX.BMI2[bit 8]
    pub fn has_bmi2(&self) -> bool {
        self.numbered_predicate(15) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Has support for LZCNT.
    /// LZCNT: CPUID.EAX=80000001H:ECX.LZCNT[bit 5]
    pub fn has_lzcnt(&self) -> bool {
        self.numbered_predicate(16) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:168
    }
    /// Computed predicate `x86.has_avx()`.
    pub fn use_avx(&self) -> bool {
        self.numbered_predicate(17) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_avx() && x86.has_avx2()`.
    pub fn use_avx2(&self) -> bool {
        self.numbered_predicate(18) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_avx512bitalg()`.
    pub fn use_avx512bitalg(&self) -> bool {
        self.numbered_predicate(19) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_avx512dq()`.
    pub fn use_avx512dq(&self) -> bool {
        self.numbered_predicate(20) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_avx512f()`.
    pub fn use_avx512f(&self) -> bool {
        self.numbered_predicate(21) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_avx512vbmi()`.
    pub fn use_avx512vbmi(&self) -> bool {
        self.numbered_predicate(22) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_avx512vl()`.
    pub fn use_avx512vl(&self) -> bool {
        self.numbered_predicate(23) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_bmi1()`.
    pub fn use_bmi1(&self) -> bool {
        self.numbered_predicate(24) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_bmi2()`.
    pub fn use_bmi2(&self) -> bool {
        self.numbered_predicate(25) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_cmpxchg16b()`.
    pub fn use_cmpxchg16b(&self) -> bool {
        self.numbered_predicate(26) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_avx() && x86.has_fma()`.
    pub fn use_fma(&self) -> bool {
        self.numbered_predicate(27) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_lzcnt()`.
    pub fn use_lzcnt(&self) -> bool {
        self.numbered_predicate(28) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_popcnt() && x86.has_sse42()`.
    pub fn use_popcnt(&self) -> bool {
        self.numbered_predicate(29) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_sse41()`.
    pub fn use_sse41(&self) -> bool {
        self.numbered_predicate(30) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_sse41() && x86.has_sse42()`.
    pub fn use_sse42(&self) -> bool {
        self.numbered_predicate(31) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
    /// Computed predicate `x86.has_ssse3()`.
    pub fn use_ssse3(&self) -> bool {
        self.numbered_predicate(32) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:198
    }
}
static DESCRIPTORS: [detail::Descriptor; 84] = [ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:262
    detail::Descriptor {
        name: "has_sse3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for SSE3.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 0 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_ssse3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for SSSE3.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 1 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_cmpxchg16b", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for CMPXCHG16b.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 2 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_sse41", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for SSE4.1.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 3 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_sse42", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for SSE4.2.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 4 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_avx", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for AVX.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 5 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_avx2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for AVX2.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 6 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_fma", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for FMA.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 7 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_avx512bitalg", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for AVX512BITALG.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 0 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_avx512dq", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for AVX512DQ.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 1 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_avx512vl", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for AVX512VL.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 2 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_avx512vbmi", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for AVX512VMBI.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 3 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_avx512f", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for AVX512F.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 4 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_popcnt", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for POPCNT.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 5 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_bmi1", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for BMI1.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 6 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_bmi2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for BMI2.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 7 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "has_lzcnt", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:270
        description: "Has support for LZCNT.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:271
        offset: 2, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:272
        detail: detail::Detail::Bool { bit: 0 }, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:275
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:297
    detail::Descriptor {
        name: "sse3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "SSE3 and earlier.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "ssse3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "SSSE3 and earlier.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 3, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "sse41", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "SSE4.1 and earlier.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 6, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "sse42", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "SSE4.2 and earlier.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "baseline", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "A baseline preset with no extensions enabled.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 12, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "nocona", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Nocona microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 15, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "core2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Core 2 microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 18, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "penryn", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Penryn microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 21, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "atom", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Atom microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 24, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "bonnell", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Bonnell microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 27, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "silvermont", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Silvermont microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 30, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "slm", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Silvermont microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 33, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "goldmont", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Goldmont microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 36, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "goldmont-plus", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Goldmont Plus microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 39, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "tremont", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Tremont microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 42, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "alderlake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Alderlake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 45, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "sierraforest", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Sierra Forest microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 48, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "grandridge", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Grandridge microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 51, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "nehalem", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Nehalem microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 54, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "corei7", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Core i7 microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 57, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "westmere", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Westmere microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 60, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "sandybridge", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Sandy Bridge microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 63, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "corei7-avx", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Core i7 AVX microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 66, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "ivybridge", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Ivy Bridge microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 69, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "core-avx-i", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Intel Core CPU with 64-bit extensions.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 72, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "haswell", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Haswell microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 75, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "core-avx2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Intel Core CPU with AVX2 extensions.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 78, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "broadwell", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Broadwell microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 81, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "skylake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Skylake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 84, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "knl", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Knights Landing microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 87, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "knm", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Knights Mill microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 90, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "skylake-avx512", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Skylake AVX512 microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 93, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "skx", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Skylake AVX512 microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 96, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "cascadelake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Cascade Lake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 99, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "cooperlake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Cooper Lake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 102, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "cannonlake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Canon Lake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 105, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "icelake-client", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Ice Lake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 108, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "icelake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Ice Lake microarchitecture", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 111, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "icelake-server", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Ice Lake (server) microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 114, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "tigerlake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Tiger Lake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 117, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "sapphirerapids", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Sapphire Rapids microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 120, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "raptorlake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Raptor Lake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 123, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "meteorlake", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Meteor Lake microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 126, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "graniterapids", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Granite Rapids microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 129, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "opteron", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Opteron microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 132, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "k8", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "K8 Hammer microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 135, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "athlon64", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Athlon64 microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 138, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "athlon-fx", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Athlon FX microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 141, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "opteron-sse3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Opteron microarchitecture with support for SSE3 instructions.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 144, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "k8-sse3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "K8 Hammer microarchitecture with support for SSE3 instructions.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 147, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "athlon64-sse3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Athlon 64 microarchitecture with support for SSE3 instructions.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 150, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "barcelona", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Barcelona microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 153, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "amdfam10", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "AMD Family 10h microarchitecture", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 156, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "btver1", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Bobcat microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 159, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "btver2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Jaguar microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 162, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "bdver1", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Bulldozer microarchitecture", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 165, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "bdver2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Piledriver microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 168, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "bdver3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Steamroller microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 171, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "bdver4", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Excavator microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 174, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "znver1", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Zen (first generation) microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 177, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "znver2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Zen (second generation) microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 180, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "znver3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Zen (third generation) microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 183, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "znver4", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Zen (fourth generation) microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 186, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "x86-64", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Generic x86-64 microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 189, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "x86-64-v2", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Generic x86-64 (V2) microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 192, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "x84_64_v3", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Generic x86_64 (V3) microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 195, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
    detail::Descriptor {
        name: "x86_64_v4", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:302
        description: "Generic x86_64 (V4) microarchitecture.", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:303
        offset: 198, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:304
        detail: detail::Detail::Preset, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:305
    }
    , // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:307
]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:313
static ENUMERATORS: [&str; 0] = [ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:316
]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:322
static HASH_TABLE: [u16; 128] = [ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:332
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    78, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    77, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    76, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    24, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    79, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    67, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    81, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    23, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    51, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    60, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    15, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    14, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    30, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    1, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    42, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    71, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    68, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    5, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    36, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    66, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    6, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    45, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    22, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    65, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    16, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    7, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    48, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    50, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    25, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    63, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    12, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    44, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    39, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    53, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    70, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    4, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    32, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    3, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    59, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    11, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    13, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    31, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    80, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    74, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    40, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    29, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    47, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    46, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    9, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    55, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    72, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    10, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    75, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    73, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    2, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    62, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    82, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    34, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    8, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    19, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    20, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    49, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    17, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    54, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    61, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    21, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    64, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    69, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    57, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    83, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    27, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    28, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    35, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    37, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    41, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    43, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    33, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    58, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    52, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    18, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    56, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    0xffff, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:344
    26, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
    38, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:336
]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:348
static PRESETS: [(u8, u8); 201] = [ // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:351
    // sse3: has_sse3
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // ssse3: has_sse3, has_ssse3
    (0b00000011, 0b00000011), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // sse41: has_sse3, has_ssse3, has_sse41
    (0b00001011, 0b00001011), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // sse42: has_sse3, has_ssse3, has_sse41, has_sse42
    (0b00011011, 0b00011011), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // baseline: 
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // nocona: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // core2: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // penryn: has_sse3, has_ssse3, has_sse41, has_cmpxchg16b
    (0b00001111, 0b00001111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // atom: has_sse3, has_ssse3, has_cmpxchg16b
    (0b00000111, 0b00000111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // bonnell: has_sse3, has_ssse3, has_cmpxchg16b
    (0b00000111, 0b00000111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // silvermont: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // slm: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // goldmont: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // goldmont-plus: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // tremont: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // alderlake: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // sierraforest: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // grandridge: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // nehalem: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // corei7: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // westmere: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // sandybridge: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx
    (0b00111111, 0b00111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // corei7-avx: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx
    (0b00111111, 0b00111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // ivybridge: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx
    (0b00111111, 0b00111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // core-avx-i: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx
    (0b00111111, 0b00111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // haswell: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // core-avx2: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // broadwell: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // skylake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // knl: has_popcnt, has_avx512f, has_fma, has_bmi1, has_bmi2, has_lzcnt, has_cmpxchg16b
    (0b10000100, 0b10000100), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11110000, 0b11110000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // knm: has_popcnt, has_avx512f, has_fma, has_bmi1, has_bmi2, has_lzcnt, has_cmpxchg16b
    (0b10000100, 0b10000100), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11110000, 0b11110000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // skylake-avx512: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11110110, 0b11110110), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // skx: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11110110, 0b11110110), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // cascadelake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11110110, 0b11110110), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // cooperlake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11110110, 0b11110110), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // cannonlake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11111110, 0b11111110), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // icelake-client: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // icelake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // icelake-server: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // tigerlake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // sapphirerapids: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // raptorlake: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // meteorlake: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // graniterapids: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // opteron: 
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // k8: 
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // athlon64: 
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // athlon-fx: 
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // opteron-sse3: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // k8-sse3: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // athlon64-sse3: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // barcelona: has_popcnt, has_lzcnt, has_cmpxchg16b
    (0b00000100, 0b00000100), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // amdfam10: has_popcnt, has_lzcnt, has_cmpxchg16b
    (0b00000100, 0b00000100), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // btver1: has_sse3, has_ssse3, has_lzcnt, has_popcnt, has_cmpxchg16b
    (0b00000111, 0b00000111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // btver2: has_sse3, has_ssse3, has_lzcnt, has_popcnt, has_cmpxchg16b, has_avx, has_bmi1
    (0b00100111, 0b00100111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b01100000, 0b01100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // bdver1: has_lzcnt, has_popcnt, has_sse3, has_ssse3, has_cmpxchg16b
    (0b00000111, 0b00000111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // bdver2: has_lzcnt, has_popcnt, has_sse3, has_ssse3, has_cmpxchg16b, has_bmi1
    (0b00000111, 0b00000111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b01100000, 0b01100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // bdver3: has_lzcnt, has_popcnt, has_sse3, has_ssse3, has_cmpxchg16b, has_bmi1
    (0b00000111, 0b00000111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b01100000, 0b01100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // bdver4: has_lzcnt, has_popcnt, has_sse3, has_ssse3, has_cmpxchg16b, has_bmi1, has_avx2, has_bmi2
    (0b01000111, 0b01000111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // znver1: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_cmpxchg16b
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // znver2: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_cmpxchg16b
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // znver3: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_cmpxchg16b
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // znver4: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_cmpxchg16b, has_avx512bitalg, has_avx512dq, has_avx512f, has_avx512vbmi, has_avx512vl
    (0b10011111, 0b10011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11111111, 0b11111111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // x86-64: 
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // x86-64-v2: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b
    (0b00011111, 0b00011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00100000, 0b00100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000000, 0b00000000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // x84_64_v3: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx2
    (0b11011111, 0b11011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100000, 0b11100000), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    // x86_64_v4: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx2, has_avx512dq, has_avx512vl
    (0b11011111, 0b11011111), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b11100110, 0b11100110), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
    (0b00000001, 0b00000001), // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:364
]; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:368
static TEMPLATE: detail::Template = detail::Template {
    name: "x86", // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:383
    descriptors: &DESCRIPTORS, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:384
    enumerators: &ENUMERATORS, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:385
    hash_table: &HASH_TABLE, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:386
    defaults: &[0x00, 0x00, 0x00], // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:387
    presets: &PRESETS, // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:388
}
; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:391
/// Create a `settings::Builder` for the x86 settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE) // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:398
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[x86]")?; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_settings.rs:407
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
