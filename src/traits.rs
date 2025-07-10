use crate::objects::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::*, fmt::Display};

pub(crate) trait ChartJsRsObject {
    fn is_empty(&self) -> bool;
}
impl<T: Display> ChartJsRsObject for T {
    fn is_empty(&self) -> bool {
        self.to_string().is_empty()
    }
}

pub trait DatasetTrait: for<'a> Deserialize<'a> + Serialize + Default + Clone {
    fn labels(self) -> Vec<NumberOrDateString>;
}
pub trait DatasetDataExt {
    fn presorted_to_dataset_data(self) -> DatasetData;
    fn unsorted_to_dataset_data(self) -> DatasetData;
}

impl<I> DatasetDataExt for I
where
    I: Iterator<Item = (NumberOrDateString, NumberString, Option<Value>)>,
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
    ) -> impl Iterator<Item = (NumberOrDateString, NumberString, Option<Value>)>
    where
        Self: Iterator<Item = (X, Y)> + Sized,
        X: Into<NumberOrDateString>,
        Y: Into<NumberString>,
    {
        self.map(|(x, y)| (x.into(), y.into(), None))
    }
    fn into_data_iter_with_description<X, Y, D>(
        self,
    ) -> impl Iterator<Item = (NumberOrDateString, NumberString, Option<Value>)>
    where
        Self: Iterator<Item = (X, Y, D)> + Sized,
        X: Into<NumberOrDateString>,
        Y: Into<NumberString>,
        D: Serialize,
    {
        self.map(|(x, y, d)| (x.into(), y.into(), Some(serde_json::to_value(d).unwrap())))
    }
}
impl<T> DatasetIterExt for T where T: Iterator + ?Sized {}
