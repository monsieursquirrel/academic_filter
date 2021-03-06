extern crate num;

use num::Complex;
use std::f32;

pub struct FilterTwoPole {
    // use circular buffers for previous sample data
    prev_in: [f32; 2],
    last_write_in: usize,
    prev_out: [f32; 2],
    last_write_out: usize,

    // coefficients
    in_coeffs: [f32; 3],
    out_coeffs: [f32; 2]
}

impl FilterTwoPole {
    pub fn new(sample_rate: u32) -> FilterTwoPole {
        FilterTwoPole {
            prev_in: [0.0; 2],
            last_write_in: 0,
            prev_out: [0.0; 2],
            last_write_out: 0,
            in_coeffs: [0.0012074046354035072, 0.0024148092708070144, 0.0012074046354035072],
            out_coeffs: [1.8993325472756315, -0.9041621658172454]
        }
    }

    pub fn lowpass(cutoff_freq: f32, sample_rate: u32) -> FilterTwoPole {
        // zeros are both at -1
        let z = [Complex::<f32>::new(-1.0, 0.0); 2];

        // poles are a pair with angle +/- pi*Fc/Fs
        // the magnitude is set at 0.5 for now
        let pole_angle = (f32::consts::PI * cutoff_freq) / (sample_rate as f32);
        let pole_mag = 0.5;
        let p = [Complex::<f32>::from_polar(&pole_mag, &pole_angle),
                Complex::<f32>::from_polar(&pole_mag, &-pole_angle)];

        FilterTwoPole {
            prev_in: [0.0; 2],
            last_write_in: 0,
            prev_out: [0.0; 2],
            last_write_out: 0,
            in_coeffs: [1.0, -(z[0] + z[1]).norm(), (z[0] * z[1]).norm()],
            out_coeffs: [-(p[0] + p[1]).norm(), (p[0] * p[1]).norm()]
        }
    }

    pub fn process_sample(&mut self, sample: f32) -> f32 {
        let in_scaled = sample * self.in_coeffs[0];

        // skip first coefficient here as it applies to the current input sample
        let prev_in_sum = self.in_coeffs.iter().skip(1)
            .zip(self.prev_in.iter().skip(self.last_write_in).cycle())
            .fold(0.0, |sum, (coeff, sample)| {
                sum + (coeff * sample)
            });

        let prev_out_sum = self.out_coeffs.iter()
            .zip(self.prev_out.iter().skip(self.last_write_out).cycle())
            .fold(0.0, |sum, (coeff, sample)| {
                sum + (coeff * sample)
            });

        let out_sample = in_scaled + prev_in_sum + prev_out_sum;

        // update buffers
        self.last_write_in = (self.last_write_in + 1) % self.prev_in.len();
        self.prev_in[self.last_write_in] = sample;

        self.last_write_out = (self.last_write_out + 1) % self.prev_out.len();
        self.prev_out[self.last_write_out] = out_sample;

        out_sample
    }
}
