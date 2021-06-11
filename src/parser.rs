use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

#[cfg(feature = "serde")]
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

lazy_static! {
    static ref RELEASE_REGEX: Regex = Regex::new(r#"^(@?[^@]+)@(.+?)$"#).unwrap();
    static ref VERSION_REGEX: Regex = Regex::new(
        r#"(?x)
        ^
            (?P<major>[0-9][0-9]*)
            (?:\.(?P<minor>[0-9][0-9]*))?
            (?:\.(?P<patch>[0-9][0-9]*))?
            (?:\.(?P<revision>[0-9][0-9]*))?
            (?:
                (?P<prerelease>
                    (?:-|[a-z])
                    (?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*)?
                    (?:\.(?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*))*)
                )?
            (?:\+(?P<build_code>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?
        $
    "#
    )
    .unwrap();
    static ref HEX_REGEX: Regex = Regex::new(r#"^[a-fA-F0-9]+$"#).unwrap();
    // what can or cannot go through the API which is a limiting factor for
    // releases and environments.
    static ref VALID_API_ATTRIBUTE_REGEX: Regex = Regex::new(r"^[^/\r\n\t\x0c]*\z").unwrap();
}

/// An error indicating invalid versions.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct InvalidVersion;

impl std::error::Error for InvalidVersion {}

impl fmt::Display for InvalidVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid version")
    }
}

/// An error indicating invalid releases.
#[derive(Debug, Clone, PartialEq)]
pub enum InvalidRelease {
    /// The release name was too long
    TooLong,
    /// Release name is restricted
    RestrictedName,
    /// The release contained invalid characters
    BadCharacters,
}

/// An error indicating invalid environment.
#[derive(Debug, Clone, PartialEq)]
pub enum InvalidEnvironment {
    /// The environment name was too long
    TooLong,
    /// Environment name is restricted
    RestrictedName,
    /// The environment contained invalid characters
    BadCharacters,
}

impl std::error::Error for InvalidRelease {}

impl fmt::Display for InvalidRelease {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid release: {}",
            match *self {
                InvalidRelease::BadCharacters => "bad characters in release name",
                InvalidRelease::RestrictedName => "restricted release name",
                InvalidRelease::TooLong => "release name too long",
            }
        )
    }
}

impl std::error::Error for InvalidEnvironment {}

impl fmt::Display for InvalidEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid environment: {}",
            match *self {
                InvalidEnvironment::BadCharacters => "bad characters in environment name",
                InvalidEnvironment::RestrictedName => "restricted environment name",
                InvalidEnvironment::TooLong => "environment name too long",
            }
        )
    }
}

/// Represents a parsed version.
#[derive(Debug, Clone, PartialEq)]
pub struct Version<'a> {
    raw: &'a str,
    major: &'a str,
    minor: &'a str,
    patch: &'a str,
    revision: &'a str,
    pre: &'a str,
    build_code: &'a str,
    components: u8,
}

#[cfg(feature = "serde")]
impl<'a> Serialize for Version<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Version", 5)?;
        state.serialize_field("major", &self.major())?;
        state.serialize_field("minor", &self.minor())?;
        state.serialize_field("patch", &self.patch())?;
        state.serialize_field("revision", &self.revision())?;
        state.serialize_field("pre", &self.pre())?;
        state.serialize_field("build_code", &self.build_code())?;
        state.serialize_field("components", &self.components())?;
        state.serialize_field("raw_quad", &self.raw_quad())?;
        state.end()
    }
}

fn is_build_hash(s: &str) -> bool {
    match s.len() {
        12 | 16 | 20 | 32 | 40 | 64 => HEX_REGEX.is_match(s),
        _ => false,
    }
}

