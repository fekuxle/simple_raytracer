#![allow(dead_code)]
mod models;

#[cfg(test)]
mod testing {
    use crate::models::NormalizedColor;

    #[test]
    fn test_color_conversion() {
        let normal_red = NormalizedColor::new(1.0, 0.0, 0.0);
        let rgb_red = normal_red.to_rgb();

        assert_eq!(rgb_red[0], 255u8);
        assert_eq!(rgb_red[1], 0u8);
        assert_eq!(rgb_red[2], 0u8);
    }
}
