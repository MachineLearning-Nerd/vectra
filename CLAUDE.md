# CLAUDE.md — vectra (Vector Search Engine)

## Project Overview
A vector database built from scratch in Rust for deep learning of ANN algorithms and systems engineering.
This is a **learning project** — the process matters more than the output.

## Golden Rules

### 1. Never Implement Core Logic for the User
The user is learning by building. For items marked "What User Implements" in LEARNING_PHASE.md:
- Set up the scaffolding (function signatures, types, test cases)
- Add clear TODO comments explaining what needs to be implemented
- Explain the concept and trade-offs
- Let the user write the 5-10 lines of critical logic
- Review and discuss their implementation after

### 2. Follow the Phase Order
- Always check LEARNING_PHASE.md for current phase status
- Do NOT skip ahead to later phases
- Do NOT introduce concepts from future phases prematurely
- Each phase must meet its exit criteria before moving on

### 3. Explain, Don't Just Code
- Before writing any scaffolding, explain WHY this structure was chosen
- After the user implements something, discuss what they wrote
- Connect implementation details to how production vector DBs work
- Use benchmarks as teaching tools (not just validation)

### 4. Rust Teaching is Incremental
- Only introduce Rust concepts listed for the current phase
- Don't use advanced patterns (async, unsafe, macros) until the phase that needs them
- When a Rust concept is new, explain it briefly in context
- Prefer readable code over idiomatic cleverness in early phases

## Project Structure (evolves per phase)
```
vectra/
  Cargo.toml
  src/
    lib.rs          # Public API
    distance.rs     # Phase 1: Distance functions
    store.rs        # Phase 1: Vector storage
    persistence.rs  # Phase 2: Disk I/O
    hnsw.rs         # Phase 3: HNSW index
    quantize.rs     # Phase 4: Quantization
    filter.rs       # Phase 5: Metadata filtering
    api/            # Phase 6: HTTP API
  tests/
    integration/    # Integration tests per phase
  benches/
    benchmarks.rs   # Criterion benchmarks
```

## Coding Standards
- Use `f32` for vector components (standard in vector search)
- Dimension is always `usize`, configurable per collection
- Error handling: `thiserror` for library errors, `anyhow` in tests/benches
- Testing: `#[cfg(test)]` module in each file + integration tests
- Benchmarks: `criterion` crate, always compare against brute force baseline
- No `unwrap()` in library code — use proper error handling
- Comments: explain WHY, not WHAT (the code shows what)

## Naming Conventions
- Types: PascalCase (`VectorStore`, `HnswIndex`)
- Functions: snake_case (`cosine_similarity`, `search_knn`)
- Constants: SCREAMING_SNAKE_CASE (`DEFAULT_EF_SEARCH`)
- Module files: snake_case matching the concept

## Dependencies Policy
- Minimize external crates — learn by implementing
- Allowed: `rand`, `serde`, `criterion`, `thiserror`, `memmap2`
- Phase 6 only: `tokio`, `axum` or `tonic`, `serde_json`
- Do NOT add: `ndarray`, `nalgebra`, or any vector math library (defeats the purpose)
- Every dependency must be justified

## Testing Approach
- TDD: write tests first, then implement
- Every distance function: test against known values
- Every search: test correctness before benchmarking
- Benchmark at the end of each phase to see progress
- Use fixed seeds for reproducible test data

## What NOT To Do
- Do NOT generate embedding vectors from real data (use random vectors for testing)
- Do NOT build a CLI or TUI until Phase 6
- Do NOT optimize prematurely — correctness first, then speed
- Do NOT add features not in the current phase
- Do NOT use `unsafe` until Phase 4 (SIMD)
- Do NOT add vector math crate dependencies — implement distance functions by hand
