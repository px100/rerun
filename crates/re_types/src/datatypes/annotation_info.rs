// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/annotation_info.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
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

/// **Datatype**: Annotation info annotating a class id or key-point id.
///
/// Color and label will be used to annotate entities/keypoints which reference the id.
/// The id refers either to a class or key-point id
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AnnotationInfo {
    /// `ClassId` or `KeypointId` to which this annotation info belongs.
    pub id: u16,

    /// The label that will be shown in the UI.
    pub label: Option<crate::datatypes::Utf8>,

    /// The color that will be applied to the annotated entity.
    pub color: Option<crate::datatypes::Rgba32>,
}

impl ::re_types_core::SizeBytes for AnnotationInfo {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.id.heap_size_bytes() + self.label.heap_size_bytes() + self.color.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <u16>::is_pod()
            && <Option<crate::datatypes::Utf8>>::is_pod()
            && <Option<crate::datatypes::Rgba32>>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(AnnotationInfo);

impl ::re_types_core::Loggable for AnnotationInfo {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.AnnotationInfo".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new("id", DataType::UInt16, false),
            Field::new("label", <crate::datatypes::Utf8>::arrow_datatype(), true),
            Field::new("color", <crate::datatypes::Rgba32>::arrow_datatype(), true),
        ]))
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
                Self::arrow_datatype(),
                vec![
                    {
                        let (somes, id): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.id.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let id_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt16,
                            id.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            id_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, label): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum =
                                    datum.as_ref().map(|datum| datum.label.clone()).flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let label_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                                label.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.0.len()).unwrap_or_default()
                                }),
                            )?
                            .into();
                            let inner_data: arrow2::buffer::Buffer<u8> = label
                                .into_iter()
                                .flatten()
                                .flat_map(|datum| datum.0 .0)
                                .collect();

                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                            unsafe {
                                Utf8Array::<i32>::new_unchecked(
                                    DataType::Utf8,
                                    offsets,
                                    inner_data,
                                    label_bitmap,
                                )
                            }
                            .boxed()
                        }
                    },
                    {
                        let (somes, color): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum =
                                    datum.as_ref().map(|datum| datum.color.clone()).flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let color_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt32,
                            color
                                .into_iter()
                                .map(|datum| datum.map(|datum| datum.0).unwrap_or_default())
                                .collect(),
                            color_bitmap,
                        )
                        .boxed()
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
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.AnnotationInfo")?;
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
                let id = {
                    if !arrays_by_name.contains_key("id") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "id",
                        ))
                        .with_context("rerun.datatypes.AnnotationInfo");
                    }
                    let arrow_data = &**arrays_by_name["id"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt16Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt16;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.AnnotationInfo#id")?
                        .into_iter()
                        .map(|opt| opt.copied())
                };
                let label = {
                    if !arrays_by_name.contains_key("label") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "label",
                        ))
                        .with_context("rerun.datatypes.AnnotationInfo");
                    }
                    let arrow_data = &**arrays_by_name["label"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::Utf8Array<i32>>()
                            .ok_or_else(|| {
                                let expected = DataType::Utf8;
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context("rerun.datatypes.AnnotationInfo#label")?;
                        let arrow_data_buf = arrow_data.values();
                        let offsets = arrow_data.offsets();
                        arrow2::bitmap::utils::ZipValidity::new_with_validity(
                            offsets.iter().zip(offsets.lengths()),
                            arrow_data.validity(),
                        )
                        .map(|elem| {
                            elem.map(|(start, len)| {
                                let start = *start as usize;
                                let end = start + len;
                                if end as usize > arrow_data_buf.len() {
                                    return Err(DeserializationError::offset_slice_oob(
                                        (start, end),
                                        arrow_data_buf.len(),
                                    ));
                                }

                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                let data =
                                    unsafe { arrow_data_buf.clone().sliced_unchecked(start, len) };
                                Ok(data)
                            })
                            .transpose()
                        })
                        .map(|res_or_opt| {
                            res_or_opt.map(|res_or_opt| {
                                res_or_opt.map(|v| {
                                    crate::datatypes::Utf8(::re_types_core::ArrowString(v))
                                })
                            })
                        })
                        .collect::<DeserializationResult<Vec<Option<_>>>>()
                        .with_context("rerun.datatypes.AnnotationInfo#label")?
                        .into_iter()
                    }
                };
                let color = {
                    if !arrays_by_name.contains_key("color") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "color",
                        ))
                        .with_context("rerun.datatypes.AnnotationInfo");
                    }
                    let arrow_data = &**arrays_by_name["color"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt32Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt32;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.AnnotationInfo#color")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .map(|res_or_opt| res_or_opt.map(|v| crate::datatypes::Rgba32(v)))
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(id, label, color),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(id, label, color)| {
                        Ok(Self {
                            id: id
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.AnnotationInfo#id")?,
                            label,
                            color,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.AnnotationInfo")?
            }
        })
    }
}
