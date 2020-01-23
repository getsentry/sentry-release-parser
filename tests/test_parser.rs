use sentry_release_parser::Release;

#[test]
fn test_basic() {
    let release = Release::parse("@foo.bar.baz/blah@1.2.3-dev+BUILD-code");
    assert_eq!(release.package(), Some("@foo.bar.baz/blah"));
    assert_eq!(release.version_raw(), "1.2.3-dev+BUILD-code");

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 2);
    assert_eq!(version.patch(), 3);
    assert_eq!(version.triple(), (1, 2, 3));
    assert_eq!(version.pre(), Some("dev"));
    assert_eq!(version.build_code(), Some("BUILD-code"));
    assert_eq!(version.normalized_build_code(), "build-code");

    assert_eq!(release.build_hash(), None);
    assert_eq!(
        release.to_string(),
        "@foo.bar.baz/blah@1.2.3-dev+BUILD-code"
    );
    assert_eq!(release.describe().to_string(), "1.2.3-dev (BUILD-code)");
}

#[test]
fn test_basic_short_ver() {
    let release = Release::parse("@foo.bar.baz/blah@1a+build-code");
    assert_eq!(release.package(), Some("@foo.bar.baz/blah"));
    assert_eq!(release.version_raw(), "1a+build-code");

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 0);
    assert_eq!(version.patch(), 0);
    assert_eq!(version.triple(), (1, 0, 0));
    assert_eq!(version.pre(), Some("a"));
    assert_eq!(version.build_code(), Some("build-code"));

    assert_eq!(release.build_hash(), None);
    assert_eq!(release.to_string(), "@foo.bar.baz/blah@1.0.0-a+build-code");

    assert_eq!(release.describe().to_string(), "1.0.0-a (build-code)");
}

#[test]
fn test_release_is_hash() {
    let release = Release::parse("a86d127c4b2f23a0a862620280427dcc01c78676");
    assert_eq!(release.package(), None);
    assert_eq!(release.version(), None);
    assert_eq!(
        release.version_raw(),
        "a86d127c4b2f23a0a862620280427dcc01c78676"
    );
    assert_eq!(
        release.build_hash(),
        Some("a86d127c4b2f23a0a862620280427dcc01c78676")
    );
    assert_eq!(
        release.to_string(),
        "a86d127c4b2f23a0a862620280427dcc01c78676"
    );
    assert_eq!(release.describe().to_string(), "a86d127c4b2f");

    let release = Release::parse("some-package@a86d127c4b2f23a0a862620280427dcc01c78676");
    assert_eq!(release.package(), Some("some-package"));
    assert_eq!(release.version(), None);
    assert_eq!(
        release.version_raw(),
        "a86d127c4b2f23a0a862620280427dcc01c78676"
    );
    assert_eq!(
        release.build_hash(),
        Some("a86d127c4b2f23a0a862620280427dcc01c78676")
    );
    assert_eq!(
        release.to_string(),
        "some-package@a86d127c4b2f23a0a862620280427dcc01c78676"
    );
    assert_eq!(release.describe().to_string(), "a86d127c4b2f");
}
