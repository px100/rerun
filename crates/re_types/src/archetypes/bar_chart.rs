// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/bar_chart.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A Barchart.
///
/// The x values will be the indices of the array, and the bar heights will be the provided values.
///
/// ## Example
///
/// ```ignore
/// //! Create and log a bar chart
///
/// use rerun::{archetypes::BarChart, RecordingStreamBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (rec, storage) = RecordingStreamBuilder::new("rerun_example_bar_chart").memory()?;
///
///     rec.log(
///         "bar_chart",
///         &BarChart::new(vec![8_i64, 4, 0, 9, 1, 4, 1, 6, 9, 0]),
///     )?;
///
///     rerun::native_viewer::show(storage.take())?;
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct BarChart {
    /// The values. Should always be a rank-1 tensor.
    pub values: crate::components::TensorData,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.TensorData".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.BarChartIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.InstanceKey".into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.TensorData".into(),
            "rerun.components.BarChartIndicator".into(),
            "rerun.components.InstanceKey".into(),
        ]
    });

impl BarChart {
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`BarChart`] [`crate::Archetype`]
pub type BarChartIndicator = crate::GenericIndicatorComponent<BarChart>;

impl crate::Archetype for BarChart {
    type Indicator = BarChartIndicator;

    #[inline]
    fn name() -> crate::ArchetypeName {
        "rerun.archetypes.BarChart".into()
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        1
    }

    fn as_component_batches(&self) -> Vec<crate::MaybeOwnedComponentBatch<'_>> {
        [
            Some(Self::Indicator::batch(self.num_instances() as _).into()),
            Some((&self.values as &dyn crate::ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn try_to_arrow(
        &self,
    ) -> crate::SerializationResult<
        Vec<(::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    > {
        use crate::{Loggable as _, ResultExt as _};
        Ok([{
            Some({
                let array = <crate::components::TensorData>::try_to_arrow([&self.values]);
                array.map(|array| {
                    let datatype = ::arrow2::datatypes::DataType::Extension(
                        "rerun.components.TensorData".into(),
                        Box::new(array.data_type().clone()),
                        None,
                    );
                    (
                        ::arrow2::datatypes::Field::new("values", datatype, false),
                        array,
                    )
                })
            })
            .transpose()
            .with_context("rerun.archetypes.BarChart#values")?
        }]
        .into_iter()
        .flatten()
        .collect())
    }

    #[inline]
    fn try_from_arrow(
        arrow_data: impl IntoIterator<
            Item = (::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>),
        >,
    ) -> crate::DeserializationResult<Self> {
        use crate::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let values = {
            let array = arrays_by_name
                .get("values")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.archetypes.BarChart#values")?;
            <crate::components::TensorData>::try_from_arrow_opt(&**array)
                .with_context("rerun.archetypes.BarChart#values")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.archetypes.BarChart#values")?
        };
        Ok(Self { values })
    }
}

impl BarChart {
    pub fn new(values: impl Into<crate::components::TensorData>) -> Self {
        Self {
            values: values.into(),
        }
    }
}