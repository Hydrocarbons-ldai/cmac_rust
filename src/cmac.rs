//! Core CMAC implementation used by the crate.

/// CMAC-based imputer for `f64` sequences that use `NaN` as placeholders for
/// missing observations.
///
/// # Examples
///
/// ```
/// use cmac_rust::Cmac;
/// use std::f64;
///
/// let mut cmac = Cmac::new(vec![1.0, f64::NAN, 1.0], 1);
/// let filled = cmac.impute(0.5, 4);
/// assert!(!filled[1].is_nan());
/// ```
#[derive(Debug, Clone)]
pub struct Cmac {
    arr: Vec<f64>,
    genp: usize,
    weight: Vec<f64>,
}

impl Cmac {
    /// Creates a new CMAC model using the provided data and generalization
    /// parameter.
    ///
    /// # Panics
    ///
    /// Panics if `generalization_parameter` equals zero.
    pub fn new(arr: Vec<f64>, generalization_parameter: usize) -> Self {
        assert!(
            generalization_parameter > 0,
            "generalization parameter must be positive"
        );

        let genp = generalization_parameter;
        let memory = arr.len() + genp - 1;

        let mut sum = 0.0;
        let mut count = 0usize;
        for value in &arr {
            if !value.is_nan() {
                sum += *value;
                count += 1;
            }
        }

        let mean = if count > 0 { sum / count as f64 } else { 0.0 };
        let init_weight = mean / genp as f64;
        let weight = vec![init_weight; memory];

        Self { arr, genp, weight }
    }

    /// Returns the current generalization parameter.
    #[inline]
    pub fn generalization_parameter(&self) -> usize {
        self.genp
    }

    /// Exposes an immutable view of the internal weight vector.
    #[inline]
    pub fn weights(&self) -> &[f64] {
        &self.weight
    }

    /// Computes the CMAC prediction for the supplied coordinate.
    ///
    /// # Panics
    ///
    /// Panics if `coord` lies outside `0..self.arr.len()`.
    pub fn predict(&self, coord: usize) -> f64 {
        assert!(
            coord < self.arr.len(),
            "coordinate {coord} is out of bounds"
        );
        let end = coord + self.genp;
        debug_assert!(end <= self.weight.len());
        self.weight[coord..end].iter().sum()
    }

    /// Trains the CMAC weights using the supplied coordinates, targets and
    /// hyper-parameters.
    ///
    /// The `learning_rate` controls how strongly each error term influences the
    /// receptive field weights, while `epochs` defines how many full passes over
    /// the provided samples are performed.
    ///
    /// # Panics
    ///
    /// Panics if `inputs` and `targets` differ in length.
    pub fn train(&mut self, inputs: &[usize], targets: &[f64], learning_rate: f64, epochs: usize) {
        assert_eq!(inputs.len(), targets.len());

        if inputs.is_empty() || epochs == 0 {
            return;
        }

        for _ in 0..epochs {
            for (i, coord) in inputs.iter().enumerate() {
                let prediction = self.predict(*coord);
                let error = targets[i] - prediction;
                let delta = learning_rate * error / self.genp as f64;

                let start = *coord;
                let end = start + self.genp;

                for weight in &mut self.weight[start..end] {
                    *weight += delta;
                }
            }
        }
    }

    /// Imputes missing (`NaN`) entries in the original sequence using the
    /// current CMAC configuration.
    ///
    /// The function returns a freshly computed vector with every missing value
    /// replaced by a CMAC prediction and also updates the internally stored
    /// sequence to match the returned data.
    #[must_use]
    pub fn impute(&mut self, learning_rate: f64, epochs: usize) -> Vec<f64> {
        let mut inputs: Vec<usize> = Vec::new();
        let mut targets: Vec<f64> = Vec::new();

        for (index, value) in self.arr.iter().enumerate() {
            if !value.is_nan() {
                inputs.push(index);
                targets.push(*value);
            }
        }

        self.train(&inputs, &targets, learning_rate, epochs);

        let mut new_arr = self.arr.clone();
        for (index, value) in new_arr.iter_mut().enumerate() {
            if value.is_nan() {
                *value = self.predict(index);
            }
        }

        self.arr.clone_from(&new_arr);

        new_arr
    }

    /// Retains the legacy API name for backwards compatibility.
    #[deprecated(note = "Use `impute` instead.")]
    pub fn full_row(&mut self, learning_rate: f64, epochs: usize) -> Vec<f64> {
        self.impute(learning_rate, epochs)
    }
}

#[cfg(test)]
mod tests {
    use super::Cmac;

    #[test]
    fn imputes_single_gap() {
        let arr = vec![1.0, f64::NAN, 1.0];
        let mut cmac = Cmac::new(arr, 1);
        let imputed = cmac.impute(0.5, 10);

        assert!(!imputed[1].is_nan());
        assert!((imputed[1] - 1.0).abs() < 1e-9);
    }
}
