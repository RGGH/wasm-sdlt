use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_sdlt(property_value: f64) -> f64 {
    if property_value <= 250000.0 {
        0.0
    } else if property_value <= 925000.0 {
        // 5% on the portion between £250,001 and £925,000
        (property_value - 250000.0) * 0.05
    } else if property_value <= 1500000.0 {
        // 5% on the portion between £250,001 and £925,000
        let portion1 = (925000.0 - 250000.0) * 0.05;
        // 10% on the portion between £925,001 and £1.5 million
        let portion2 = (property_value - 925000.0) * 0.10;
        portion1 + portion2
    } else {
        // 5% on the portion between £250,001 and £925,000
        let portion1 = (925000.0 - 250000.0) * 0.05;
        // 10% on the portion between £925,001 and £1.5 million
        let portion2 = (1500000.0 - 925000.0) * 0.10;
        // 12% on the portion above £1.5 million
        let portion3 = (property_value - 1500000.0) * 0.12;
        portion1 + portion2 + portion3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdlt_below_250k() {
        let property_value = 200000.0;
        let expected = 0.0;
        assert_eq!(calculate_sdlt(property_value), expected);
    }

    #[test]
    fn test_sdlt_at_250k() {
        let property_value = 250000.0;
        let expected = 0.0;
        assert_eq!(calculate_sdlt(property_value), expected);
    }

    #[test]
    fn test_sdlt_between_250k_and_925k() {
        let property_value = 500000.0;
        let expected = (500000.0 - 250000.0) * 0.05;
        assert_eq!(calculate_sdlt(property_value), expected);
    }

    #[test]
    fn test_sdlt_between_925k_and_1_5m() {
        let property_value = 1000000.0;
        let portion1 = (925000.0 - 250000.0) * 0.05;
        let portion2 = (1000000.0 - 925000.0) * 0.10;
        let expected = portion1 + portion2;
        assert_eq!(calculate_sdlt(property_value), expected);
    }

    #[test]
    fn test_sdlt_above_1_5m() {
        let property_value = 2000000.0;
        let portion1 = (925000.0 - 250000.0) * 0.05;
        let portion2 = (1500000.0 - 925000.0) * 0.10;
        let portion3 = (2000000.0 - 1500000.0) * 0.12;
        let expected = portion1 + portion2 + portion3;
        assert_eq!(calculate_sdlt(property_value), expected);
    }
}

