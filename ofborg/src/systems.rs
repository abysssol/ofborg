#[derive(Clone, Debug)]
pub enum System {
    X8664Linux,
    Aarch64Linux,
    X8664Darwin,
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            System::X8664Linux => write!(f, "x86_64-linux"),
            System::Aarch64Linux => write!(f, "aarch64-linux"),
            System::X8664Darwin => write!(f, "x86_64-darwin"),
        }
    }
}

impl System {
    pub fn as_build_destination(&self) -> (Option<String>, Option<String>) {
        (None, Some(format!("build-inputs-{}", self.to_string())))
    }

    pub fn can_run_nixos_tests(&self) -> bool {
        match self {
            System::X8664Linux => true,
            System::Aarch64Linux => true,
            System::X8664Darwin => false,
        }
    }
}
