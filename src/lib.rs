
#[cfg(test)]
mod tests {
    #[test]
    fn test_globset() {
        extern crate globset;

        let glob = globset::Glob::new("*.rs").unwrap().compile_matcher();
        assert!(glob.is_match("foo.rs"));
        assert!(glob.is_match("foo/bar.rs"));
        assert!(glob.is_match("foo/foo2/bar.rs"));
        assert!(glob.is_match("foo/foo2/foo3/bar.rs"));

        let glob = globset::Glob::new("/foo").unwrap().compile_matcher();
        assert!(!glob.is_match("foo.rs"));
        assert!(!glob.is_match("foo/bar.rs")); // XXX
        assert!(!glob.is_match("foo/foo2/bar.rs")); // XXX
        assert!(!glob.is_match("foo/foo2/foo3/bar.rs")); // XXX

        let glob = globset::Glob::new("foo").unwrap().compile_matcher();
        assert!(!glob.is_match("foo.rs"));
        assert!(!glob.is_match("foo/bar.rs")); // XXX
        assert!(!glob.is_match("foo/foo2/bar.rs")); // XXX
        assert!(!glob.is_match("foo/foo2/foo3/bar.rs")); // XXX

        let glob = globset::Glob::new("foo/").unwrap().compile_matcher();
        assert!(!glob.is_match("foo.rs"));
        assert!(!glob.is_match("foo/bar.rs")); // XXX
        assert!(!glob.is_match("foo/foo2/bar.rs")); // XXX
        assert!(!glob.is_match("foo/foo2/foo3/bar.rs")); // XXX

        let glob = globset::Glob::new("foo/*").unwrap().compile_matcher();
        assert!(!glob.is_match("foo.rs"));
        assert!(glob.is_match("foo/bar.rs"));
        assert!(glob.is_match("foo/foo2/bar.rs"));
        assert!(glob.is_match("foo/foo2/foo3/bar.rs"));

        let glob = globset::Glob::new("foo/**").unwrap().compile_matcher();
        assert!(!glob.is_match("foo.rs"));
        assert!(glob.is_match("foo/bar.rs"));
        assert!(glob.is_match("foo/foo2/bar.rs"));
        assert!(glob.is_match("foo/foo2/foo3/bar.rs"));
    }

    #[test]
    fn test_glob() {
        extern crate glob;

        let glob = glob::Pattern::new("*.rs").unwrap();
        assert!(glob.matches("foo.rs"));
        assert!(glob.matches("foo/bar.rs"));
        assert!(glob.matches("foo/foo2/bar.rs"));
        assert!(glob.matches("foo/foo2/foo3/bar.rs"));

        let glob = glob::Pattern::new("/foo").unwrap();
        assert!(!glob.matches("foo.rs"));
        assert!(!glob.matches("foo/bar.rs")); // XXX
        assert!(!glob.matches("foo/foo2/bar.rs")); // XXX
        assert!(!glob.matches("foo/foo2/foo3/bar.rs")); // XXX

        let glob = glob::Pattern::new("foo").unwrap();
        assert!(!glob.matches("foo.rs"));
        assert!(!glob.matches("foo/bar.rs")); // XXX
        assert!(!glob.matches("foo/foo2/bar.rs")); // XXX
        assert!(!glob.matches("foo/foo2/foo3/bar.rs")); // XXX

        let glob = glob::Pattern::new("foo/").unwrap();
        assert!(!glob.matches("foo.rs"));
        assert!(!glob.matches("foo/bar.rs")); // XXX
        assert!(!glob.matches("foo/foo2/bar.rs")); // XXX
        assert!(!glob.matches("foo/foo2/foo3/bar.rs")); // XXX

        let glob = glob::Pattern::new("foo/*").unwrap();
        assert!(!glob.matches("foo.rs"));
        assert!(glob.matches("foo/bar.rs"));
        assert!(glob.matches("foo/foo2/bar.rs"));
        assert!(glob.matches("foo/foo2/foo3/bar.rs"));

        let glob = glob::Pattern::new("foo/**").unwrap();
        assert!(!glob.matches("foo.rs"));
        assert!(glob.matches("foo/bar.rs"));
        assert!(glob.matches("foo/foo2/bar.rs"));
        assert!(glob.matches("foo/foo2/foo3/bar.rs"));
    }
}
