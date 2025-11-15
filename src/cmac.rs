#[derive(Debug, Clone)]
pub struct CMAC {
    arr: Vec<f64>,
    genp: usize,
    weight: Vec<f64>,
}

impl CMAC {
    pub fn new(arr: Vec<f64>, generalization_parameter: usize) -> Self {
        let genp = generalization_parameter;
        let memory = arr.len() + genp - 1;

        let mut sum = 0.0;
        let mut count = 0usize;
        for v in &arr {
            if !v.is_nan() {
                sum += *v;
                count += 1;
            }
        }

        let mean = if count > 0 { sum / count as f64 } else { 0.0 };
        let init_weight = mean / genp as f64;
        let weight = vec![init_weight; memory];

        Self {
            arr,
            genp,
            memory,
            weight,
        }
    }

    pub fn predict(&self, coord: usize) -> f64 {
        self.weight[coord..coord + self.genp].iter().sum()
    }

    pub fn train(
        &mut self,
        inputs: &[usize],
        targets: &[f64],
        learning_rate: f64,
        epochs: usize,
    ) {
        assert_eq!(inputs.len(), targets.len());

        for _ in 0..epochs {
            for (i, coord) in inputs.iter().enumerate() {
                let prediction = self.predict(*coord);
                let error = targets[i] - prediction;
                let delta = learning_rate * error / self.genp as f64;

                let start = *coord;
                let end = start + self.genp;

                for idx in start..end {
                    self.weight[idx] += delta;
                }
            }
        }
    }

    pub fn full_row(&mut self, learning_rate: f64, epochs: usize) -> Vec<f64> {
        let mut inputs: Vec<usize> = Vec::new();
        let mut targets: Vec<f64> = Vec::new();

        for (i, v) in self.arr.iter().enumerate() {
            if !v.is_nan() {
                inputs.push(i);
                targets.push(*v);
            }
        }

        self.train(&inputs, &targets, learning_rate, epochs);

        let mut new_arr = self.arr.clone();
        for (i, v) in new_arr.iter_mut().enumerate() {
            if v.is_nan() {
                *v = self.predict(i);
            }
        }

        new_arr
    }
}