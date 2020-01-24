#![cfg(feature = "serde")]
use sentry_release_parser::Release;

#[test]
fn test_basic() {
    let release = Release::parse("@foo.bar.baz--blah@1.2.3-dev+BUILD-code").unwrap();
    insta::assert_yaml_snapshot!(&release, @r###"
    ---
    package: "@foo.bar.baz--blah"
    version_raw: 1.2.3-dev+BUILD-code
    version_parsed:
      major: 1
      minor: 2
      patch: 3
      pre: dev
      build_code: BUILD-code
    format: versioned
    "###);
}

#[test]
fn test_hash() {
    let release = Release::parse("085240e737828d8326719bf97730188e927e49ca").unwrap();
    insta::assert_yaml_snapshot!(&release, @r###"
    ---
    package: ~
    version_raw: 085240e737828d8326719bf97730188e927e49ca
    version_parsed: ~
    format: unqualified
    "###);
}

#[test]
fn test_qualified_hash() {
    let release = Release::parse("package@085240e737828d8326719bf97730188e927e49ca").unwrap();
    insta::assert_yaml_snapshot!(&release, @r###"
    ---
    package: package
    version_raw: 085240e737828d8326719bf97730188e927e49ca
    version_parsed:
      major: 0
      minor: 0
      patch: 0
      pre: 85240e737828d8326719bf97730188e927e49ca
      build_code: ~
    format: versioned
    "###);
}
