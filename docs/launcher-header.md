# Optional inline launcher header

Stock-compatible generated widgets remain OWL6. The inline cog/refresh/back beside the launcher close icon requires the OWL8 `screen.header` extension. The included `patches/omalauncher-header-actions.patch` records the local Omalauncher changes tested against upstream commit `ca99985b3536d26dfd8935391bbfa5c0d215488a`. It also sets a separate debug application ID/file authority for side-by-side installation and uses the active theme accent for close.

Apply the patch in an Omalauncher source checkout, fetch its native PRoot libraries using its documented build steps, and build/test that launcher. A separate app has separate configuration and Linux data: installing it does not automatically migrate the original app. Preserve the original installation. No APK or native libraries are committed here.

Generate the matching widget after `make assemble`:

```sh
python3 tools/header-variant.py /tmp/70-miilo-fleet.owl
```

Install that output into the patched launcher's widget directory. The default installer intentionally installs OWL6 for compatibility. `--preview-robots` adds clearly labeled tile-only visual examples; it never registers providers or creates action targets.

Local validation: the patched launcher passed 98 Android unit tests and 83 OWL tests. The live emulator uses the patched app with preserved real configuration. All current robot/evidence/action rendering resides in the shared widget fragments; the generator only moves controls into the native header.

The patch modifies upstream Omalauncher code and remains subject to that project’s license; the widget repository’s license does not replace it.
