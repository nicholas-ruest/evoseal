use evoseal_application::{IntegrationError, RrsiResult, RrsiSelector};
use evoseal_domain::PromotionRequest;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

pub struct PythonRrsiSelector {
    python: PathBuf,
    source_root: PathBuf,
    bridge: PathBuf,
    timeout: Duration,
}

impl PythonRrsiSelector {
    pub fn new(
        python: impl Into<PathBuf>,
        source_root: impl Into<PathBuf>,
        bridge: impl Into<PathBuf>,
        timeout: Duration,
    ) -> Self {
        Self {
            python: python.into(),
            source_root: source_root.into(),
            bridge: bridge.into(),
            timeout,
        }
    }

    fn verify_source(&self) -> Result<(), IntegrationError> {
        let expected = self.source_root.join("rrsi/selection.py");
        if !expected.is_file() {
            return Err(IntegrationError::Upstream(format!(
                "RRSI source missing: {}",
                expected.display()
            )));
        }
        if !Path::new(&self.bridge).is_file() {
            return Err(IntegrationError::Upstream(format!(
                "bridge missing: {}",
                self.bridge.display()
            )));
        }
        Ok(())
    }
}

impl RrsiSelector for PythonRrsiSelector {
    fn select(&self, request: &PromotionRequest) -> Result<RrsiResult, IntegrationError> {
        self.verify_source()?;
        let mut child = Command::new(&self.python)
            .arg(&self.bridge)
            .env("PYTHONPATH", &self.source_root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| IntegrationError::Upstream(error.to_string()))?;
        let payload = serde_json::to_vec(request)
            .map_err(|error| IntegrationError::Upstream(error.to_string()))?;
        child
            .stdin
            .take()
            .ok_or_else(|| IntegrationError::Upstream("stdin unavailable".into()))?
            .write_all(&payload)
            .map_err(|error| IntegrationError::Upstream(error.to_string()))?;
        let status = child
            .wait_timeout(self.timeout)
            .map_err(|error| IntegrationError::Upstream(error.to_string()))?;
        let Some(status) = status else {
            child
                .kill()
                .map_err(|error| IntegrationError::Upstream(error.to_string()))?;
            let _ = child.wait();
            return Err(IntegrationError::Timeout);
        };
        let mut stdout = String::new();
        child
            .stdout
            .take()
            .ok_or_else(|| IntegrationError::Upstream("stdout unavailable".into()))?
            .read_to_string(&mut stdout)
            .map_err(|error| IntegrationError::Upstream(error.to_string()))?;
        if !status.success() {
            let mut stderr = String::new();
            if let Some(mut pipe) = child.stderr.take() {
                let _ = pipe.read_to_string(&mut stderr);
            }
            return Err(IntegrationError::Upstream(stderr));
        }
        serde_json::from_str(&stdout).map_err(|error| IntegrationError::Upstream(error.to_string()))
    }
}
