use ipis::{core::anyhow::Result, env::Infer};
use ipnis_common::onnxruntime::{GraphOptimizationLevel, LoggingLevel};

#[derive(Clone, Debug, PartialEq)]
pub struct ClientConfig {
    pub log_level: LoggingLevel,
    pub optimization_level: GraphOptimizationLevel,
    pub number_threads: u8,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            log_level: LoggingLevel::Warning,
            optimization_level: GraphOptimizationLevel::Basic,
            number_threads: 1,
        }
    }
}

impl<'a> Infer<'a> for ClientConfig {
    type GenesisArgs = ();
    type GenesisResult = Self;

    fn try_infer() -> Result<Self>
    where
        Self: Sized,
    {
        // TODO: collect from environment variables
        Ok(Self::default())
    }

    fn genesis((): <Self as Infer<'a>>::GenesisArgs) -> Result<<Self as Infer<'a>>::GenesisResult> {
        Ok(Self::default())
    }
}
