use crate::defs::*;

struct YCbCrInfo<'a>{
    ycbcr_vec: &'a mut Vec<YCbCrVal>,
    width: u16,
    height: u16,
}

fn get_pixel(ycbcr_info: &mut YCbCrInfo, x_cord: u16, y_cord: u16) -> YCbCrVal {
    let mut def_val = YCbCrVal {
        luminance: 0,
        c_blue: 0,
        c_red: 0,
    };
    let index: usize = x_cord as usize + (y_cord as usize * ycbcr_info.width as usize) as usize;

    def_val.luminance = ycbcr_info.ycbcr_vec[index].luminance;
    def_val.c_blue = ycbcr_info.ycbcr_vec[index].c_blue;
    def_val.c_red = ycbcr_info.ycbcr_vec[index].c_red;
    
    return def_val;
}

// Don't care about luminance of ycbcr_val
fn set_pixel(ycbcr_val: &YCbCrVal, ycbcr_info: &mut YCbCrInfo, x_cord: u16, y_cord: u16) {
    let index: usize = x_cord as usize + y_cord as usize * ycbcr_info.width as usize;

    if x_cord < ycbcr_info.width && y_cord < ycbcr_info.height{
        ycbcr_info.ycbcr_vec[index].c_blue = ycbcr_val.c_blue;
        ycbcr_info.ycbcr_vec[index].c_red = ycbcr_val.c_red;
    } 
}

// Subsamples surrounding 4 pixels, with x,y being top left of the 4
fn pixel_subsample(ycbcr_info: &mut YCbCrInfo, x_cord: u16, y_cord: u16) {
    let mut ycbcr_val = get_pixel(ycbcr_info, x_cord, y_cord);
    let mut blue: u16 = ycbcr_val.c_blue as u16;
    let mut red: u16 = ycbcr_val.c_red as u16;
    let mut avg_divisior: u8 = 1;


    if x_cord + 1 < ycbcr_info.width {
        ycbcr_val = get_pixel(ycbcr_info, x_cord + 1, y_cord);
        blue += ycbcr_val.c_blue as u16;
        red += ycbcr_val.c_red as u16;
        avg_divisior += 1;
    }

    if y_cord + 1< ycbcr_info.height {
        ycbcr_val = get_pixel(ycbcr_info, x_cord, y_cord + 1);
        blue += ycbcr_val.c_blue as u16;
        red += ycbcr_val.c_red as u16;
        avg_divisior += 1;
    }

    if x_cord + 1 < ycbcr_info.width && y_cord + 1< ycbcr_info.height {
        ycbcr_val = get_pixel(ycbcr_info, x_cord + 1, y_cord + 1);
        blue += ycbcr_val.c_blue as u16;
        red += ycbcr_val.c_red as u16;
        avg_divisior += 1;
    }

    ycbcr_val.c_blue = (((blue as f32)/avg_divisior as f32).round()) as u8;
    ycbcr_val.c_red = (((red as f32)/avg_divisior as f32).round()) as u8;

    set_pixel(&ycbcr_val, ycbcr_info, x_cord, y_cord);
    set_pixel(&ycbcr_val, ycbcr_info, x_cord + 1, y_cord);
    set_pixel(&ycbcr_val, ycbcr_info, x_cord, y_cord + 1);
    set_pixel(&ycbcr_val, ycbcr_info, x_cord + 1, y_cord + 1);
}

// 4:2:0 chroma subsampling -> will modify ycbcr_vec, so should be copied if we care about that
pub fn subsample(ycbcr_vec: &mut Vec<YCbCrVal>, width: u16, height: u16) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;   
    let mut ycbcr_info = YCbCrInfo {
        ycbcr_vec: ycbcr_vec,
        width: width,
        height: height,
    };

    while j < height {
        while i < width {
            pixel_subsample(&mut ycbcr_info, i, j);
            i += 2;
        }
        i = 0;
        j += 2;
    } 
}

