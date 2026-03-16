# VectorDB Learning Phases

## Project: vectra (Vector Search Engine in Rust)

A from-scratch vector database built for deep learning of ANN algorithms, systems engineering, and Rust.

**Approach**: Guided build — Claude sets up scaffolding, user implements core logic (5-10 lines at key decision points). Each phase produces a working, benchmarkable system.

**Target Scale**: 100K-1M vectors (medium scale — where brute force breaks and indexing becomes essential).

---

## Phase 1: Foundations — "The Dumb Version"
**Status**: Not Started
**Goal**: In-memory vector store with brute-force search

### What Claude Sets Up
- Project structure (`cargo init`, modules, tests scaffold)
- Vector struct and store skeleton
- Test cases and benchmark harness

### What User Implements
- [ ] Distance functions: cosine similarity, euclidean distance, dot product
- [ ] Brute-force KNN search (iterate all vectors, return top-K)
- [ ] Basic insert logic

### Key Concepts to Learn
- What are embeddings and why vectors represent meaning
- Why cosine similarity is preferred over euclidean for normalized vectors
- The O(n*d) cost of brute force (n=vectors, d=dimensions)
- How to benchmark and measure baseline performance

### Rust Concepts Introduced
- Structs, Vec<f32>, iterators
- `impl` blocks, methods
- Basic traits (`Ord`, `PartialOrd` for sorting)
- Unit tests with `#[test]`

### Exit Criteria
- [ ] All three distance functions pass unit tests
- [ ] KNN search returns correct results on small test set
- [ ] Benchmark: measure queries/sec on 10K random vectors (128d)

---

## Phase 2: Persistence — "Don't Lose My Data"
**Status**: Not Started
**Goal**: Serialize vectors to disk, memory-mapped file access

### What Claude Sets Up
- File format design discussion
- Storage module skeleton
- Test fixtures with known data

### What User Implements
- [ ] Binary serialization format for vectors + metadata
- [ ] Write vectors to disk
- [ ] Read vectors from disk using memory-mapped I/O
- [ ] Validate data integrity on load

### Key Concepts to Learn
- How databases persist data (pages, segments, headers)
- Memory-mapped I/O — why it's powerful for read-heavy workloads
- The difference between serialization formats (binary vs JSON vs protobuf)
- How mmap lets the OS manage your cache

### Rust Concepts Introduced
- File I/O (`std::fs`)
- `serde` for serialization
- `memmap2` crate for mmap
- Error handling with `Result<T, E>` and custom errors

### Exit Criteria
- [ ] Write 100K vectors to disk, read them back, verify correctness
- [ ] Benchmark: mmap read vs full file read performance comparison
- [ ] Storage format is documented

---

## Phase 3: Indexing — "Make It Fast" (Core Phase)
**Status**: Not Started
**Goal**: HNSW (Hierarchical Navigable Small World) index from scratch

### What Claude Sets Up
- HNSW paper walkthrough and explanation
- Graph data structure skeleton
- Visualization helpers for debugging the graph

### What User Implements
- [ ] Random level generation (exponential distribution)
- [ ] Greedy search on a single layer
- [ ] Multi-layer search (entry point -> layer 0)
- [ ] Node insertion with neighbor selection
- [ ] KNN query using the HNSW graph

### Key Concepts to Learn
- Skip list intuition → how HNSW layers work
- Greedy search on graphs — why it works for high-dimensional spaces
- The "small world" property — short paths between any two nodes
- Trade-off: construction time vs query speed vs recall
- Parameters: M (max connections), efConstruction, efSearch

### Rust Concepts Introduced
- Graph representation (adjacency lists)
- `rand` crate for random number generation
- Generics and trait bounds
- Lifetimes (if needed for graph references)

### Exit Criteria
- [ ] HNSW index builds on 100K vectors (128d)
- [ ] Recall@10 >= 0.95 on test queries
- [ ] Benchmark: HNSW search vs brute force (expect 50-100x speedup)
- [ ] Understand impact of M and efSearch on recall vs speed

---

## Phase 4: Quantization — "Make It Fit"
**Status**: Not Started
**Goal**: Compress vectors to reduce memory footprint

### What Claude Sets Up
- Quantization theory explanation
- Scaffolding for scalar and product quantization
- Memory usage measurement tools

