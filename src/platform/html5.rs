
///
/// Abstract renderer for html canvas environments
/// 

use web_sys::HtmlCanvasElement;



pub trait FromCanvas: Sized {

    fn from_canvas(canvas: HtmlCanvasElement) -> Result<Self, crate::Error>;
}