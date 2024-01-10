// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/tensor_data.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: A multi-dimensional `Tensor` of data.
///
/// The number of dimensions and their respective lengths is specified by the `shape` field.
/// The dimensions are ordered from outermost to innermost. For example, in the common case of
/// a 2D RGB Image, the shape would be `[height, width, channel]`.
///
/// These dimensions are combined with an index to look up values from the `buffer` field,
/// which stores a contiguous array of typed values.
#[derive(Clone, Debug, PartialEq)]
pub struct TensorData {
    pub shape: Vec<crate::datatypes::TensorDimension>,
    pub buffer: crate::datatypes::TensorBuffer,
}

impl ::re_types_core::SizeBytes for TensorData {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.shape.heap_size_bytes() + self.buffer.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::datatypes::TensorDimension>>::is_pod()
            && <crate::datatypes::TensorBuffer>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(TensorData);

impl ::re_types_core::Loggable for TensorData {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.TensorData".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "shape".to_owned(),
                data_type: DataType::List(Box::new(Field {
                    name: "item".to_owned(),
                    data_type: <crate::datatypes::TensorDimension>::arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                })),
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "buffer".to_owned(),
                data_type: <crate::datatypes::TensorBuffer>::arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
        ])
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::datatypes::TensorData>::arrow_datatype(),
                vec![
                    {
                        let (somes, shape): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { shape, .. } = &**datum;
                                    shape.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let shape_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let shape_inner_data: Vec<_> = shape
                                .iter()
                                .flatten()
                                .flatten()
                                .cloned()
                                .map(Some)
                                .collect();
                            let shape_inner_bitmap: Option<arrow2::bitmap::Bitmap> = None;
                            let offsets =
                                arrow2::offset::Offsets::<i32>::try_from_lengths(shape.iter().map(
                                    |opt| opt.as_ref().map(|datum| datum.len()).unwrap_or_default(),
                                ))
                                .unwrap()
                                .into();
                            ListArray::new(
                                DataType::List(Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: <crate::datatypes::TensorDimension>::arrow_datatype(
                                    ),
                                    is_nullable: false,
                                    metadata: [].into(),
                                })),
                                offsets,
                                {
                                    _ = shape_inner_bitmap;
                                    crate::datatypes::TensorDimension::to_arrow_opt(
                                        shape_inner_data,
                                    )?
                                },
                                shape_bitmap,
                            )
                            .boxed()
                        }
                    },
                    {
                        let (somes, buffer): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { buffer, .. } = &**datum;
                                    buffer.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let buffer_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = buffer_bitmap;
                            crate::datatypes::TensorBuffer::to_arrow_opt(buffer)?
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![
                            Field {
                                name: "shape".to_owned(),
                                data_type: DataType::List(Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: <crate::datatypes::TensorDimension>::arrow_datatype(
                                    ),
                                    is_nullable: false,
                                    metadata: [].into(),
                                })),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                            Field {
                                name: "buffer".to_owned(),
                                data_type: <crate::datatypes::TensorBuffer>::arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                        ]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.TensorData")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let shape = {
                    if !arrays_by_name.contains_key("shape") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "shape",
                        ))
                        .with_context("rerun.datatypes.TensorData");
                    }
                    let arrow_data = &**arrays_by_name["shape"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| {
                                DeserializationError::datatype_mismatch(
                                    DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type:
                                            <crate::datatypes::TensorDimension>::arrow_datatype(),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    arrow_data.data_type().clone(),
                                )
                            })
                            .with_context("rerun.datatypes.TensorData#shape")?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                crate::datatypes::TensorDimension::from_arrow_opt(arrow_data_inner)
                                    .with_context("rerun.datatypes.TensorData#shape")?
                                    .into_iter()
                                    .collect::<Vec<_>>()
                            };
                            let offsets = arrow_data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets.iter().zip(offsets.lengths()),
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, len)| {
                                    let start = *start as usize;
                                    let end = start + len;
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data
                                        .iter()
                                        .cloned()
                                        .map(Option::unwrap_or_default)
                                        .collect();
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                let buffer = {
                    if !arrays_by_name.contains_key("buffer") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "buffer",
                        ))
                        .with_context("rerun.datatypes.TensorData");
                    }
                    let arrow_data = &**arrays_by_name["buffer"];
                    crate::datatypes::TensorBuffer::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.TensorData#buffer")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(shape, buffer),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(shape, buffer)| {
                        Ok(Self {
                            shape: shape
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.TensorData#shape")?,
                            buffer: buffer
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.TensorData#buffer")?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.TensorData")?
            }
        })
    }
}
