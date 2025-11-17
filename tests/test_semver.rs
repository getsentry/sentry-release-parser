#![cfg(feature = "semver")]

use sentry_release_parser::Release;
use similar_asserts::assert_eq;

#[test]
fn test_basic() {
    let release = Release::parse("@foo.bar.baz--blah@1.2.3-dev+BUILD-code").unwrap();
    assert_eq!(release.package(), Some("@foo.bar.baz--blah"));
    assert_eq!(release.version_raw(), "1.2.3-dev+BUILD-code");

    let version = release.version().unwrap();
    let semver = version.as_semver();

    assert_eq!(
        semver,
        semver::Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre: vec![semver::Identifier::AlphaNumeric("dev".into())],
            build: vec![semver::Identifier::AlphaNumeric("BUILD-code".into())],
        }
    )
}
