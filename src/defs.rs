pub struct RGBVal {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Copy, Clone)]
pub struct YCbCrVal {
    pub luminance: u8,
    pub c_blue: u8, 
    pub c_red: u8
}
#[derive(Copy, Clone)]
pub struct SYCbCrVal {
    pub s_luminance: i8,
    pub s_c_blue: i8,
    pub s_c_red: i8,
}

