mod ppm;
mod defs;
mod util;

use ppm::*;
use defs::*;
use util::*;

fn main(){
    let mut img_info = PPMImgInfo {
        img_path: "images/berries.ppm".to_string(),
        img_header: String::new(),
        img_vec: Vec::new(),
        vec_size: 0,
    };
    let mut ycbcr_vec: Vec<YCbCrVal> = Vec::new();
    let mut rgb_out_vec: Vec<u8> = Vec::new();
    let mut diff_vec: Vec<u8> = Vec::new();

    match read_ppm(&mut img_info) {
        Ok(()) => {},
        Err(error) => panic!("Error reading image: {error}"),
    };

    // Processing begin
    rgb_to_ycbr(&mut img_info.img_vec, &mut ycbcr_vec);
    ycbcr_to_rgb(&mut rgb_out_vec, &mut ycbcr_vec);
    get_difference(&mut img_info.img_vec, &mut rgb_out_vec, &mut diff_vec, 20);
    // Processing end
    
    match write_ppm(&mut img_info) {
        Ok(()) => {},
        Err(error) => panic!("Error writing image: {error}"),
    };

    img_info.img_vec = diff_vec;
    img_info.img_path = "images/berries_diff.ppm".to_string();

    match write_ppm(&mut img_info) {
        Ok(()) => {},
        Err(error) => panic!("Error writing image: {error}"),
    };
}


