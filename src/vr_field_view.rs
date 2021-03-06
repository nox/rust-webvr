// The VRFieldOfView interface represents a field of view, 
// as given by 4 degrees describing the view from a center point.

#[allow(unused_attributes)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VRFieldOfView {
    pub up_degrees: f64,
    pub right_degrees: f64,
    pub down_degrees: f64,
    pub left_degrees: f64,
}

impl Default for VRFieldOfView {
    fn default() -> VRFieldOfView {
        VRFieldOfView {
            up_degrees: 0.0,
            right_degrees: 0.0,
            down_degrees: 0.0,
            left_degrees: 0.0
        }
    }
}
