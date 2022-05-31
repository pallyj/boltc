#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BuildProfile {
    Debug,
    Less,
    Normal,
    Aggressive
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BuildOutput {
    LLVM,
    ASM,
    Object,
}

#[derive(Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct BuildConfig {
    pub(crate) profile: BuildProfile,
    pub(crate) output:  BuildOutput,
    pub(crate) target:  Option<String>,
}

impl BuildConfig {
    pub fn new(profile: BuildProfile, output: BuildOutput, target: Option<String>) -> BuildConfig { BuildConfig { profile, output, target } }
}
