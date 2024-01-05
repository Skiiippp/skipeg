mod ppm;
mod defs;
mod util;
mod subsample;

use subsample::*;
use ppm::*;
use defs::*;
use util::*;

fn main(){
    let mut img_info = PPMImgInfo {
        img_path: "images/berries.ppm".to_string(),
        img_header: String::new(),
        img_vec: Vec::new(),
        vec_size: 0,
        width: 0,
        height: 0,
    };
    let mut ycbcr_vec: Vec<YCbCrVal> = Vec::new();
    let mut signed_subsampled_ycbcr_vec: Vec<SYCbCrVal> = Vec::new();
    let mut out_ppm_vec: Vec<u8> = Vec::new();
    let mut diff_vec: Vec<u8> = Vec::new();

    match read_ppm(&mut img_info) {
        Ok(()) => {},
        Err(error) => panic!("Error reading image: {error}"),
    };

    // **Processing begin**
    // Encoding
    rgb_to_ycbr(&mut img_info.img_vec, &mut ycbcr_vec);
    subsample(&mut ycbcr_vec, img_info.width, img_info.height);
    level_shift_in(&ycbcr_vec, &mut signed_subsampled_ycbcr_vec);   // At this point ycbcr_vec is subsampled

    // Decoding
    level_shift_out(&mut ycbcr_vec, &signed_subsampled_ycbcr_vec);
    ycbcr_to_rgb(&mut out_ppm_vec, &mut ycbcr_vec);
    
    // Analysis
    get_difference(&mut img_info.img_vec, &mut out_ppm_vec, &mut diff_vec, 1);
    
    // Show subsampled image
    img_info.img_vec = out_ppm_vec;
    img_info.img_path = "images/subsampled.ppm".to_string();
    match write_ppm(&mut img_info) {
        Ok(()) => {},
        Err(error) => panic!("Error writing image: {error}"),
    };

    // Show diff image
    img_info.img_vec = diff_vec;
    img_info.img_path = "images/diff.ppm".to_string();
    match write_ppm(&mut img_info) {
        Ok(()) => {},
        Err(error) => panic!("Error writing image: {error}"),
    };
    

}


