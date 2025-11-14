#![cfg(any(feature = "semver", feature = "semver-1"))]

#[cfg(feature = "semver")]
use sentry_release_parser::Release;

#[cfg(feature = "semver-1")]
use {
    sentry_release_parser::Version,
    std::cmp::Ordering,
};

use similar_asserts::assert_eq;

#[cfg(feature = "semver")]
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

#[cfg(feature = "semver-1")]
#[test]
fn test_basic_semver_1() {
    let v1 = Version::parse("1.2.3-dev+BUILD-code").unwrap();
    let semver1 = v1.as_semver();
    assert_eq!(semver1.major, 1);
    assert_eq!(semver1.minor, 2);
    assert_eq!(semver1.patch, 3);
    assert_eq!(semver1.pre.as_str(), "dev");
    assert_eq!(semver1.build.as_str(), "BUILD-code");

    let v2 = Version::parse("1.2.3").unwrap();
    let semver2 = v2.as_semver();
    assert_eq!(semver2.major, 1);
    assert_eq!(semver2.minor, 2);
    assert_eq!(semver2.patch, 3);
    assert_eq!(semver2.pre, semver_1::Prerelease::EMPTY);
    assert_eq!(semver2.build, semver_1::BuildMetadata::EMPTY);
}

#[cfg(feature = "semver-1")]
#[test]
fn test_cmp_precedence() {
    let v1 = Version::parse("1.0.0").unwrap();
    let v2 = Version::parse("2.0.0").unwrap();
    assert_eq!(v1.cmp_precedence(&v2), Ordering::Less);

    let v3 = Version::parse("1.0.0-alpha").unwrap();
    let v4 = Version::parse("1.0.0").unwrap();
    assert_eq!(v3.cmp_precedence(&v4), Ordering::Less);

    let v5 = Version::parse("1.0.0+build1").unwrap();
    let v6 = Version::parse("1.0.0+build2").unwrap();
    assert_eq!(v5.cmp_precedence(&v6), Ordering::Equal);

    let v9 = Version::parse("1.0.0-alpha+build2").unwrap();
    let v10 = Version::parse("1.0.0-beta+build1").unwrap();
    assert_eq!(v9.cmp_precedence(&v10), Ordering::Less);
}