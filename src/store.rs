/// In-memory vector store with brute-force search.
///
/// This is the simplest possible vector database: store vectors in a Vec,
/// search by comparing the query against every single stored vector.
/// It's slow at scale but gives us a correct baseline to benchmark against.

use crate::distance::{compute_distance, DistanceMetric};
use crate::error::VectraError;

/// A single vector with its ID and data.
/// In a real database, this would also carry metadata (Phase 5).
#[derive(Debug, Clone)]
pub struct Vector {
    pub id: u64,
    pub data: Vec<f32>,
}

/// A search result: which vector matched and how close it was.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: u64,
    pub distance: f32,
}

/// The vector store holds all vectors and provides insert + search operations.
pub struct VectorStore {
    /// All stored vectors
    vectors: Vec<Vector>,
    /// The number of dimensions each vector must have
    dimension: usize,
    /// Which distance metric to use for search
    metric: DistanceMetric,
    /// Counter for auto-generating IDs
    next_id: u64,
}

impl VectorStore {
    /// Create a new empty vector store.
    ///
    /// # Arguments
    /// * `dimension` - Number of dimensions for each vector (e.g., 128, 384, 1536)
    /// * `metric` - How to measure distance between vectors
    pub fn new(dimension: usize, metric: DistanceMetric) -> Self {
        VectorStore {
            vectors: Vec::new(),
            dimension,
            metric,
            next_id: 0,
        }
    }

    /// Insert a vector into the store. Returns the assigned ID.
    ///
    /// # Arguments
    /// * `data` - The vector data. Must have exactly `self.dimension` elements.
    pub fn insert(&mut self, data: Vec<f32>) -> Result<u64, VectraError> {
        if data.len() != self.dimension {
            return Err(VectraError::DimensionMismatch {
                expected: self.dimension,
                actual: data.len(),
            });
        }

        let id = self.next_id;
        self.next_id += 1;
        self.vectors.push(Vector { id, data });
        Ok(id)
    }

    /// Return how many vectors are stored.
    pub fn len(&self) -> usize {
        self.vectors.len()
    }

    /// Check if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.vectors.is_empty()
    }

    /// Search for the K nearest neighbors to the query vector.
    ///
    /// This is BRUTE FORCE search: it compares the query against every
    /// stored vector. Time complexity is O(n * d) where n is the number
    /// of vectors and d is the dimension.
    ///
    /// # Arguments
    /// * `query` - The query vector to search for neighbors of
    /// * `k` - How many nearest neighbors to return
    ///
    /// # Returns
    /// A Vec of SearchResult, sorted by distance (closest first), with at most K results.
    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>, VectraError> {
        if query.len() != self.dimension {
            return Err(VectraError::DimensionMismatch {
                expected: self.dimension,
                actual: query.len(),
            });
        }
        if k == 0 {
            return Err(VectraError::InvalidK);
        }

        // TODO: Implement brute-force KNN search
        //
        // Steps:
        //   1. Compute the distance from `query` to EVERY vector in self.vectors
        //      Use: compute_distance(&query, &vector.data, self.metric)?
        //
        //   2. Collect results as Vec<SearchResult> (id + distance)
        //
        //   3. Sort by distance (ascending — closest first)
        //      Hint: f32 doesn't implement Ord (because of NaN),
        //      so use .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
        //
        //   4. Truncate to at most K results
        //      Hint: .truncate(k) or .into_iter().take(k).collect()
        //
        //   5. Return the sorted, truncated results
        //
        // This is O(n*d) for computing distances + O(n*log(n)) for sorting.
        // For 100K vectors, this will be noticeably slow — that's the point!
        // Phase 3's HNSW index will make this 50-100x faster.

        todo!("Implement brute-force KNN search")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_len() {
        let mut store = VectorStore::new(3, DistanceMetric::Cosine);
        assert_eq!(store.len(), 0);
        assert!(store.is_empty());

        let id = store.insert(vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(id, 0);
        assert_eq!(store.len(), 1);

        let id2 = store.insert(vec![4.0, 5.0, 6.0]).unwrap();
        assert_eq!(id2, 1);
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_insert_wrong_dimension() {
        let mut store = VectorStore::new(3, DistanceMetric::Cosine);
        let result = store.insert(vec![1.0, 2.0]); // 2D into 3D store
        assert!(result.is_err());
    }

    #[test]
    fn test_search_returns_closest() {
        let mut store = VectorStore::new(2, DistanceMetric::Euclidean);

        // Insert three 2D vectors
        store.insert(vec![0.0, 0.0]).unwrap(); // id 0: origin
        store.insert(vec![1.0, 0.0]).unwrap(); // id 1: close to query
        store.insert(vec![10.0, 10.0]).unwrap(); // id 2: far away

        // Query near [1, 0] — should find id=1 as closest
        let results = store.search(&[1.1, 0.0], 2).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, 1, "Closest should be id=1 (at [1,0])");
        assert_eq!(results[1].id, 0, "Second closest should be id=0 (at [0,0])");
    }

    #[test]
    fn test_search_k_larger_than_store() {
        let mut store = VectorStore::new(2, DistanceMetric::Euclidean);
        store.insert(vec![1.0, 2.0]).unwrap();
        store.insert(vec![3.0, 4.0]).unwrap();

        // Ask for 10 but only 2 exist
        let results = store.search(&[0.0, 0.0], 10).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_cosine() {
        let mut store = VectorStore::new(2, DistanceMetric::Cosine);

        store.insert(vec![1.0, 0.0]).unwrap();  // id 0: pointing right
        store.insert(vec![0.0, 1.0]).unwrap();  // id 1: pointing up
        store.insert(vec![-1.0, 0.0]).unwrap(); // id 2: pointing left

        // Query pointing mostly right — id=0 should be closest
        let results = store.search(&[0.9, 0.1], 3).unwrap();
        assert_eq!(results[0].id, 0, "Vector pointing right should be closest to query pointing mostly right");
    }

    #[test]
    fn test_search_invalid_k() {
        let store = VectorStore::new(2, DistanceMetric::Euclidean);
        assert!(store.search(&[1.0, 2.0], 0).is_err());
    }

    #[test]
    fn test_search_empty_store() {
        let store = VectorStore::new(2, DistanceMetric::Euclidean);
        let results = store.search(&[1.0, 2.0], 5).unwrap();
        assert!(results.is_empty());
    }
}
