mod models;

#[cfg(test)]
mod testing {
    use crate::models::Vector3;

    #[test]
    fn test_color_conversion() {
        let normal_red = Vector3::new(1.0, 0.0, 0.0);
        let rgb_red = normal_red.to_rgb();

        assert_eq!(rgb_red[0], 255u8);
        assert_eq!(rgb_red[1], 0u8);
        assert_eq!(rgb_red[2], 0u8);
    }
}
