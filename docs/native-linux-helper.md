# Native Linux helper downloads

Miilo One for Omarchy needs `miilo-owl` in addition to `miilo-local`. These are
separate releases from the plugin. For plugin 0.5.0 the helper version is 1.7.2.

The original 1.7.2 release provided an ARM64 OmaLauncher widget bundle only.
The native-helper recovery workflow adds a helper-only x86-64 archive built from
that same immutable tag. It does not change the tag, replace ARM64 downloads,
or include the later Android conversation work.

## Download and verify

Use the helper's [GitHub Releases](https://github.com/miilo-ai/miilo-owl-widgets/releases/tag/v1.7.2).
For an x86-64 Omarchy desktop, select:

- `miilo-owl-1.7.2-linux-x86_64.tar.gz`
- `SHA256SUMS-native-x86_64`

Run `sha256sum -c SHA256SUMS-native-x86_64` before extraction. The archive
contains the static `miilo-owl` binary, license, installation instructions and
`BUILD-INFO` identifying source commit and compilation target. The widget bundle's
older `SHA256SUMS` remains unchanged and does not cover this added native archive.
The helper-only download does not contain an OmaLauncher installer or widgets.

The source commit for `v1.7.2` is `ad17ef9462df770f82b6a7db31b52332bbf27adc`.
The target is `x86_64-unknown-linux-musl`. Install the verified binary on your
shell PATH as described in the included `INSTALL.md`; configure observation,
model credentials and the plugin separately. Installing the helper does not
select sessions, enable capture or start a collector.

## Manual build and publication

In GitHub Actions, select **Build native Linux helper**, then **Run workflow**:

1. Choose `main` as the workflow branch, tag `v1.7.2`, architecture `x86_64`,
   and leave `publish` unchecked for a build-only rehearsal.
2. The workflow builds the tag's locked source and runs native Rust tests. It
   extracts the package and executes its binary as a non-root process in an
   empty container with no network or extra runtime libraries.
3. Inspect the uploaded workflow artifact. Repeat with `publish` checked to
   append the package and its separate checksum to the existing release.
   The release environment's protections and the repository's emergency release
   pause still apply.

Publishing is allowed only from the default workflow branch. The source tag must
already have a GitHub release, its package version must match, and build metadata
must match the tag's full commit SHA. Existing asset names cause refusal; there
is no overwrite, tag movement, or automatic asset deletion. A failed or partial
publication needs maintainer inspection before retrying.

Regular future widget releases build and test both native architectures. Their
per-architecture archives are verified before merging the checksum list; a
missing architecture or mismatched checksum prevents publication.
