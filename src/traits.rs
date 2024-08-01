use crate::types::*;
use serde::Serialize;
use std::collections::*;

pub trait DatasetTrait: Serialize {}
pub trait DatasetDataExt {
    fn presorted_to_dataset_data(self) -> DatasetData;
    fn unsorted_to_dataset_data(self) -> DatasetData;
}

impl<I> DatasetDataExt for I
where
    I: Iterator<Item = (NumberOrDateString, NumberString, Option<String>)>,
{
    fn presorted_to_dataset_data(self) -> DatasetData {
        DatasetData(serde_json::to_value(self.map(XYPoint::from).collect::<Vec<_>>()).unwrap())
    }
    fn unsorted_to_dataset_data(self) -> DatasetData {
        DatasetData(serde_json::to_value(self.map(XYPoint::from).collect::<BTreeSet<_>>()).unwrap())
    }
}

pub trait DatasetIterExt: Iterator {
    fn into_data_iter<X, Y>(
        self,
    ) -> impl Iterator<Item = (NumberOrDateString, NumberString, Option<String>)>
    where
        Self: Iterator<Item = (X, Y)> + Sized,
        X: Into<NumberOrDateString>,
        Y: Into<NumberString>,
    {
        self.map(|(x, y)| (x.into(), y.into(), None))
    }
    fn into_data_iter_with_description<X, Y, D>(
        self,
    ) -> impl Iterator<Item = (NumberOrDateString, NumberString, Option<String>)>
    where
        Self: Iterator<Item = (X, Y, D)> + Sized,
        X: Into<NumberOrDateString>,
        Y: Into<NumberString>,
        D: Into<String>,
    {
        self.map(|(x, y, d)| (x.into(), y.into(), Some(d.into())))
    }
}
impl<T> DatasetIterExt for T where T: Iterator + ?Sized {}

pub trait Annotation: Serialize {}
