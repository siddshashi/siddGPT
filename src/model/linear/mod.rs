// linear affine transformation
use ndarray::{Array1, Array2};

mod helpers;

pub struct Linear {
    weight: Array2<f32>,
    bias: Array1<f32>,
}

impl Linear {
    pub fn new(din: usize, dout: usize, is_ffn: bool) -> Linear {
        let weight: Array2<f32> = helpers::initialize_weights(din, dout, is_ffn);
        let bias: Array1<f32> = Array1::<f32>::zeros(dout);

        Linear { weight , bias }
    }

    // forward method, applies Wx + b
    
    // backprop method, applies gradient to change weight, bias with optimizer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dims() {
        let lin: Linear = Linear::new(2, 3, true);

        assert_eq!(lin.weight.shape(), &[2, 3]);
        assert_eq!(lin.bias.shape(), &[3]);
    }

    #[test]
    fn test_identical() {
        let lin1: Linear = Linear::new(10, 10, true);
        let lin2: Linear = Linear::new(10, 10, true);
        
        assert_ne!(lin1.weight, lin2.weight);
    }

    #[test]
    fn test_he() {
        let din = 100;
        let dout = 50;
        let lin: Linear = Linear::new(din, dout, true);

        let mean: f32 = lin.weight.iter().sum::<f32>() / lin.weight.len() as f32;

        let variance: f32 = lin.weight.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / lin.weight.len() as f32;
    
        let expected_variance = 2.0 / din as f32;
        
        assert!(mean.abs() < 0.01);
        assert!((variance - expected_variance).abs() < 0.01);
    }

    #[test]
    fn test_xavier() {
        let din = 100;
        let dout = 50;
        let lin: Linear = Linear::new(din, dout, false);

        let mean: f32 = lin.weight.iter().sum::<f32>() / lin.weight.len() as f32;

        let limit = (6.0 / (din + dout) as f32).sqrt();
        let expected_variance = limit.powi(2) / 3.0;

        let variance: f32 = lin.weight.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / lin.weight.len() as f32;

        assert!(mean.abs() < 0.01);
        assert!((variance - expected_variance).abs() < 0.01);
    }
}
