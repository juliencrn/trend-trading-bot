use crate::ema;

#[derive(PartialEq, Debug)]
pub struct MACD {
    pub macd: Vec<f64>,
    pub signal: Vec<f64>,
}

/// Calculate the Moving Average Convergence Divergence (MACD)
pub fn macd(
    data_set: &Vec<f64>,
    fast_length: usize,
    slow_length: usize,
    signal_length: usize,
) -> Option<MACD> {
    // Build macd main lines from ema (12 and 26 generally)
    let fast_ema_result = ema(data_set, fast_length);
    let slow_ema_result = ema(data_set, slow_length);
    let (fast_ema, slow_ema) = match (fast_ema_result, slow_ema_result) {
        (Some(fast_ema), Some(slow_ema)) => (fast_ema, slow_ema),
        _ => return None,
    };

    // Build macd line
    let mut macd: Vec<f64> = Vec::new();
    for i in 0..slow_ema.len() {
        let macd_val = fast_ema[(fast_ema.len() - slow_ema.len()) + i] - slow_ema[i];
        macd.push(macd_val);
    }

    // Get signal (generate ema(9))
    let signal_result = ema(&macd, signal_length);
    let signal = match signal_result {
        Some(signal) => signal,
        _ => return None,
    };

    Some(MACD { macd, signal })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_moving_average_convergence_divergence() {
        let data_set = vec![
            5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
        ];

        let result = macd(&data_set, 12, 26, 9);
        assert_eq!(None, result);

        let result = macd(&data_set, 3, 6, 2).unwrap();
        assert_eq!(8, result.macd.len());
        assert_eq!(
            vec![
                -1.5,
                -1.0178571428571432,
                -0.48596938775510257,
                -0.1194424198250732,
                0.02852327155351908,
                0.18443626539537084,
                0.32091429671097904,
                0.4309544083649852
            ],
            result.macd
        );
        assert_eq!(7, result.signal.len());
        assert_eq!(
            vec![
                -1.2589285714285716,
                -0.7436224489795923,
                -0.32750242954324627,
                -0.09015196214540272,
                0.09290685621511298,
                0.24491181654569036,
                0.36894021109188696
            ],
            result.signal
        );
    }
}
