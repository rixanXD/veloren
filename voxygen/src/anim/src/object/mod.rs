use super::{FigureBoneData, Skeleton};
use vek::*;

#[derive(Clone)]
pub struct ObjectSkeleton;
pub struct SkeletonAttr;

impl ObjectSkeleton {
    #[allow(clippy::new_without_default)] // TODO: Pending review in #587
    pub fn new() -> Self { Self {} }
}

const SCALE: f32 = 1.0 / 11.0;

impl Skeleton for ObjectSkeleton {
    type Attr = SkeletonAttr;

    #[cfg(feature = "use-dyn-lib")]
    const COMPUTE_FN: &'static [u8] = b"object_compute_mats\0";

    fn bone_count(&self) -> usize { 15 }

    #[cfg_attr(feature = "be-dyn-lib", export_name = "object_compute_mats")]
    fn compute_matrices_inner(&self) -> ([FigureBoneData; 16], Vec3<f32>) {
        (
            [
                FigureBoneData::new(Mat4::scaling_3d(Vec3::broadcast(SCALE))),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
                FigureBoneData::new(vek::Mat4::identity()),
            ],
            Vec3::default(),
        )
    }

    fn interpolate(&mut self, _target: &Self, _dt: f32) {}
}
