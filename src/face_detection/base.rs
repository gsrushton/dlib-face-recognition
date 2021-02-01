use super::location::FaceLocations;
use crate::matrix::ImageMatrix;

pub trait FaceDetectorTrait {
    /// Detect face rectangles from an image.
    fn face_locations(&mut self, image: &ImageMatrix) -> FaceLocations;
}
