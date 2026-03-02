# Quantum Computing Experiments

Small rust sandbox for quick quantum-computing experiments

## Running Examples

`kets-bra` builds the Pauli-X gate from ket/bra outer products and applies it to a state vector.

```bash
cargo run --example kets-bra
```

`bloch-sphere` renders a Bloch sphere visualization from `theta` and `phi` (degrees) and writes `target/bloch-sphere.svg`.

```bash
cargo run --example bloch-sphere
cargo run --example bloch-sphere -- 60 45
```
