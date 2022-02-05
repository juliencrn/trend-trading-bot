/// Calculate the Exponential Moving Average (EMA)
pub fn ema(data_set: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    if window_size > data_set.len() {
        return None;
    }

    let mut result: Vec<f64> = Vec::new();
    let weighted_multiplier = 2.0 / (window_size as f64 + 1.0);

    // Calc the first using SMA
    let first_slice = &data_set[0..window_size];
    let first_sma: f64 = first_slice.iter().sum::<f64>() / window_size as f64;
    result.push(first_sma);

    // Then calc the next using EMA
    for i in window_size..data_set.len() {
        let previous_ema = result[result.len() - 1];
        let ema: f64 =
            (data_set[i] * weighted_multiplier) + previous_ema * (1.0 - weighted_multiplier);
        result.push(ema);
    }

    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exponential_moving_average() {
        let data_set = vec![5.0, 6.0, 4.0, 2.0];

        let result = ema(&data_set, 2).unwrap();
        assert_eq!(3, result.len());
        assert_eq!(vec![5.5, 4.5, 2.8333333333333335], result);

        let result = ema(&data_set, 4).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(vec![4.25], result);

        let result = ema(&data_set, 5);
        assert_eq!(None, result);

        let data_set = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 22.15, 22.39,
        ];
        let result = ema(&data_set, 10).unwrap();
        assert_eq!(3, result.len());
        assert_eq!(
            vec![22.220999999999997, 22.208090909090906, 22.241165289256195],
            result
        );
    }
}
