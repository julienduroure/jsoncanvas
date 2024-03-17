use jsoncanvas::JsonCanvas;
use std::str::FromStr;

fn main() {

    let s: String = "{\"nodes\":[{\"id\":\"id7\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"background\":\"path/to/image.png\",\"type\":\"group\"},{\"id\":\"id5\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"#ff0000\",\"label\":\"Label\",\"type\":\"group\"},{\"id\":\"id2\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"red\",\"file\":\"dir/to/path/file.png\",\"type\":\"file\"},{\"id\":\"id4\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"red\",\"url\":\"https://www.google.com\",\"type\":\"link\"},{\"id\":\"id6\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"type\":\"group\"},{\"id\":\"id3\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"red\",\"file\":\"dir/to/path/file.png\",\"subpath\":\"#here\",\"type\":\"file\"},{\"id\":\"id8\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"background\":\"path/to/image.png\",\"backgroundStyle\":\"cover\",\"type\":\"group\"},{\"id\":\"id\",\"x\":0,\"y\":0,\"width\":100,\"height\":100,\"color\":\"red\",\"text\":\"Test\",\"type\":\"text\"}],\"edges\":[{\"id\":\"edge2\",\"fromNode\":\"id3\",\"toNode\":\"id4\",\"color\":\"cyan\",\"label\":\"edge label\",\"toSide\":\"left\",\"toEnd\":\"arrow\"},{\"id\":\"edge1\",\"fromNode\":\"id1\",\"toNode\":\"id2\",\"toSide\":\"left\",\"toEnd\":\"arrow\"}]}".to_string();
    let canvas = JsonCanvas::from_str(&s).unwrap();

    let _s = canvas.to_string();

}
