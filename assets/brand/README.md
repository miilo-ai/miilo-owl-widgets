# Miilo widget branding

These are the custom Miilo wordmark and its simplified robot symbol. Full robot
portraits remain the observation illustrations; the brand symbol is separate.

Stock OWL6 renders the artwork through generated canvas paths. The generator
samples the SVG curves and preserves the visor cutout without image downloads,
external URLs, fonts or animation clocks. Ink follows the host foreground color.

After editing an SVG, run:

```sh
node tools/generate-brand.cjs
cargo xtask assemble
make check-desktop
```

`node tools/generate-brand.cjs --check` rejects a stale canvas projection.
Generated widget versions come from `helper/Cargo.toml`; About labels this as the
widget version rather than claiming it identifies an independently installed helper.
