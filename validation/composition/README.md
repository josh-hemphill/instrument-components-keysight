# All-five static composition proof

This application references all five independently packaged driver libraries,
the facade/runtime, and development-only TestKit. Production driver code is
resolved from versioned package archives, never sibling paths, copied SDK DLLs,
reflection activation, or implementation project references.

`composition-lock.json` pins the source commit and successful package-proof CI
run for every repository. `fetch-inputs.py` verifies each run's source, success,
workflow path, artifact digest, and golden-file hashes. Only source-derived
case data/runners are copied from those locked sources into the app. The
application consumes the actual release-form packages in those CI artifacts.
It also records the selected SDK archive hashes; these are explicit artifact
bootstrap inputs, not public-registry evidence.

`validate.py` creates fresh SDK/package feeds and package caches, validates
production dependency/resource boundaries, and runs:

- Every 47 IDN hint plus USB, same-IDN/USB consolidation, conflicting identity
  evidence and equal-rank ambiguity, all without IO.
- One 2602B backend shared by typed DMM/PSU views and one connection lifetime.
- Every independently specified external wire case through all execution modes.
- Rust default/async consumers and clippy, actual C# Native AOT publication and
  execution, native x64 PE/ELF checks, no shared .NET runtime, Linux source/network
  isolation, repeated lifecycle execution and a deliberately failing IL2026 gate.

The `composition.yml` workflow runs Linux/Windows x64 × .NET8/10. The later bench
software gate separately packages the pinned standards VISA transport, removes
its source checkout and runs the bench's fake self-test/config validation. That
step does not open VISA or claim bench evidence. The OpenTAP host is separate.

For local development, install the reviewed SDK wheel, fetch sources with
`fetch-inputs.py --sources-only`, and supply the five per-driver package-proof
artifact directories to `validate.py`. `--development-inputs` explicitly labels
unhosted inputs and cannot be presented as immutable CI or public-registry
validation. Linux namespace setup needs bubblewrap permissions on the host;
CI uses sudo only for this isolated execution.

After previews are published, the driver preview workflows also run fresh
public-registry package consumers. Record those separately. Physical evidence
is collected later using the reviewed configuration in `validation/bench`.
