use std::io;

use std::io::BufWriter;
use std::io::Write;

use std::io::Read;

use png::Encoder;

pub fn writer2padded_writer_new<W>(
    writer: W,
) -> impl FnOnce(&[u8], u16, u16) -> Result<(), io::Error>
where
    W: Write,
{
    move |data: &[u8], width: u16, height: u16| {
        let bw = BufWriter::new(writer);
        let mut enc = Encoder::new(bw, width.into(), height.into());
        enc.set_color(png::ColorType::Grayscale);
        enc.set_depth(png::BitDepth::Eight);
        let mut wtr = enc.write_header()?;
        wtr.write_image_data(data)?;
        wtr.finish()?;
        Ok(())
    }
}

pub fn writer2nopad_dummy_writer_new<W>(
    _: W,
) -> impl FnOnce(&[u8], u16, u16) -> Result<(), io::Error> {
    move |_: &[u8], _: u16, _: u16| Err(io::Error::other("padded data only"))
}

pub fn reader2png2data<R>(rdr: R, buf: &mut Vec<u8>) -> Result<(), io::Error>
where
    R: Read,
{
    let decoder = png::Decoder::new(rdr);
    let mut prdr = decoder.read_info()?;
    let bufsz: usize = prdr.output_buffer_size();
    buf.resize(bufsz, 0);
    prdr.next_frame(buf)?;
    Ok(())
}

pub fn reader2png2data2writer<R, W>(
    reader: R,
    buf: &mut Vec<u8>,
    mut writer: W,
) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    reader2png2data(reader, buf)?;
    writer.write_all(buf)?;
    Ok(())
}
