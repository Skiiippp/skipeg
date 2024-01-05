use std::fs::File;
use std::io::{BufRead, BufReader, Read, BufWriter, Write};
use std::str::SplitWhitespace;
use std::usize;

pub struct PPMImgInfo {
    pub img_path: String,
    pub img_header: String,
    pub img_vec: Vec<u8>,
    pub vec_size: usize,    // Bytes - Div by 3 to get pixels
    pub width: u16,
    pub height: u16,
}

fn str_to_int(s: &String) -> u16 {
    let ret_val: u16 = match s.parse::<u16>() {
        Ok(value) => value,
        Err(error) => panic!("Error: {error}"),
    };
    return ret_val;
}

fn get_width_heigh(width: &mut u16, height: &mut u16, s_iter: &mut SplitWhitespace) {
    let mut dim_string;
    dim_string = match s_iter.next() {
        Some(value) => value.to_string(),
        None => panic!("Error with width_height iterator"),
    };
    *width = str_to_int(&dim_string);
    dim_string.clear();

    dim_string = match s_iter.next() {
        Some(value) => value.to_string(),
        None => panic!("Error with width_height iterator"),
    }; 
    *height = str_to_int(&dim_string);
}

fn populate_vec(img_vec: &mut Vec<u8>, img_reader: &mut BufReader<File>) {
    match img_reader.read_exact(img_vec) {
        Ok(()) => {},
        Err(error) => panic!("Error populating pixel buffer: {error}"),
    };
}

// Remember - rust defaults to "root" directory being root of project
// Assumes img is ppm
pub fn read_ppm(img_info: &mut PPMImgInfo) -> std::io::Result<()> {
    let img = File::open(img_info.img_path.as_str())?;
    let mut img_reader = BufReader::new(img);
    let mut info_str = String::new();
    let mut width: u16 = 0;
    let mut height: u16 = 0;
    let mut s_iter: SplitWhitespace;
    let vec_size_ptr = &mut img_info.vec_size;
    let img_vec_ptr = &mut img_info.img_vec;
    let img_header_ptr = &mut img_info.img_header; 
    
    img_reader.read_line(&mut info_str)?;
    img_header_ptr.push_str(info_str.as_str());
    info_str.pop();
    if info_str != "P6" {
        panic!("Error: Unsupported image type!");
    }

    info_str.clear();
    img_reader.read_line(&mut info_str)?;
    img_header_ptr.push_str(info_str.as_str());
    s_iter = info_str.split_whitespace();
    get_width_heigh(&mut width, &mut height, &mut s_iter);
    img_info.width = width;
    img_info.height = height;

    info_str.clear();
    img_reader.read_line(&mut info_str)?;
    img_header_ptr.push_str(info_str.as_str());
    info_str.pop();
    if info_str != "255" {
        println!("Line: {info_str}");
        panic!("Error: Unsuported color depth!");
    }
 
    *vec_size_ptr = (3 * width as u32 * height as u32) as usize;
    *img_vec_ptr = vec![0; *vec_size_ptr];
    populate_vec(&mut *img_vec_ptr, &mut img_reader);

    Ok(())
}

pub fn write_ppm(img_info: &mut PPMImgInfo) -> std::io::Result<()> {
    let img = File::create(img_info.img_path.as_str())?;
    let mut img_writer = BufWriter::new(img);
    img_writer.write_all(img_info.img_header.as_bytes())?;
    img_writer.write_all(&img_info.img_vec)?;

    Ok(())
}

