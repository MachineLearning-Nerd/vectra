/// Distance functions for comparing vectors.
///
/// These are the mathematical heart of a vector database.
/// Every search operation ultimately calls one of these functions
/// millions of times, so understanding them deeply matters.

use crate::error::VectraError;

/// Represents the different ways to measure "similarity" between vectors.
/// Each metric has different geometric meaning and use cases.
#[derive(Debug, Clone, Copy)]
pub enum DistanceMetric {
    /// Measures the angle between two vectors (ignores magnitude).
    /// Range: 0.0 (identical direction) to 2.0 (opposite direction)
    /// Most common for text embeddings (OpenAI, Cohere, etc.)
    Cosine,

    /// Straight-line distance between two points in space.
    /// Range: 0.0 (identical) to infinity
    /// Good for image embeddings and spatial data.
    Euclidean,

    /// Sum of element-wise products. Higher = more similar.
    /// Range: -infinity to +infinity
    /// Fastest to compute. Used when vectors are already normalized.
    DotProduct,
}

/// Validates that two vectors can be compared.
/// Both must be non-empty and have the same number of dimensions.
fn validate_vectors(a: &[f32], b: &[f32]) -> Result<(), VectraError> {
    if a.is_empty() || b.is_empty() {
        return Err(VectraError::EmptyVector);
    }
    if a.len() != b.len() {
        return Err(VectraError::DimensionMismatch {
            expected: a.len(),
            actual: b.len(),
        });
    }
    Ok(())
}

/// Compute the dot product of two vectors.
///
/// The dot product is the sum of element-wise multiplications:
///   dot(a, b) = a[0]*b[0] + a[1]*b[1] + ... + a[n]*b[n]
///
/// Geometrically, it measures how much two vectors "agree" in direction,
/// scaled by their magnitudes. For normalized vectors (length=1),
/// dot product equals cosine similarity.
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector (must have same length as `a`)
///
/// # Returns
/// The dot product as f32. Higher values = more similar.
pub fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, VectraError> {
    validate_vectors(a, b)?;

    // TODO: Implement the dot product
    //
    // Hint: You need to multiply corresponding elements and sum them up.
    // Rust's iterator methods make this clean:
    //   - .iter() gives you an iterator over the slice
    //   - .zip() pairs up elements from two iterators
    //   - .map() transforms each pair
    //   - .sum() adds everything up
    //
    // Try to do it in one iterator chain!

    todo!("Implement dot_product")
}

/// Compute the cosine similarity between two vectors, returned as a distance.
///
/// Cosine similarity measures the angle between two vectors,
/// ignoring their magnitude (length). This is crucial because
/// embedding models don't guarantee consistent vector lengths.
///
/// Formula:
///   cosine_similarity = dot(a, b) / (||a|| * ||b||)
///   cosine_distance = 1.0 - cosine_similarity
///
/// Where ||a|| is the L2 norm (length) of vector a:
///   ||a|| = sqrt(a[0]^2 + a[1]^2 + ... + a[n]^2)
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector (must have same length as `a`)
///
/// # Returns
/// Cosine distance: 0.0 means identical direction, 2.0 means opposite.
pub fn cosine_distance(a: &[f32], b: &[f32]) -> Result<f32, VectraError> {
    validate_vectors(a, b)?;

    // TODO: Implement cosine distance
    //
    // Steps:
    //   1. Compute dot product of a and b
    //      (you can call your dot_product function, or compute inline)
    //   2. Compute the L2 norm of a: sqrt(sum of squares of a's elements)
    //   3. Compute the L2 norm of b: sqrt(sum of squares of b's elements)
    //   4. Handle edge case: if either norm is 0.0, return error (zero vector)
    //   5. Return 1.0 - (dot_product / (norm_a * norm_b))
    //
    // Useful: f32 has .sqrt() method, and you can use iter().map(|x| x * x).sum::<f32>()

    todo!("Implement cosine_distance")
}

/// Compute the Euclidean distance between two vectors.
///
/// This is the "straight line" distance — the same distance formula
/// you learned in geometry, extended to N dimensions.
///
/// Formula:
///   distance = sqrt((a[0]-b[0])^2 + (a[1]-b[1])^2 + ... + (a[n]-b[n])^2)
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector (must have same length as `a`)
///
/// # Returns
/// Euclidean distance: 0.0 means identical, larger means more different.
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32, VectraError> {
    validate_vectors(a, b)?;

    // TODO: Implement euclidean distance
    //
    // Steps:
    //   1. For each pair of elements (a[i], b[i]), compute the difference
    //   2. Square each difference
    //   3. Sum all squared differences
    //   4. Take the square root of the sum
    //
    // This can be done in one iterator chain:
    //   .iter().zip().map(difference and square).sum() then .sqrt()

    todo!("Implement euclidean_distance")
}

