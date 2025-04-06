/// Computes the sum of all elements in a vector `a`.
///
/// # Arguments
/// * `a` - The vector.
///
/// # Returns
/// The sum of all elements in `a`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let sum_value = intspan::sum(&a);
/// assert_eq!(sum_value, 55.0);
/// ```
pub fn sum(a: &[f32]) -> f32 {
    a.iter().sum()
}

/// Computes the mean (average) of a vector `a`.
///
/// # Arguments
/// * `a` - The vector.
///
/// # Returns
/// The mean of the vector `a`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let mean_value = intspan::mean(&a);
/// assert_eq!(mean_value, 5.5);
/// ```
pub fn mean(a: &[f32]) -> f32 {
    sum(a) / a.len() as f32
}

/// Computes the Pearson correlation coefficient between two vectors `a` and `b`.
///
/// Two equivalent formulas:
///
/// 1. Using deviations from mean (implemented here for better numerical stability):
/// `$r = \frac{\sum(x - \bar{x})(y - \bar{y})}{\sqrt{\sum(x - \bar{x})^2\sum(y - \bar{y})^2}}$`
///
/// 2. Direct computation:
/// `$r = \frac{n\sum xy - \sum x\sum y}{\sqrt{(n\sum x^2 - (\sum x)^2)(n\sum y^2 - (\sum y)^2)}}$`
///
/// where `$\bar{x}$` and `$\bar{y}$` are the means of vectors `$x$` and `$y$` respectively,
/// and `$n$` is the length of the vectors.
///
/// Note: Formula 1 is used in this implementation because it:
/// * Reduces the risk of numerical overflow by centering the data
/// * Provides better numerical stability for large values
///
/// # Arguments
/// * `a` - The first vector.
/// * `b` - The second vector.
///
/// # Returns
/// The Pearson correlation coefficient between `a` and `b`.
/// If either vector is empty or their lengths do not match, returns `NaN`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let b = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
/// let correlation = intspan::pearson_correlation(&a, &b);
/// assert_eq!(format!("{:.4}", correlation), "-1.0000".to_string()); // Perfect negative correlation
///
/// let empty: [f32; 0] = [];
/// assert!(intspan::pearson_correlation(&empty, &empty).is_nan()); // Check handling of empty vectors
/// ```
pub fn pearson_correlation(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return f32::NAN; // Return NaN if lengths do not match or vectors are empty
    }

    // Compute means of a and b
    let mean_a = mean(a);
    let mean_b = mean(b);

    let numerator = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (a - mean_a) * (b - mean_b))
        .sum::<f32>();

    let denom1 = a.iter().map(|a| (a - mean_a).powi(2)).sum::<f32>().sqrt();

    let denom2 = b.iter().map(|b| (b - mean_b).powi(2)).sum::<f32>().sqrt();

    numerator / (denom1 * denom2)
}

/// Computes the Jaccard intersection of two vectors `a` and `b`.
/// The Jaccard intersection is the sum of the minimum values of corresponding elements.
///
/// # Arguments
/// * `a` - The first vector.
/// * `b` - The second vector.
///
/// # Returns
/// The Jaccard intersection of `a` and `b`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let b = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
/// let intersection = intspan::jaccard_intersection(&a, &b);
/// assert_eq!(intersection, 30.0);
/// ```
pub fn jaccard_intersection(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| f32::min(*x, *y)).sum()
}

/// Computes the Jaccard union of two vectors `a` and `b`.
/// The Jaccard union is the sum of the maximum values of corresponding elements.
///
/// # Arguments
/// * `a` - The first vector.
/// * `b` - The second vector.
///
/// # Returns
/// The Jaccard union of `a` and `b`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let b = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
/// let union = intspan::jaccard_union(&a, &b);
/// assert_eq!(union, 80.0);
/// ```
pub fn jaccard_union(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| f32::max(*x, *y)).sum()
}

pub fn weighted_jaccard_similarity(a: &[f32], b: &[f32]) -> f32 {
    let numerator = jaccard_intersection(a, b);
    let denominator = jaccard_union(a, b);

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

/// Computes the dot product of two vectors `a` and `b`.
///
/// # Arguments
/// * `a` - The first vector.
/// * `b` - The second vector.
///
/// # Returns
/// The dot product of `a` and `b`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let b = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
/// let dot = intspan::dot_product(&a, &b);
/// assert_eq!(dot, 220.0);
/// ```
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Computes the L2 norm (Euclidean norm) of a vector `a`.
///
/// # Arguments
/// * `a` - The vector.
///
/// # Returns
/// The L2 norm of `a`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let norm = intspan::norm_l2(&a);
/// assert_eq!(format!("{:.4}", norm), "19.6214".to_string());
/// ```
#[inline]
pub fn norm_l2(a: &[f32]) -> f32 {
    norm_l2_sq(a).sqrt()
}

/// Computes the squared L2 norm of a vector `a`.
///
/// # Arguments
/// * `a` - The vector.
///
/// # Returns
/// The squared L2 norm of `a`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let norm_sq = intspan::norm_l2_sq(&a);
/// assert_eq!(norm_sq, 385.0);
/// ```
pub fn norm_l2_sq(a: &[f32]) -> f32 {
    a.iter().map(|x| x * x).sum()
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product = dot_product(a, b);
    let denominator = norm_l2(a) * norm_l2(b);

    if denominator == 0.0 {
        0.0
    } else {
        dot_product / denominator
    }
}

/// Computes the Euclidean distance between two vectors `a` and `b`.
///
/// # Arguments
/// * `a` - The first vector.
/// * `b` - The second vector.
///
/// # Returns
/// The Euclidean distance between `a` and `b`.
///
/// # Examples
/// ```
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let b = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
/// let distance = intspan::euclidean_distance(&a, &b);
/// assert_eq!(format!("{:.4}", distance), "18.1659".to_string());
/// ```
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let diff = x - y;
            diff * diff
        })
        .sum::<f32>()
        .sqrt()
}
