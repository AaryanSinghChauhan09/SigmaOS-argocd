//! argocd model — GitOps continuous delivery for Kubernetes.

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub namespace: String,
    pub repo: String,
    pub path: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub apps: Vec<Application>,
}

impl Default for Model {
    fn default() -> Self { Self { apps: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn sync(&mut self, name: &str) -> bool {
        if let Some(a) = self.apps.iter_mut().find(|a| a.name == name) { a.status = "Synced".into(); true } else { false }
    }

    pub fn add(&mut self, app: Application) { self.apps.push(app); }
    pub fn list(&self) -> &[Application] { &self.apps }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sync() {
        let mut a = Model::new();
        a.add(Application { name: "web".into(), namespace: "default".into(), repo: "https://github.com/org/repo".into(), path: "manifests".into(), status: "OutOfSync".into() });
        assert!(a.sync("web"));
        assert_eq!(a.list()[0].status, "Synced");
    }
}
