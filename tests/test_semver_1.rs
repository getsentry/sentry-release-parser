#![cfg(feature = "semver-1")]

use sentry_release_parser::Version;
use similar_asserts::assert_eq;

#[test]
fn test_basic() {
    let v1 = Version::parse("1.2.3-dev+BUILD-code").unwrap();
    let semver1 = v1.as_semver1();
    assert_eq!(semver1.major, 1);
    assert_eq!(semver1.minor, 2);
    assert_eq!(semver1.patch, 3);
    assert_eq!(semver1.pre.as_str(), "dev");
    assert_eq!(semver1.build.as_str(), "BUILD-code");

    let v2 = Version::parse("1.2.3").unwrap();
    let semver2 = v2.as_semver1();
    assert_eq!(semver2.major, 1);
    assert_eq!(semver2.minor, 2);
    assert_eq!(semver2.patch, 3);
    assert_eq!(semver2.pre, semver_1::Prerelease::EMPTY);
    assert_eq!(semver2.build, semver_1::BuildMetadata::EMPTY);
}
