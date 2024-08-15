use super::SetOpt;
use curl::easy::{Easy2, SslVersion};

#[derive(Clone, Debug)]
pub(crate) struct TLSVersions {
    min_version: SslVersion,
    max_version: SslVersion,
}

impl TLSVersions {
    pub fn new(min_version: SslVersion, max_version: SslVersion,) -> Self {
        Self {
            min_version: min_version,
            max_version: max_version
        }
    }
}

impl SetOpt for TLSVersions {
    fn set_opt<H>(&self, easy: &mut Easy2<H>) -> Result<(), curl::Error> {
        easy.ssl_min_max_version(self.min_version, self.max_version)?;
    }
}
