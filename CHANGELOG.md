# Changelog

## [Unreleased] - ReleaseDate

### Changed
- Broadened the dependency spec on Nix, for better compatibility with
  downstream consumers that have various requirements. (#80)

## [0.3.0] - 2024-10-01
### Changed
- Updated CI to use latest supported FreeBSD versions (#48)
- Updated the nix crate (#64)
- Updated the sysctl crate (#48)
- Updated all dependencies (#48)
- Update to the 2021 edition (#48)
- Eliminate dependency on unmaintained "users" crate (#51)
- Fix the build with `-Zdirect-minimal-versions` (#60)

## [0.2.0] - 2021-09-25
### Changed
- CI is now based on GitHub Actions (#25)
- Updated the nix crate to 0.20 (#28)
- Updated the number_prefix crate (#18)
- Updated the users crate (#22)
- Update to 2018 edition (#37)
- Update the nix crate (#41)

### Fixed
- Fixed build on ARM/POWER (#29)
- Handle FreeBSD 13 changes in State::check (#31)

## [0.1.0] - 2019-03-30
### Added
- Serde feature flag (#6)
- Code Coverage scan (#12)
### Changed
- Updated the Users crate (#1, #7)
- Updated the sysctl crate (#9)
- Updated the nix crate (#11)
- Updated the number_prefix crate (#10)

## [0.0.5] - 2018-12-20
### Added
- This Changelog
- Docs on github pages
- Repository tag on crates.io
