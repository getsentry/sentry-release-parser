use sentry_release_parser::Release;

#[test]
fn test_basic() {
    let release = Release::new("@foo.bar.baz/blah@1.2.3-dev");
    assert_eq!(release.package(), "@foo.bar.baz/blah");
    assert_eq!(release.version_raw(), "1.2.3-dev");
}