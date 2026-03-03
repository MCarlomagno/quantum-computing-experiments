# Quantum Computing Experiments

Small sandbox for learning quantum computing.

## Rust Examples

`kets-bra` builds the Pauli-X gate from ket/bra outer products and applies it to a state vector.

```bash
cargo run --example kets-bra
```

`bloch-sphere` renders a Bloch sphere visualization from `theta` and `phi` (degrees) and writes `target/bloch-sphere.svg`.

```bash
cargo run --example bloch-sphere
cargo run --example bloch-sphere -- 60 45
```

## Qiskit notebooks

This repo also includes a Python notebook and scripts for Qiskit workflows against IBM Quantum backends.

Use the Miniconda environments

```bash
conda activate <name>
```

If needed, install dependencies:

```bash
pip install qiskit qiskit-ibm-runtime matplotlib
```

Set your IBM token before running notebook cells that connect to IBM Quantum:

```bash
export IBM_TOKEN=your_token_here
```