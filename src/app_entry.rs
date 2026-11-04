#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub categories: Vec<String>,
}
impl AppEntry {
    pub fn new(name: &str, exec: &str) -> Self {
        Self { name: name.to_string(), exec: exec.to_string(), icon: String::new(), categories: Vec::new() }
    }
    pub fn matches(&self, query: &str) -> bool {
        let q = query.to_lowercase();
        self.name.to_lowercase().contains(&q) || self.exec.to_lowercase().contains(&q)
    }
}
pub struct AppCatalog { entries: Vec<AppEntry> }
impl AppCatalog {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn add(&mut self, entry: AppEntry) { self.entries.push(entry); }
    pub fn search(&self, query: &str) -> Vec<&AppEntry> {
        if query.is_empty() { return self.entries.iter().collect(); }
        self.entries.iter().filter(|e| e.matches(query)).collect()
    }
    pub fn count(&self) -> usize { self.entries.len() }
}
impl Default for AppCatalog { fn default() -> Self { Self::new() } }
