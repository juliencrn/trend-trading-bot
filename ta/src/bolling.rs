use crate::sma;

#[derive(PartialEq, Debug)]
pub struct BollingerBands {
    pub upper_bound: Vec<f64>,
    pub middle_bound: Vec<f64>,
    pub lower_bound: Vec<f64>,
}

/// Calculate the Bollinger Bands
pub fn bolling(data_set: &Vec<f64>, window_size: usize, multiplier: f64) -> Option<BollingerBands> {
    let middle_bound_option = sma(&data_set, window_size);
    let middle_bound = match middle_bound_option {
        Some(data) => data,
        _ => return None,
    };

    let mut upper_bound: Vec<f64> = Vec::new();
    let mut lower_bound: Vec<f64> = Vec::new();

    for i in 0..middle_bound.len() {
        let slice = &data_set[i..window_size + i];
        let variance = slice
            .iter()
            .map(|val| {
                let diff = middle_bound[i] - val;
                diff * diff
            })
            .sum::<f64>()
            / (window_size as f64);

        let standard_deviation = variance.sqrt();

        upper_bound.push(middle_bound[i] + multiplier * standard_deviation);
        lower_bound.push(middle_bound[i] - multiplier * standard_deviation);
    }

    Some(BollingerBands {
        upper_bound,
        middle_bound,
        lower_bound,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bollinger_bands() {
        let data_set = vec![
            5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
        ];
        let result = bolling(&data_set, 20, 2.0);
        assert_eq!(None, result);

        let result = bolling(&data_set, 8, 2.0).unwrap();
        assert_eq!(6, result.middle_bound.len());
        assert_eq!(
            vec![3.0625, 2.875, 2.5625, 2.5625, 2.875, 3.3125],
            result.middle_bound
        );
        assert_eq!(6, result.upper_bound.len());
        assert_eq!(
            vec![
                6.395572906493346,
                5.906088913245535,
                4.589659342528357,
                4.589659342528357,
                5.206844763272204,
                5.758798223847616
            ],
            result.upper_bound
        );
        assert_eq!(6, result.lower_bound.len());
        assert_eq!(
            vec![
                -0.27057290649334576,
                -0.1560889132455352,
                0.535340657471643,
                0.535340657471643,
                0.5431552367277961,
                0.8662017761523844
            ],
            result.lower_bound
        );
    }
}