impl<'a> Version<'a> {
    /// Parses a version from a string.
    pub fn parse(version: &'a str) -> Result<Version<'a>, InvalidVersion> {
        let caps = if let Some(caps) = VERSION_REGEX.captures(version) {
            caps
        } else {
            return Err(InvalidVersion);
        };

        let components = 1
            + caps.get(2).map_or(0, |_| 1)
            + caps.get(3).map_or(0, |_| 1)
            + caps.get(4).map_or(0, |_| 1);
        Ok(Version {
            raw: version,
            major: caps.get(1).map(|x| x.as_str()).unwrap_or_default(),
            minor: caps.get(2).map(|x| x.as_str()).unwrap_or_default(),
            patch: caps.get(3).map(|x| x.as_str()).unwrap_or_default(),
            revision: caps.get(4).map(|x| x.as_str()).unwrap_or_default(),
            pre: caps
                .get(5)
                .map(|x| {
                    let mut pre = x.as_str();
                    if pre.starts_with('-') {
                        pre = &pre[1..]
                    }
                    pre
                })
                .unwrap_or(""),
            build_code: caps.get(6).map(|x| x.as_str()).unwrap_or(""),
            components,
        })
    }

    /// Converts the version into a semver.
    ///
    /// Requires the `semver` feature.
    #[cfg(feature = "semver")]
    pub fn as_semver(&self) -> semver::Version {
        fn split(s: &str) -> Vec<semver::Identifier> {
            s.split('.')
                .map(|item| {
                    if let Ok(val) = item.parse::<u64>() {
                        semver::Identifier::Numeric(val)
                    } else {
                        semver::Identifier::AlphaNumeric(item.into())
                    }
                })
                .collect()
        }

        semver::Version {
            major: self.major(),
            minor: self.minor(),
            patch: self.patch(),
            pre: split(self.pre),
            build: split(self.build_code),
        }
    }

    /// Returns the major version component.
    pub fn major(&self) -> u64 {
        self.major.parse().unwrap_or_default()
    }

    /// Returns the minor version component.
    pub fn minor(&self) -> u64 {
        self.minor.parse().unwrap_or_default()
    }

    /// Returns the patch level version component.
    pub fn patch(&self) -> u64 {
        self.patch.parse().unwrap_or_default()
    }

    /// Returns the revision level version component.
    pub fn revision(&self) -> u64 {
        self.revision.parse().unwrap_or_default()
    }

    /// If a pre-release identifier is included returns that.
    pub fn pre(&self) -> Option<&str> {
        if self.pre.is_empty() {
            None
        } else {
            Some(self.pre)
        }
    }

    /// If a build code is included returns that.
    pub fn build_code(&self) -> Option<&str> {
        if self.build_code.is_empty() {
            None
        } else {
            Some(self.build_code)
        }
    }

    /// Returns the build code as build number.
    pub fn build_number(&self) -> Option<u64> {
        self.build_code().and_then(|val| val.parse().ok())
    }

    /// Returns the number of components.
    pub fn components(&self) -> u8 {
        self.components
    }

    /// Returns the raw version as string.
    ///
    /// It's generally better to use `to_string` which normalizes.
    pub fn raw(&self) -> &str {
        self.raw
    }

    /// Returns the version triple (major, minor, patch)
    pub fn triple(&self) -> (u64, u64, u64) {
        (self.major(), self.minor(), self.patch())
    }

    /// Returns the version quadruple.
    pub fn quad(&self) -> (u64, u64, u64, u64) {
        (self.major(), self.minor(), self.patch(), self.revision())
    }

    /// Returns the version quadruple as raw strings.
    pub fn raw_quad(&self) -> (&str, Option<&str>, Option<&str>, Option<&str>) {
        (
            self.major,
            (self.components > 1).then(|| self.minor),
            (self.components > 2).then(|| self.patch),
            (self.components > 3).then(|| self.revision),
        )
    }
}

impl<'a> fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&VersionDescription(self), f)?;
        if let Some(pre) = self.pre() {
            write!(f, "-{}", pre)?;
        }
        if let Some(build_code) = self.build_code() {
            write!(f, "+{}", build_code)?;
        }
        Ok(())
    }
}

/// Represents a parsed release.
#[derive(Debug, Clone, PartialEq)]
pub struct Release<'a> {
    raw: &'a str,
    package: &'a str,
    version_raw: &'a str,
    version: Option<Version<'a>>,
}

#[cfg(feature = "serde")]
impl<'a> Serialize for Release<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Release", 6)?;
        state.serialize_field("package", &self.package())?;
        state.serialize_field("version_raw", &self.version_raw())?;
        state.serialize_field("version_parsed", &self.version())?;
        state.serialize_field("build_hash", &self.build_hash())?;
        state.serialize_field("description", &self.describe().to_string())?;
        state.end()
    }
}

/// Given a string checks if the release is generally valid.
pub fn validate_release(release: &str) -> Result<(), InvalidRelease> {
    if release.len() > 200 {
        Err(InvalidRelease::TooLong)
    } else if release == "." || release == ".." || release.eq_ignore_ascii_case("latest") {
        Err(InvalidRelease::RestrictedName)
    } else if !VALID_API_ATTRIBUTE_REGEX.is_match(release) {
        Err(InvalidRelease::BadCharacters)
    } else {
        Ok(())
    }
}

/// Given a string checks if the environment name is generally valid.
pub fn validate_environment(environment: &str) -> Result<(), InvalidEnvironment> {
    if environment.len() > 64 {
        Err(InvalidEnvironment::TooLong)
    } else if environment == "." || environment == ".." || environment.eq_ignore_ascii_case("none")
    {
        Err(InvalidEnvironment::RestrictedName)
    } else if !VALID_API_ATTRIBUTE_REGEX.is_match(environment) {
        Err(InvalidEnvironment::BadCharacters)
    } else {
        Ok(())
    }
}

