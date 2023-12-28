
// Assumes 8-bit channels
struct RGBVal {
    red: u8,
    green: u8,
    blue: u8,
}

struct YCbCrVal {
    luminance: u8,
    c_blue: u8,
    c_red: u8
}

fn rgb_to_ycbcr(rgb_val: RGBVal) -> YCbCrVal {
    YCbCrVal {
        luminance: (0.299 * (rgb_val.red as f32) + 0.587 * (rgb_val.green as f32) + 0.114 * (rgb_val.blue as f32)) as u8,
        c_blue: (-0.1687 * (rgb_val.red as f32) - 0.3313 * (rgb_val.green as f32) + 0.5 * (rgb_val.blue as f32) + 128.0) as u8,
        c_red: (0.5 * (rgb_val.red as f32) - 0.4187 * (rgb_val.green as f32) - 0.0813 * (rgb_val.blue as f32) + 128.0) as u8,
    }
}

fn main() {
    let rgb_val = RGBVal {
        red: 123,
        green: 45,
        blue: 67,
    };
    let ycbcr_val = rgb_to_ycbcr(rgb_val);
    println!("Y: {}, Cb: {}, Cr: {}", ycbcr_val.luminance, ycbcr_val.c_blue, ycbcr_val.c_red);
}


