#![cfg(feature = "serde")]
use sentry_release_parser::Release;

#[test]
fn test_basic() {
    let release = Release::parse("@foo.bar.baz--blah@1.2.3-dev+BUILD-code").unwrap();
    insta::assert_yaml_snapshot!(&release, @r###"
    ---
    raw: "@foo.bar.baz--blah@1.2.3-dev+BUILD-code"
    package: "@foo.bar.baz--blah"
    version_raw: 1.2.3-dev+BUILD-code
    version:
      raw: 1.2.3-dev+BUILD-code
      major: 1
      minor: 2
      patch: 3
      pre: dev
      build_code: BUILD-code
    format_type: versioned
    "###);
}
