# Mie Scattering Monte Carlo

This Rust workspace simulates photon transport and Mie scattering in an
infinite circular cylinder.

## Documentation

- [`workflow.md`](workflow.md): concise model and simulation workflow.
- [`docs/execution-flow.md`](docs/execution-flow.md): detailed execution,
  units, sampling, boundary handling, statistics, outputs, and limitations.

## Run

From the repository root:

```powershell
cargo run -p simulation --release
```

The simulation parameters are in `simulation/src/simulat_const.rs`. The current
configuration requests 500,000,000,000 photons; reduce `PHOTON_CONST` when
running a short test.

Run the workspace tests with:

```powershell
cargo test --workspace
```