/// Convenience function: compute distance using the specified metric.
pub fn compute_distance(a: &[f32], b: &[f32], metric: DistanceMetric) -> Result<f32, VectraError> {
    match metric {
        DistanceMetric::Cosine => cosine_distance(a, b),
        DistanceMetric::Euclidean => euclidean_distance(a, b),
        DistanceMetric::DotProduct => {
            // For dot product, we negate so that "lower = more similar"
            // (consistent with other distance metrics)
            dot_product(a, b).map(|d| -d)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Dot Product Tests ───────────────────────────────

    #[test]
    fn test_dot_product_basic() {
        // [1, 2, 3] . [4, 5, 6] = 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = dot_product(&a, &b).unwrap();
        assert!((result - 32.0).abs() < 1e-6, "Expected 32.0, got {}", result);
    }

    #[test]
    fn test_dot_product_orthogonal() {
        // Perpendicular vectors have dot product = 0
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let result = dot_product(&a, &b).unwrap();
        assert!((result - 0.0).abs() < 1e-6, "Orthogonal vectors should have dot product 0");
    }

    #[test]
    fn test_dot_product_identical() {
        // [3, 4] . [3, 4] = 9 + 16 = 25 (this is the squared magnitude)
        let a = vec![3.0, 4.0];
        let result = dot_product(&a, &a).unwrap();
        assert!((result - 25.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product_negative() {
        // Opposite vectors have negative dot product
        let a = vec![1.0, 2.0];
        let b = vec![-1.0, -2.0];
        let result = dot_product(&a, &b).unwrap();
        assert!((result - (-5.0)).abs() < 1e-6);
    }

    // ─── Cosine Distance Tests ───────────────────────────

    #[test]
    fn test_cosine_identical_vectors() {
        // Same direction = distance 0
        let a = vec![1.0, 2.0, 3.0];
        let result = cosine_distance(&a, &a).unwrap();
        assert!(result.abs() < 1e-6, "Identical vectors should have cosine distance ~0, got {}", result);
    }

    #[test]
    fn test_cosine_opposite_vectors() {
        // Opposite direction = distance 2
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];
        let result = cosine_distance(&a, &b).unwrap();
        assert!((result - 2.0).abs() < 1e-6, "Opposite vectors should have cosine distance ~2.0, got {}", result);
    }

    #[test]
    fn test_cosine_orthogonal_vectors() {
        // Perpendicular = distance 1
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let result = cosine_distance(&a, &b).unwrap();
        assert!((result - 1.0).abs() < 1e-6, "Orthogonal vectors should have cosine distance ~1.0, got {}", result);
    }

    #[test]
    fn test_cosine_magnitude_invariant() {
        // Cosine should not care about magnitude
        let a = vec![1.0, 0.0];
        let b = vec![100.0, 0.0]; // same direction, much larger
        let result = cosine_distance(&a, &b).unwrap();
        assert!(result.abs() < 1e-6, "Cosine distance should ignore magnitude, got {}", result);
    }

    // ─── Euclidean Distance Tests ────────────────────────

    #[test]
    fn test_euclidean_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let result = euclidean_distance(&a, &a).unwrap();
        assert!(result.abs() < 1e-6, "Distance to self should be 0");
    }

    #[test]
    fn test_euclidean_known_value() {
        // Classic 3-4-5 triangle
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        let result = euclidean_distance(&a, &b).unwrap();
        assert!((result - 5.0).abs() < 1e-6, "Expected 5.0, got {}", result);
    }

    #[test]
    fn test_euclidean_symmetric() {
        // distance(a, b) == distance(b, a)
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let d1 = euclidean_distance(&a, &b).unwrap();
        let d2 = euclidean_distance(&b, &a).unwrap();
        assert!((d1 - d2).abs() < 1e-6, "Euclidean distance should be symmetric");
    }

    #[test]
    fn test_euclidean_1d() {
        // In 1D, euclidean distance = absolute difference
        let a = vec![3.0];
        let b = vec![7.0];
        let result = euclidean_distance(&a, &b).unwrap();
        assert!((result - 4.0).abs() < 1e-6);
    }

    // ─── Error Handling Tests ────────────────────────────

    #[test]
    fn test_dimension_mismatch() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(dot_product(&a, &b).is_err());
        assert!(cosine_distance(&a, &b).is_err());
        assert!(euclidean_distance(&a, &b).is_err());
    }

    #[test]
    fn test_empty_vectors() {
        let a: Vec<f32> = vec![];
        let b: Vec<f32> = vec![];
        assert!(dot_product(&a, &b).is_err());
        assert!(cosine_distance(&a, &b).is_err());
        assert!(euclidean_distance(&a, &b).is_err());
    }
}
