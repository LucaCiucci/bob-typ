
use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn art_to_svg(art: &[u8]) -> Result<Vec<u8>, String> {
    let art = std::str::from_utf8(art).map_err(|op| format!("Invalid UTF-8: {}", op))?;
    let svg = svgbob::to_svg(art);
    Ok(svg.as_bytes().to_vec())
}