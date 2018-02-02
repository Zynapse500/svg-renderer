
pub struct Rectangle {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64
}


impl Rectangle {
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Rectangle {
        Rectangle { left, right, top, bottom }
    }
}
