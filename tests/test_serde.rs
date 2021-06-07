#![cfg(feature = "serde")]
use sentry_release_parser::Release;

macro_rules! assert_release_snapshot {
    ($release:expr) => {
        fn release(s: &str) -> Release<'_> {
            Release::parse(s).unwrap()
        }
        insta::assert_json_snapshot!(&release($release));
    };
}

#[test]
fn test_basic() {
    assert_release_snapshot!("@foo.bar.baz--blah@1.2.3-dev+BUILD-code");
}

#[test]
fn test_mobile() {
    assert_release_snapshot!("foo.bar.baz.App@1.0+20200101100");
}

#[test]
fn test_mobile_three_components() {
    assert_release_snapshot!("foo.bar.baz.App@1.0.0+20200101100");
}

#[test]
fn test_mobile_dotted_secondary() {
    assert_release_snapshot!("foo.bar.baz.App@1.0+1.0.200");
}

#[test]
fn test_hash() {
    assert_release_snapshot!("085240e737828d8326719bf97730188e927e49ca");
}

#[test]
fn test_qualified_hash() {
    assert_release_snapshot!("package@085240e737828d8326719bf97730188e927e49ca");
}

#[test]
fn test_single_component() {
    assert_release_snapshot!("com.foogame.FooGame@7211+7211");
}

#[test]
fn test_invalid_date_release() {
    assert_release_snapshot!("some-api@2020.05.26-01.38.42");
}

#[test]
fn test_valid_dotted_release() {
    assert_release_snapshot!("some-api@2020.2-1.2.3");
}

#[test]
fn test_basic_prerelease() {
    assert_release_snapshot!("some-api@1.2.3-test");
}

#[test]
fn test_implied_prerelease() {
    assert_release_snapshot!("some-api@1.0alpha2");
}

#[test]
fn test_four_components() {
    assert_release_snapshot!("some-api@1.0.0.0");
}

#[test]
fn test_dashed_numeric_prerelease() {
    assert_release_snapshot!("some-api@1.0-1234");
}

#[test]
fn test_leading_zeroes() {
    assert_release_snapshot!("foo@01.02.003.4-alpha+1234");
}

#[test]
fn test_version_only() {
    assert_release_snapshot!("foo@20210505090610352561");
}

#[test]
fn test_empty_version() {
    assert_release_snapshot!("foo@");
}
