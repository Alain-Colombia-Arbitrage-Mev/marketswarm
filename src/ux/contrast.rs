//! WCAG 2.x contrast from resolved solid colors. No model is involved.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn hex(self) -> String {
        if self.a == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        }
    }
}

pub fn parse_hex(input: &str) -> Option<Rgba> {
    let s = input.trim().strip_prefix('#')?;
    let expand = |h: &str| u8::from_str_radix(h, 16).ok().map(|v| v * 17);
    let byte = |h: &str| u8::from_str_radix(h, 16).ok();
    match s.len() {
        3 | 4 => {
            let r = expand(&s[0..1])?;
            let g = expand(&s[1..2])?;
            let b = expand(&s[2..3])?;
            let a = if s.len() == 4 { expand(&s[3..4])? } else { 255 };
            Some(Rgba { r, g, b, a })
        }
        6 | 8 => {
            let r = byte(&s[0..2])?;
            let g = byte(&s[2..4])?;
            let b = byte(&s[4..6])?;
            let a = if s.len() == 8 { byte(&s[6..8])? } else { 255 };
            Some(Rgba { r, g, b, a })
        }
        _ => None,
    }
}

fn channel(c: u8) -> f64 {
    let s = f64::from(c) / 255.0;
    if s <= 0.03928 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    }
}

pub fn relative_luminance(color: Rgba) -> f64 {
    0.2126 * channel(color.r) + 0.7152 * channel(color.g) + 0.0722 * channel(color.b)
}

/// Composite `src` over an opaque `dst` in sRGB, then treat the result as opaque.
pub fn composite(src: Rgba, dst: Rgba) -> Rgba {
    let a = f64::from(src.a) / 255.0;
    let mix = |s: u8, d: u8| -> u8 {
        (f64::from(s) * a + f64::from(d) * (1.0 - a))
            .round()
            .clamp(0.0, 255.0) as u8
    };
    Rgba {
        r: mix(src.r, dst.r),
        g: mix(src.g, dst.g),
        b: mix(src.b, dst.b),
        a: 255,
    }
}

pub fn contrast_ratio(a: Rgba, b: Rgba) -> f64 {
    let l1 = relative_luminance(a);
    let l2 = relative_luminance(b);
    let (hi, lo) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (hi + 0.05) / (lo + 0.05)
}

pub fn is_large_text(font_size: Option<f64>, font_weight: Option<u32>) -> bool {
    match font_size {
        Some(size) if size >= 24.0 => true,
        Some(size) if size >= 18.66 && font_weight.unwrap_or(400) >= 700 => true,
        _ => false,
    }
}

pub fn aa_minimum(large: bool) -> f64 {
    if large {
        3.0
    } else {
        4.5
    }
}

pub fn passes_aa(ratio: f64, large: bool) -> bool {
    ratio + 1e-9 >= aa_minimum(large)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_on_white_is_twenty_one() {
        let black = parse_hex("#000000").unwrap();
        let white = parse_hex("#FFFFFF").unwrap();
        assert!((relative_luminance(black) - 0.0).abs() < 1e-12);
        assert!((relative_luminance(white) - 1.0).abs() < 1e-12);
        assert!((contrast_ratio(black, white) - 21.0).abs() < 1e-9);
        assert!(passes_aa(contrast_ratio(black, white), false));
    }

    #[test]
    fn known_gray_boundary_and_alpha() {
        let white = parse_hex("#FFFFFF").unwrap();
        let fail = parse_hex("#777777").unwrap();
        let pass = parse_hex("#767676").unwrap();
        assert!(!passes_aa(contrast_ratio(fail, white), false));
        assert!(passes_aa(contrast_ratio(pass, white), false));
        let translucent = parse_hex("#FFFFFF80").unwrap();
        let over_black = composite(translucent, parse_hex("#000000").unwrap());
        let ratio = contrast_ratio(over_black, parse_hex("#000000").unwrap());
        assert!(ratio < 21.0 && ratio > 4.0);
        assert_eq!(parse_hex("#abc").unwrap(), parse_hex("#AABBCC").unwrap());
    }

    #[test]
    fn large_text_threshold_matches_wcag_points() {
        assert!(is_large_text(Some(24.0), Some(400)));
        assert!(!is_large_text(Some(18.0), Some(700)));
        assert!(is_large_text(Some(18.66), Some(700)));
        assert!(!is_large_text(Some(18.66), Some(600)));
        assert_eq!(aa_minimum(true), 3.0);
        assert_eq!(aa_minimum(false), 4.5);
    }
}
