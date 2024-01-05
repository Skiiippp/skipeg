use core::cmp::min;

use crate::defs::*;

fn pixel_rgb_to_ycbcr(rgb_val: &RGBVal) -> YCbCrVal {
    YCbCrVal {
        luminance: (0.299 * (rgb_val.red as f32) + 0.587 * (rgb_val.green as f32) + 0.114 * (rgb_val.blue as f32)).round() as u8,
        c_blue: (-0.1687 * (rgb_val.red as f32) - 0.3313 * (rgb_val.green as f32) + 0.5 * (rgb_val.blue as f32) + 128.0).round() as u8,
        c_red: (0.5 * (rgb_val.red as f32) - 0.4187 * (rgb_val.green as f32) - 0.0813 * (rgb_val.blue as f32) + 128.0).round() as u8,
    }
}

pub fn rgb_to_ycbr(rgb_vec: &mut Vec<u8>, ycbcr_vec: &mut Vec<YCbCrVal>) {
    let mut rgb_val = RGBVal {
        red: 0, green: 0, blue: 0,
    };
    let rgb_size: usize = rgb_vec.len();
    let mut i: usize = 0;

    ycbcr_vec.clear();
    while i < rgb_size {
        rgb_val.red = rgb_vec[i];
        rgb_val.green = rgb_vec[i+1];
        rgb_val.blue = rgb_vec[i+2];
        ycbcr_vec.push(pixel_rgb_to_ycbcr(&rgb_val));
        i += 3;
    }

}

fn pixel_ycbcr_to_rgb(ycbcr_val: &YCbCrVal) -> RGBVal {
    let f_luminance = ycbcr_val.luminance as f32;
    let f_c_blue = ycbcr_val.c_blue as f32;
    let f_c_red = ycbcr_val.c_red as f32;

    RGBVal {
        red: (f_luminance + 1.402 * (f_c_red - 128.0)).round() as u8,
        green: (f_luminance - 0.34414 * (f_c_blue - 128.0) - 0.71414 * (f_c_red - 128.0)).round() as u8,
        blue: (f_luminance + 1.772 * (f_c_blue - 128.0)).round() as u8, 
    }
}

pub fn ycbcr_to_rgb(rgb_vec: &mut Vec<u8>, ycbcr_vec: &mut Vec<YCbCrVal>) {
    let mut rgb_val: RGBVal;
    
    rgb_vec.clear();
    for ycbcr_val in ycbcr_vec {
        rgb_val = pixel_ycbcr_to_rgb(&ycbcr_val);
        rgb_vec.push(rgb_val.red);
        rgb_vec.push(rgb_val.green);
        rgb_vec.push(rgb_val.blue);
    }
}

// Result vec should be difference between vec1 and vec2 - difference multiplied by diff_factor
// Note - difference has a ceiling of 255, b/c 24-bit color
pub fn get_difference(vec1: &mut Vec<u8>, vec2: &mut Vec<u8>, res_vec: &mut Vec<u8>, diff_factor: u32) {
    assert!(vec1.len() == vec2.len());
    let mut i: usize = 0;

    res_vec.clear();
    while i < vec1.len() {
        let b_v1 = vec1[i] as i16;
        let b_v2 = vec2[i] as i16;
        let res = (b_v1 - b_v2).abs() as u8;
        res_vec.push((min::<u32>(diff_factor * res as u32, 255)) as u8);
        i += 1;
    }
}

// Converts unsigned to signed
pub fn level_shift_in(unsigned_vec: &Vec<YCbCrVal>, signed_vec: &mut Vec<SYCbCrVal>) {
    let mut temp_signed = SYCbCrVal {
        s_luminance: 0,
        s_c_blue: 0,
        s_c_red: 0,
    };

    signed_vec.clear();
    for unsigned_val in unsigned_vec {
        temp_signed.s_luminance = (unsigned_val.luminance as i16 - 128) as i8;
        temp_signed.s_c_blue = (unsigned_val.c_blue as i16 - 128) as i8;
        temp_signed.s_c_red = (unsigned_val.c_red as i16 - 128) as i8;
        signed_vec.push(temp_signed);
    }
}


pub fn level_shift_out(unsigned_vec: &mut Vec<YCbCrVal>, signed_vec: &Vec<SYCbCrVal>) {
    let mut temp_unsigned = YCbCrVal {
        luminance: 0,
        c_blue: 0,
        c_red: 0,
    };

    unsigned_vec.clear();
    for signed_val in signed_vec {
        temp_unsigned.luminance = (signed_val.s_luminance as i16 + 128) as u8;
        temp_unsigned.c_blue = (signed_val.s_c_blue as i16 + 128) as u8;
        temp_unsigned.c_red = (signed_val.s_c_red as i16 + 128) as u8;
        unsigned_vec.push(temp_unsigned);
    }
}