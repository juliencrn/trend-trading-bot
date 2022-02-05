/// Calculate the Relative Strength Index (RSI)
pub fn rsi(data_set: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    if window_size > data_set.len() {
        return None;
    }

    let mut result: Vec<f64> = Vec::new();

    let get_gain = |index: usize| (100.0 / data_set[index - 1]) * data_set[index] - 100.0;
    let calc_rsi = |avg_gain: f64, avg_loss: f64| 100.0 - 100.0 / (1.0 + avg_gain / avg_loss);

    // RSI Step one
    let mut gains_sum = 0.0;
    let mut loss_sum = 0.0;

    for i in 0..=window_size {
        let gain = if i == 0 { 0.0 } else { get_gain(i) };
        if gain >= 0.0 {
            gains_sum += gain;
        } else {
            loss_sum += gain.abs();
        }
    }

    let current_average_gain = gains_sum / window_size as f64;
    let current_average_loss = loss_sum / window_size as f64;

    let rsi_first_step = calc_rsi(current_average_gain, current_average_loss);
    result.push(rsi_first_step);

    let mut previous_average_gain = current_average_gain;
    let mut previous_average_loss = current_average_loss;

    // RSI next steps
    for i in (window_size + 1)..data_set.len() {
        let gain = get_gain(i);
        let (current_gain, current_loss) = if gain > 0.0 {
            (gain, 0.0)
        } else {
            (0.0, gain.abs())
        };

        let current_average_gain = (previous_average_gain * (window_size as f64 - 1.0)
            + current_gain)
            / window_size as f64;
        let current_average_loss = (previous_average_loss * (window_size as f64 - 1.0)
            + current_loss)
            / window_size as f64;

        previous_average_gain = current_average_gain;
        previous_average_loss = current_average_loss;

        let rsi = 100.0 - 100.0 / (1.0 + current_average_gain / current_average_loss);
        result.push(rsi);
    }

    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_relative_strength_index() {
        let data_set = vec![
            5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
        ];

        let result = rsi(&data_set, 14);
        assert_eq!(None, result);

        let result = rsi(&data_set, 8).unwrap();
        assert_eq!(5, result.len());
        assert_eq!(
            vec![
                56.852791878172596,
                56.852791878172596,
                59.17295654731064,
                61.256328819550575,
                63.16578540011347
            ],
            result
        );

        let data_set = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28, 46.00, 46.03,
        ];
        let result = rsi(&data_set, 14).unwrap();
        assert_eq!(3, result.len());
        assert_eq!(
            vec![70.53539393736207, 66.436571546019, 66.66146763681454],
            result
        );
    }
}
