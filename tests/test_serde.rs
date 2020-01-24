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
