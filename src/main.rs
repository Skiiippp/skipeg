mod ppm_proc;

fn main(){
    let mut berries_info = ppm_proc::PPMImgInfo {
        img_path: "images/berries.ppm".to_string(),
        img_header: String::new(),
        img_vec: Vec::new(),
        vec_size: 0,
    };

    match ppm_proc::read_ppm(&mut berries_info) {
        Ok(()) => {},
        Err(error) => panic!("Error reading image: {error}"),
    };

    ppm_proc::rgb_to_ycbr(&mut berries_info.img_vec);

    match ppm_proc::write_ppm(&mut berries_info) {
        Ok(()) => {},
        Err(error) => panic!("Error writing image: {error}"),
    };
}


