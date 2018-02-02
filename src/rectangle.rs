
pub struct Rectangle {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64
}


impl Rectangle {
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Rectangle {
        Rectangle { left, right, top, bottom }
    }
}
