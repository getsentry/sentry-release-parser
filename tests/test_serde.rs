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
    build_hash: ~
    description: 1.2.3-dev (BUILD-code)
    "###);
}

#[test]
fn test_mobile() {
    let release = Release::parse("foo.bar.baz.App@1.0+20200101100").unwrap();
    insta::assert_yaml_snapshot!(&release, @r###"
    ---
    package: foo.bar.baz.App
    version_raw: 1.0+20200101100
    version_parsed:
      major: 1
      minor: 0
      patch: 0
      pre: ~
      build_code: "20200101100"
    build_hash: ~
    description: 1.0.0 (20200101100)
    "###);
}

#[test]
fn test_mobile_dotted_secondary() {
    let release = Release::parse("foo.bar.baz.App@1.0+1.0.200").unwrap();
    insta::assert_yaml_snapshot!(&release, @r###"
    ---
    package: foo.bar.baz.App
    version_raw: 1.0+1.0.200
    version_parsed:
      major: 1
      minor: 0
      patch: 0
      pre: ~
      build_code: 1.0.200
    build_hash: ~
    description: 1.0.0 (1.0.200)
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
    build_hash: 085240e737828d8326719bf97730188e927e49ca
    description: "085240e73782"
    "###);
}

#[test]
fn test_qualified_hash() {
    let release = Release::parse("package@085240e737828d8326719bf97730188e927e49ca").unwrap();
    insta::assert_yaml_snapshot!(&release, @r###"
    ---
    package: package
    version_raw: 085240e737828d8326719bf97730188e927e49ca
    version_parsed: ~
    build_hash: 085240e737828d8326719bf97730188e927e49ca
    description: "085240e73782"
    "###);
}
