use sigmaos_launcher::{AppCatalog, AppEntry, Config, LauncherComponent, Component};

#[test]
fn test_catalog_add_and_count_aaa() {
    let mut cat = AppCatalog::new();
    cat.add(AppEntry::new("Terminal", "gnome-terminal"));
    cat.add(AppEntry::new("Firefox", "firefox"));
    assert_eq!(cat.count(), 2);
}

#[test]
fn test_catalog_search_aaa() {
    let mut cat = AppCatalog::new();
    cat.add(AppEntry::new("Calculator", "gnome-calculator"));
    cat.add(AppEntry::new("Text Editor", "gedit"));
    let results = cat.search("calc");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Calculator");
}

#[test]
fn test_catalog_search_empty_returns_all_aaa() {
    let mut cat = AppCatalog::new();
    cat.add(AppEntry::new("App1", "exec1"));
    cat.add(AppEntry::new("App2", "exec2"));
    assert_eq!(cat.search("").len(), 2);
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = LauncherComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = LauncherComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-launcher");
}
