pub trait Mappable {
    fn map(&self, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32;
}

impl Mappable for f32 {
    fn map(&self, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        out_min + (out_max - out_min) * (self - in_min) / (in_max - in_min)
    }
}
