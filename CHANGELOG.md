# Changelog

## 1.3.2

### Various fixes & improvements

- fix: Return borrowed lifetime from all accessor methods (#43) by @jan-auer
- build(deps): bump ansi-regex from 4.1.0 to 4.1.1 (#33) by @dependabot

## 1.3.1

- Fix invalid release names by disallowing NUL bytes and other control characters. ([#30](https://github.com/getsentry/sentry-release-parser/pull/30), [#31](https://github.com/getsentry/sentry-release-parser/pull/31))

## 1.3.0

- Add ability to compare versions. ([#25](https://github.com/getsentry/sentry-release-parser/pull/25))

## 1.2.0

- Retain original version formatting. ([#23](https://github.com/getsentry/sentry-release-parser/pull/23))

## 1.1.4

- Lower compile target to ES6

## 1.1.3

- Add back TypeScript definitions to the NPM package.

## 1.1.2

- Single component versions require dash for pre-releases now. ([#20](https://github.com/getsentry/sentry-release-parser/pull/20))

## 1.1.1

- Apply case insensitive checks for restricted release and environment names. ([#18](https://github.com/getsentry/sentry-release-parser/pull/18))

## 1.1.0

- Add validation functions for releases and environments. ([#17](https://github.com/getsentry/sentry-release-parser/pull/17))
- Reject empty release versions as invalid. ([#16](https://github.com/getsentry/sentry-release-parser/pull/16))

## 1.0.0

Initial stable release.