impl<'a> Release<'a> {
    /// Parses a release from a string.
    pub fn parse(release: &'a str) -> Result<Release<'a>, InvalidRelease> {
        let release = release.trim();
        validate_release(release)?;
        if let Some(caps) = RELEASE_REGEX.captures(release) {
            let package = caps.get(1).unwrap().as_str();
            let version_raw = caps.get(2).unwrap().as_str();
            if !is_build_hash(version_raw) {
                let version = Version::parse(version_raw).ok();
                return Ok(Release {
                    raw: release,
                    package,
                    version_raw,
                    version,
                });
            } else {
                return Ok(Release {
                    raw: release,
                    package,
                    version_raw,
                    version: None,
                });
            }
        }
        Ok(Release {
            raw: release,
            package: "",
            version_raw: release,
            version: None,
        })
    }

    /// Returns the raw version.
    ///
    /// It's generally better to use `to_string` which normalizes.
    pub fn raw(&self) -> &str {
        self.raw
    }

    /// Returns the contained package information.
    pub fn package(&self) -> Option<&str> {
        if self.package.is_empty() {
            None
        } else {
            Some(self.package)
        }
    }

    /// The raw version part of the release.
    ///
    /// This is set even if the version part is not a valid version
    /// (for instance because it's a hash).
    pub fn version_raw(&self) -> &str {
        self.version_raw
    }

    /// If a parsed version if available returns it.
    pub fn version(&self) -> Option<&Version<'a>> {
        self.version.as_ref()
    }

    /// Returns the build hash if available.
    pub fn build_hash(&self) -> Option<&str> {
        self.version
            .as_ref()
            .and_then(|x| x.build_code())
            .filter(|x| is_build_hash(x))
            .or_else(|| {
                if is_build_hash(self.version_raw()) {
                    Some(self.version_raw())
                } else {
                    None
                }
            })
    }

    /// Returns a short description.
    ///
    /// This returns a human readable format that includes an abbreviated
    /// name of the release.  Typically it will remove the package and it
    /// will try to abbreviate build hashes etc.
    pub fn describe(&self) -> ReleaseDescription<'_> {
        ReleaseDescription(self)
    }
}

#[derive(Debug)]
struct VersionDescription<'a>(&'a Version<'a>);

impl<'a> fmt::Display for VersionDescription<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.raw_quad() {
            (major, Some(minor), Some(patch), Some(revision)) => {
                write!(f, "{}.{}.{}.{}", major, minor, patch, revision)?;
            }
            (major, Some(minor), Some(patch), None) => {
                write!(f, "{}.{}.{}", major, minor, patch,)?;
            }
            (major, Some(minor), None, None) => {
                write!(f, "{}.{}", major, minor)?;
            }
            (major, None, None, None) => {
                write!(f, "{}", major)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

/// Helper object to format a release into a description.
#[derive(Debug)]
pub struct ReleaseDescription<'a>(&'a Release<'a>);

impl<'a> fmt::Display for ReleaseDescription<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_hash = self
            .0
            .build_hash()
            .map(|hash| hash.get(..12).unwrap_or(hash));

        if let Some(ver) = self.0.version() {
            fmt::Display::fmt(&VersionDescription(ver), f)?;
            if let Some(pre) = ver.pre() {
                write!(f, "-{}", pre)?;
            }
            if let Some(short_hash) = short_hash {
                write!(f, " ({})", short_hash)?;
            } else if let Some(build_code) = ver.build_code() {
                write!(f, " ({})", build_code)?;
            }
        } else if let Some(short_hash) = short_hash {
            write!(f, "{}", short_hash)?;
        } else {
            write!(f, "{}", self.0)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Release<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut have_package = false;
        if let Some(package) = self.package() {
            write!(f, "{}", package)?;
            have_package = true;
        }
        if let Some(version) = self.version() {
            if have_package {
                write!(f, "@")?;
            }
            write!(f, "{}", version)?;
        } else {
            if have_package {
                write!(f, "@")?;
            }
            write!(f, "{}", self.version_raw)?;
        }
        Ok(())
    }
}

#[test]
fn test_release_validation() {
    assert_eq!(
        validate_release("latest"),
        Err(InvalidRelease::RestrictedName)
    );
    assert_eq!(validate_release("."), Err(InvalidRelease::RestrictedName));
    assert_eq!(validate_release(".."), Err(InvalidRelease::RestrictedName));
    assert_eq!(
        validate_release("foo\nbar"),
        Err(InvalidRelease::BadCharacters)
    );
    assert_eq!(validate_release("good"), Ok(()));
}

#[test]
fn test_environment_validation() {
    assert_eq!(
        validate_environment("none"),
        Err(InvalidEnvironment::RestrictedName)
    );
    assert_eq!(
        validate_environment("."),
        Err(InvalidEnvironment::RestrictedName)
    );
    assert_eq!(
        validate_environment(".."),
        Err(InvalidEnvironment::RestrictedName)
    );
    assert_eq!(
        validate_environment("f4f3db928593f258e1d850997be07b577f0779cc5549f9968bae625ea001175bX"),
        Err(InvalidEnvironment::TooLong)
    );
    assert_eq!(
        validate_environment("foo\nbar"),
        Err(InvalidEnvironment::BadCharacters)
    );
    assert_eq!(validate_environment("good"), Ok(()));
}
