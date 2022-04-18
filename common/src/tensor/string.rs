#[cfg(feature = "onnxruntime")]
use onnxruntime::{
    session::Session,
    tensor::{AsOrtTensorDyn, OrtTensorDyn},
};

use super::{AsTensorData, TensorData};
use crate::shape::{Dimensions, TensorType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StringTensorData {
    I64(ndarray::ArcArray<i64, ndarray::Ix2>),
    F32(ndarray::ArcArray<f32, ndarray::Ix2>),
}

impl From<StringTensorData> for TensorData {
    fn from(value: StringTensorData) -> Self {
        Self::String(value)
    }
}

#[cfg(feature = "onnxruntime")]
impl<'t> AsOrtTensorDyn<'t> for StringTensorData {
    fn as_ort_tensor_dyn<'m>(&self, session: &'m Session) -> onnxruntime::Result<OrtTensorDyn<'t>>
    where
        'm: 't,
    {
        match self {
            Self::I64(v) => v.as_ort_tensor_dyn(session),
            Self::F32(v) => v.as_ort_tensor_dyn(session),
        }
    }
}

impl AsTensorData for StringTensorData {
    fn ty(&self) -> TensorType {
        match self {
            Self::I64(_) => TensorType::I64,
            Self::F32(_) => TensorType::F32,
        }
    }

    fn dimensions(&self) -> Dimensions {
        fn dimensions_with_shape(shape: &[usize]) -> Dimensions {
            Dimensions::Image {
                channels: shape[1].try_into().unwrap(),
                width: Some(shape[2]),
                height: Some(shape[3]),
            }
        }

        match self {
            Self::I64(v) => dimensions_with_shape(v.shape()),
            Self::F32(v) => dimensions_with_shape(v.shape()),
        }
    }
}