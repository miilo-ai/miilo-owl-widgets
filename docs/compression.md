# Compression validation

The helper negotiates gzip/Brotli/zstd JSON responses and enforces its existing
2,000,000-byte limit after decoding. A compressed response cannot bypass that
limit. The source test `helper/tests/compressed_http.rs` exercises all three
encodings and over-limit decoded payloads through real sockets.

No connection enrollment, credentials, snapshot semantics, action permissions or
revision confirmation behavior changes. Local observation does not require an
HTTP server. The widget does not compress requests or voice streams.

For explicitly selected history/evidence files, install the native Miilo CLI:

```sh
miilo archive export selected.jsonl history.miilo --codec zstd
miilo archive inspect history.miilo
miilo archive restore history.miilo restored.jsonl
```

This is a local terminal workflow, with no account/API requirement. It does not
silently archive raw transcripts. Restoring returns a new byte file and does not
enroll it as an agent session. Existing outputs are never overwritten. Compression
is not encryption or a reduction in model tokens.

Validation completed on macOS and on ARM64 Linux/musl using the exact SDK pin and
locked dependencies in the existing builder (2 CPUs/2 GiB). The release helper
remains a stripped statically linked ARM64 ELF; this checkpoint is 2,887,880 bytes.
SHA-256: 4281acd2ec3c0ba90af2dd2acf7585840a3cdd82ad637591d6b66e825e90ad03.
The builder read local Git objects for the pinned SDK; no credential was mounted.
Native Android/Omarchy interaction and whole-product performance remain device
acceptance gates. No installed application was replaced by these source tests.
