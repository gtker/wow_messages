# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.2.0] - 2024-02-21

### Changed

* `lookup_spells` is now a `const` `fn` for all expansions.
* `lookup_spells` is now orders of magnitude faster due to changing to a lookup array rather than linear traversal.

## [0.1.0] - 2023-05-20

### Added

* First release.

<!-- next-url -->
[Unreleased]: https://github.com/gtker/wow_messages/compare/wow_spells-v0.2.0...HEAD
[0.2.0]: https://github.com/gtker/wow_messages/compare/wow_spells-v0.1.1...wow_spells-v0.2.0
[0.1.0]: https://github.com/gtker/wow_messages/releases/tag/wow_spells-v0.1.0
