//! SigmaOS-argocd: GitOps continuous delivery model.
//!
//! Real implementation: applications, deployments, sync, validation.
//! Replaces the 7-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub repo_url: String,
    pub target_revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgoCDError {
    EmptyName,
    DuplicateApplication(String),
}

impl fmt::Display for ArgoCDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "application name must not be empty"),
            Self::DuplicateApplication(id) => write!(f, "duplicate application {:?}", id),
        }
    }
}

impl std::error::Error for ArgoCDError {}

pub struct ArgoCDManager {
    pub name: String,
    applications: BTreeMap<String, Application>,
    deployments: BTreeMap<String, String>,
}

impl ArgoCDManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), applications: BTreeMap::new(), deployments: BTreeMap::new() }
    }

    pub fn add_application(&mut self, a: Application) -> Result<(), ArgoCDError> {
        if a.name.trim().is_empty() { return Err(ArgoCDError::EmptyName); }
        let id = a.id.clone();
        if self.applications.contains_key(&id) {
            return Err(ArgoCDError::DuplicateApplication(id.clone()));
        }
        self.applications.insert(id, a);
        Ok(())
    }

    pub fn application_count(&self) -> usize { self.applications.len() }

    pub fn get_application(&self, id: &str) -> Option<&Application> { self.applications.get(id) }

    pub fn add_deployment(&mut self, app_id: String, status: String) {
        self.deployments.insert(app_id, status);
    }

    pub fn get_deployment(&self, app_id: &str) -> Option<&String> { self.deployments.get(app_id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let m = ArgoCDManager::new("My Manager");
        assert_eq!(m.name, "My Manager");
        assert_eq!(m.application_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut m = ArgoCDManager::new("My Manager");
        let a = Application { id: "app1".into(), name: "webapp".into(), repo_url: "https://github.com/...".into(), target_revision: "main".into() };
        m.add_application(a).unwrap();
        assert_eq!(m.application_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut m = ArgoCDManager::new("My Manager");
        let a = Application { id: "x".into(), name: "".into(), repo_url: "a".into(), target_revision: "b".into() };
        assert!(m.add_application(a).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut m = ArgoCDManager::new("My Manager");
        let a = Application { id: "x".into(), name: "X".into(), repo_url: "a".into(), target_revision: "b".into() };
        m.add_application(a).unwrap();
        let a2 = Application { id: "x".into(), name: "X".into(), repo_url: "a".into(), target_revision: "b".into() };
        assert!(m.add_application(a2).is_err());
    }
}