### What User Implements
- [ ] Scalar quantization (f32 -> u8 with min/max scaling)
- [ ] Approximate distance computation on quantized vectors
- [ ] Product quantization (split vector into subspaces, codebook per subspace)
- [ ] Asymmetric distance computation (query=full, db=quantized)

### Key Concepts to Learn
- Why memory is the bottleneck at scale (not compute)
- Scalar quantization: simple, ~4x compression, minimal accuracy loss
- Product quantization: ~32x compression, requires training codebooks
- Asymmetric vs symmetric distance computation trade-offs

### Rust Concepts Introduced
- Bit manipulation, type casting (`as u8`)
- Unsafe Rust for SIMD intrinsics (optional)
- Trait-based abstraction over quantization strategies
- `std::arch` for SIMD (x86_64)

### Exit Criteria
- [ ] Scalar quantization: <2% recall loss at 4x memory reduction
- [ ] Product quantization: working codebook training and search
- [ ] Benchmark: memory usage and speed comparison (full vs SQ vs PQ)

---

## Phase 5: Metadata & Filtering — "Real-World Queries"
**Status**: Not Started
**Goal**: Combine vector search with metadata-based filtering

### What Claude Sets Up
- Metadata storage design
- Filter expression parser skeleton
- Test cases with mixed vector + metadata queries

### What User Implements
- [ ] Metadata storage alongside vectors (key-value pairs)
- [ ] Pre-filtering strategy (filter first, then vector search)
- [ ] Post-filtering strategy (vector search first, then filter)
- [ ] Decide threshold: when to use pre vs post filtering

### Key Concepts to Learn
- The hard problem: vector indices don't support arbitrary filters
- Pre-filter: accurate but may reduce candidate set too much
- Post-filter: fast but may not find enough matches
- How production DBs solve this (filtered HNSW, hybrid approaches)

### Rust Concepts Introduced
- HashMap and BTreeMap for metadata indices
- Enum-based filter expressions
- Trait objects (`dyn Filter`) or generics
- Iterator combinators (`.filter()`, `.take()`)

### Exit Criteria
- [ ] Filtered queries return correct results
- [ ] Pre vs post filter performance comparison documented
- [ ] Edge case: filter excludes most vectors (high selectivity)

---

## Phase 6: API & Polish — "Ship It"
**Status**: Not Started
**Goal**: HTTP API, documentation, and portfolio-ready packaging

### What Claude Sets Up
- API framework setup (axum or tonic)
- Endpoint scaffolding
- Docker setup for easy deployment

### What User Implements
- [ ] API design (collection CRUD, vector insert, search endpoint)
- [ ] Request validation and error responses
- [ ] Batch insert optimization
- [ ] Simple client example

### Key Concepts to Learn
- How vector DB APIs are designed (compare Qdrant/Pinecone/Weaviate APIs)
- Batch operations — why they matter for ingestion performance
- API design trade-offs (REST vs gRPC, sync vs async)

### Rust Concepts Introduced
- Async Rust with `tokio`
- `axum` or `tonic` web framework
- `serde_json` for request/response
- `Arc<Mutex<>>` or `RwLock` for shared state

### Exit Criteria
- [ ] Full CRUD API working
- [ ] Can insert 100K vectors via API and search them
- [ ] README with architecture diagram
- [ ] Benchmark results documented

---

## Progress Tracking

| Phase | Status | Started | Completed |
|-------|--------|---------|-----------|
| 1. Foundations | Not Started | - | - |
| 2. Persistence | Not Started | - | - |
| 3. Indexing (HNSW) | Not Started | - | - |
| 4. Quantization | Not Started | - | - |
| 5. Metadata & Filtering | Not Started | - | - |
| 6. API & Polish | Not Started | - | - |

## References

- [HNSW Paper](https://arxiv.org/abs/1603.09320) — Malkov & Yashunin, 2018
- [Product Quantization Paper](https://hal.inria.fr/inria-00514462v2/document) — Jegou et al., 2011
- [Qdrant Source (Rust)](https://github.com/qdrant/qdrant) — reference implementation
- [hnswlib (C++)](https://github.com/nmslib/hnswlib) — reference HNSW implementation
