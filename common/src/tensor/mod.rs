pub mod class;
pub mod dynamic;
pub mod image;
pub mod string;

#[cfg(feature = "onnxruntime")]
use onnxruntime::{
    session::Session,
    tensor::{AsOrtTensorDyn, OrtTensorDyn},
};

use crate::shape::{Dimensions, Shape, TensorType};

pub trait ToTensor {
    fn to_tensor(&self, shape: &Shape) -> anyhow::Result<Tensor>;
}

impl ToTensor for Box<dyn ToTensor + Send + Sync> {
    fn to_tensor(&self, shape: &Shape) -> anyhow::Result<Tensor> {
        (**self).to_tensor(shape)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tensor<Data = TensorData> {
    pub name: String,
    pub data: Data,
}

#[cfg(feature = "onnxruntime")]
impl<'t> AsOrtTensorDyn<'t> for Tensor {
    fn as_ort_tensor_dyn<'m>(&self, session: &'m Session) -> onnxruntime::Result<OrtTensorDyn<'t>>
    where
        'm: 't,
    {
        self.data.as_ort_tensor_dyn(session)
    }
}

impl AsTensorData for Tensor {
    fn ty(&self) -> TensorType {
        self.data.ty()
    }

    fn dimensions(&self) -> Dimensions {
        self.data.dimensions()
    }
}

impl ToTensor for Tensor {
    fn to_tensor(&self, parent: &Shape) -> anyhow::Result<Self> {
        let child = self.shape();
        if parent.contains(&child) {
            Ok(self.clone())
        } else {
            bail!(
                "Shape mismatched: Expected {expected:?}, but Given {given:?}",
                expected = parent,
                given = child,
            )
        }
    }
}

impl Tensor {
    pub fn shape(&self) -> Shape {
        Shape {
            name: self.name.clone(),
            ty: self.data.ty(),
            dimensions: self.data.dimensions(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TensorData {
    Dynamic(self::dynamic::DynamicTensorData),
    Class(self::class::ClassTensorData),
    Image(self::image::ImageTensorData),
    String(self::string::StringTensorData),
}

#[cfg(feature = "onnxruntime")]
impl<'t> AsOrtTensorDyn<'t> for TensorData {
    fn as_ort_tensor_dyn<'m>(&self, session: &'m Session) -> onnxruntime::Result<OrtTensorDyn<'t>>
    where
        'm: 't,
    {
        match self {
            Self::Dynamic(v) => v.as_ort_tensor_dyn(session),
            Self::Class(v) => v.as_ort_tensor_dyn(session),
            Self::Image(v) => v.as_ort_tensor_dyn(session),
            Self::String(v) => v.as_ort_tensor_dyn(session),
        }
    }
}

impl AsTensorData for TensorData {
    fn ty(&self) -> TensorType {
        match self {
            Self::Dynamic(v) => v.ty(),
            Self::Class(v) => v.ty(),
            Self::Image(v) => v.ty(),
            Self::String(v) => v.ty(),
        }
    }

    fn dimensions(&self) -> Dimensions {
        match self {
            Self::Dynamic(v) => v.dimensions(),
            Self::Class(v) => v.dimensions(),
            Self::Image(v) => v.dimensions(),
            Self::String(v) => v.dimensions(),
        }
    }
}

pub trait AsTensorData {
    fn ty(&self) -> TensorType;

    fn dimensions(&self) -> Dimensions;
}
