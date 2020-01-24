use sentry_release_parser::{InvalidRelease, Release};

#[test]
fn test_basic() {
    let release = Release::parse("@foo.bar.baz--blah@1.2.3-dev+BUILD-code").unwrap();
    assert_eq!(release.package(), Some("@foo.bar.baz--blah"));
    assert_eq!(release.version_raw(), "1.2.3-dev+BUILD-code");

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 2);
    assert_eq!(version.patch(), 3);
    assert_eq!(version.triple(), (1, 2, 3));
    assert_eq!(version.pre(), Some("dev"));
    assert_eq!(version.build_code(), Some("BUILD-code"));
    assert_eq!(version.normalized_build_code(), "build-code");

    assert_eq!(release.build_hash(), None);
    assert_eq!(
        release.to_string(),
        "@foo.bar.baz--blah@1.2.3-dev+BUILD-code"
    );
    assert_eq!(release.describe().to_string(), "1.2.3-dev (BUILD-code)");
}

#[test]
fn test_basic_short_ver() {
    let release = Release::parse("@foo.bar.baz--blah@1a+build-code").unwrap();
    assert_eq!(release.package(), Some("@foo.bar.baz--blah"));
    assert_eq!(release.version_raw(), "1a+build-code");

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 0);
    assert_eq!(version.patch(), 0);
    assert_eq!(version.triple(), (1, 0, 0));
    assert_eq!(version.pre(), Some("a"));
    assert_eq!(version.build_code(), Some("build-code"));

    assert_eq!(release.build_hash(), None);
    assert_eq!(release.to_string(), "@foo.bar.baz--blah@1.0.0-a+build-code");

    assert_eq!(release.describe().to_string(), "1.0.0-a (build-code)");
}

#[test]
fn test_release_is_hash() {
    let release = Release::parse("a86d127c4b2f23a0a862620280427dcc01c78676").unwrap();
    assert_eq!(release.package(), None);
    assert_eq!(release.version(), None);
    assert_eq!(
        release.version_raw(),
        "a86d127c4b2f23a0a862620280427dcc01c78676"
    );
    assert_eq!(
        release.build_hash(),
        Some("a86d127c4b2f23a0a862620280427dcc01c78676")
    );
    assert_eq!(
        release.to_string(),
        "a86d127c4b2f23a0a862620280427dcc01c78676"
    );
    assert_eq!(release.describe().to_string(), "a86d127c4b2f");

    let release = Release::parse("some-package@a86d127c4b2f23a0a862620280427dcc01c78676").unwrap();
    assert_eq!(release.package(), Some("some-package"));
    assert_eq!(release.version(), None);
    assert_eq!(
        release.version_raw(),
        "a86d127c4b2f23a0a862620280427dcc01c78676"
    );
    assert_eq!(
        release.build_hash(),
        Some("a86d127c4b2f23a0a862620280427dcc01c78676")
    );
    assert_eq!(
        release.to_string(),
        "some-package@a86d127c4b2f23a0a862620280427dcc01c78676"
    );
    assert_eq!(release.describe().to_string(), "a86d127c4b2f");
}

#[test]
fn test_release_build_note_is_hash() {
    let release =
        Release::parse("@foo.bar.baz--blah@1a+a86d127c4b2f23a0a862620280427dcc01c78676").unwrap();
    assert_eq!(release.package(), Some("@foo.bar.baz--blah"));
    assert_eq!(
        release.version_raw(),
        "1a+a86d127c4b2f23a0a862620280427dcc01c78676"
    );

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 0);
    assert_eq!(version.patch(), 0);
    assert_eq!(version.triple(), (1, 0, 0));
    assert_eq!(version.pre(), Some("a"));
    assert_eq!(
        version.build_code(),
        Some("a86d127c4b2f23a0a862620280427dcc01c78676")
    );
    assert_eq!(
        release.build_hash(),
        Some("a86d127c4b2f23a0a862620280427dcc01c78676")
    );
    assert_eq!(
        release.to_string(),
        "@foo.bar.baz--blah@1.0.0-a+a86d127c4b2f23a0a862620280427dcc01c78676"
    );

    assert_eq!(release.describe().to_string(), "1.0.0-a (a86d127c4b2f)");
}

#[test]
fn test_release_build_note_is_num_starting_hash() {
    let release = Release::parse("package@085240e737828d8326719bf97730188e927e49ca").unwrap();
    assert_eq!(release.package(), Some("package"));
    assert_eq!(release.version_raw(), "085240e737828d8326719bf97730188e927e49ca");
    assert_eq!(release.version(), None);
    assert_eq!(
        release.build_hash(),
        Some("085240e737828d8326719bf97730188e927e49ca")
    );
}

#[test]
fn test_basic_ios_ver() {
    let release = Release::parse("org.example.FooApp@1.0rc1+20200101100").unwrap();
    assert_eq!(release.package(), Some("org.example.FooApp"));
    assert_eq!(release.version_raw(), "1.0rc1+20200101100");

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 0);
    assert_eq!(version.patch(), 0);
    assert_eq!(version.triple(), (1, 0, 0));
    assert_eq!(version.pre(), Some("rc1"));
    assert_eq!(version.build_code(), Some("20200101100"));
    assert_eq!(
        version.normalized_build_code(),
        "02020010110000000000000000000000"
    );

    assert_eq!(release.build_hash(), None);
    assert_eq!(
        release.to_string(),
        "org.example.FooApp@1.0.0-rc1+20200101100"
    );

    assert_eq!(release.describe().to_string(), "1.0.0-rc1 (20200101100)");
}

#[test]
fn test_basic_ios_ver2() {
    let release = Release::parse("org.example.FooApp@1.0rc1+1.2.3").unwrap();
    assert_eq!(release.package(), Some("org.example.FooApp"));
    assert_eq!(release.version_raw(), "1.0rc1+1.2.3");

    let version = release.version().unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 0);
    assert_eq!(version.patch(), 0);
    assert_eq!(version.triple(), (1, 0, 0));
    assert_eq!(version.pre(), Some("rc1"));
    assert_eq!(version.build_code(), Some("1.2.3"));
    assert_eq!(
        version.normalized_build_code(),
        "00000000000100000000020000000003"
    );

    assert_eq!(release.build_hash(), None);
    assert_eq!(release.to_string(), "org.example.FooApp@1.0.0-rc1+1.2.3");

    assert_eq!(release.describe().to_string(), "1.0.0-rc1 (1.2.3)");
}

#[test]
fn test_invalid_releases() {
    assert_eq!(Release::parse("  foo   ").unwrap().version_raw(), "foo");
    assert_eq!(
        Release::parse("foo/bar"),
        Err(InvalidRelease::BadCharacters)
    );
    assert_eq!(Release::parse("."), Err(InvalidRelease::RestrictedName));
    assert_eq!(Release::parse(".."), Err(InvalidRelease::RestrictedName));
    assert_eq!(
        Release::parse("latest"),
        Err(InvalidRelease::RestrictedName)
    );
}
