use std::process::ExitCode;

use std::io;

use std::io::Read;

use std::io::Write;

use rs_cbor2img::CBOR_EMPTY_MAP;

use rs_cbor2img::img::ImageSize;
use rs_cbor2img::img::bin2img;

use rs_cbor2img::png::reader2png2data2writer;
use rs_cbor2img::png::writer2nopad_dummy_writer_new;
use rs_cbor2img::png::writer2padded_writer_new;

fn padded_writer_png_stdout() -> impl FnOnce(&[u8], u16, u16) -> Result<(), io::Error> {
    writer2padded_writer_new(io::stdout().lock())
}

fn nopad_png_writer_invalid() -> impl FnOnce(&[u8], u16, u16) -> Result<(), io::Error> {
    writer2nopad_dummy_writer_new(&[0])
}

fn bin2stdout(bin: &[u8], width: u16, height: u16) -> Result<(), io::Error> {
    bin2img(
        bin,
        width,
        height,
        padded_writer_png_stdout(),
        nopad_png_writer_invalid(),
    )
}

fn bin2stdout_pad(
    mut bin: Vec<u8>,
    width: u16,
    height: u16,
    pad_byte: u8,
) -> Result<(), io::Error> {
    let original_size: usize = bin.len();
    let expected_size: usize = (width as usize) * (height as usize);
    if expected_size < original_size {
        return Err(io::Error::other(format!(
            "invalid image size. w={width}, h={height}"
        )));
    }

    let pad_size: usize = expected_size.saturating_sub(original_size);
    let repeated = std::iter::repeat(pad_byte);
    let taken = repeated.take(pad_size);
    bin.extend(taken);
    bin2stdout(&bin, width, height)
}

fn cbor2stdout_pad_empty_map(cbor: Vec<u8>, width: u16, height: u16) -> Result<(), io::Error> {
    bin2stdout_pad(cbor, width, height, CBOR_EMPTY_MAP)
}

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(|e| io::Error::other(format!("env var {key} missing: {e}")))
}

fn width_by_env() -> Option<u16> {
    env_val_by_key("ENV_WIDTH")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
}

fn height_by_env() -> Option<u16> {
    env_val_by_key("ENV_HEIGHT")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
}

fn size_by_env() -> Option<ImageSize> {
    width_by_env().and_then(|w: u16| {
        height_by_env().map(|h| ImageSize {
            width: w,
            height: h,
        })
    })
}

fn max_data_size_by_env() -> Result<u32, io::Error> {
    env_val_by_key("ENV_MAX_INPUT_SIZE")
        .map_err(io::Error::other)
        .and_then(|s| str::parse(s.as_str()).map_err(io::Error::other))
}

fn stdin2cbor2img2stdout() -> Result<(), io::Error> {
    let max: u64 = max_data_size_by_env()?.into();

    let i = io::stdin();
    let il = i.lock();
    let mut taken = il.take(max);
    let mut buf: Vec<u8> = Vec::new();
    taken.read_to_end(&mut buf)?;

    let sz: usize = buf.len();
    let isz: ImageSize = size_by_env().unwrap_or_else(|| sz.into());

    cbor2stdout_pad_empty_map(buf, isz.width, isz.height)?;

    Ok(())
}

fn stdin2png2data2stdout() -> Result<(), io::Error> {
    let max: u32 = max_data_size_by_env()?;

    let i = io::stdin();
    let il = i.lock();
    let taken = il.take(max.into());

    let o = io::stdout();
    let mut ol = o.lock();

    let mut buf: Vec<u8> = vec![];

    reader2png2data2writer(taken, &mut buf, &mut ol)?;

    ol.flush()
}

fn png2data() -> bool {
    env_val_by_key("ENV_DECODE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(false)
}

fn sub() -> Result<(), io::Error> {
    match png2data() {
        true => stdin2png2data2stdout(),
        false => stdin2cbor2img2stdout(),
    }
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
