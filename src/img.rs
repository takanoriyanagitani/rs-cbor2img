use std::io;

pub fn bin2img<R, P>(
    bin: &[u8],
    width: u16,
    height: u16,
    write_padded: R,
    write_nopad: P,
) -> Result<(), io::Error>
where
    R: FnOnce(&[u8], u16, u16) -> Result<(), io::Error>,
    P: FnOnce(&[u8], u16, u16) -> Result<(), io::Error>,
{
    let sz: usize = bin.len();
    let wh: usize = (width as usize) * (height as usize);
    match sz == wh {
        true => write_padded(bin, width, height),
        false => write_nopad(bin, width, height),
    }
}

pub struct ImageSize {
    pub width: u16,
    pub height: u16,
}

/// u = rt*rt
/// rt - 1 < w <= rt
/// rt < w+1 <= rt+1
/// rt < h+1 <= rt+1
/// rt(w+1) < (h+1)(w+1)
/// rt*rt < rt(w+1)
/// u = rt*rt < rt(w+1) < (h+1)(w+1)
impl From<usize> for ImageSize {
    fn from(u: usize) -> Self {
        let d: f64 = u as f64;
        let rt: f64 = d.sqrt();
        let w: usize = rt as usize;
        let h: usize = rt as usize;
        let sz: usize = w * h;
        let add: usize = match sz < u {
            true => 1,
            false => 0,
        };
        let w: usize = w + add;
        let h: usize = h + add;
        Self {
            width: w as u16,
            height: h as u16,
        }
    }
}
