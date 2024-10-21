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
