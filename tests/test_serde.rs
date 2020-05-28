#![cfg(feature = "serde")]
use sentry_release_parser::Release;

macro_rules! release_snapshot {
    ($release:expr) => {
        fn release(s: &str) -> Release<'_> {
            Release::parse(s).unwrap()
        }
        insta::assert_json_snapshot!(&release($release));
    };
}

#[test]
fn test_basic() {
    release_snapshot!("@foo.bar.baz--blah@1.2.3-dev+BUILD-code");
}

#[test]
fn test_mobile() {
    release_snapshot!("foo.bar.baz.App@1.0+20200101100");
}

#[test]
fn test_mobile_three_components() {
    release_snapshot!("foo.bar.baz.App@1.0.0+20200101100");
}

#[test]
fn test_mobile_dotted_secondary() {
    release_snapshot!("foo.bar.baz.App@1.0+1.0.200");
}

#[test]
fn test_hash() {
    release_snapshot!("085240e737828d8326719bf97730188e927e49ca");
}

#[test]
fn test_qualified_hash() {
    release_snapshot!("package@085240e737828d8326719bf97730188e927e49ca");
}

#[test]
fn test_single_component() {
    release_snapshot!("com.foogame.FooGame@7211+7211");
}

#[test]
fn test_invalid_date_release() {
    release_snapshot!("some-api@2020.05.26-01.38.42");
}

#[test]
fn test_valid_dotted_release() {
    release_snapshot!("some-api@2020.2-1.2.3");
}

#[test]
fn test_basic_prerelease() {
    release_snapshot!("some-api@1.2.3-test");
}

#[test]
fn test_implied_prerelease() {
    release_snapshot!("some-api@1.0alpha2");
}

#[test]
fn test_dashed_numeric_prerelease() {
    release_snapshot!("some-api@1.0-1234");
}
