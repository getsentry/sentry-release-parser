use sentry_release_parser::Release;

#[test]
fn test_basic() {
    let release = Release::new("@foo.bar.baz/blah@1.2.3-dev+build-code");
    assert_eq!(release.package(), "@foo.bar.baz/blah");
    assert_eq!(release.version_raw(), "1.2.3-dev+build-code");

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 2);
    assert_eq!(version.patch(), 3);
    assert_eq!(version.pre(), Some("dev"));
    assert_eq!(version.build_code(), Some("build-code"));
}

#[test]
fn test_basic_short_ver() {
    let release = Release::new("@foo.bar.baz/blah@1a+build-code");
    assert_eq!(release.package(), "@foo.bar.baz/blah");
    assert_eq!(release.version_raw(), "1a+build-code");

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 0);
    assert_eq!(version.patch(), 0);
    assert_eq!(version.pre(), Some("a"));
    assert_eq!(version.build_code(), Some("build-code"));
}
