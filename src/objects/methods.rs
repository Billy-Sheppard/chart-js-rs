#![allow(clippy::default_constructed_unit_structs)]

use {
    super::{chart_objects::*, helper_objects::*},
    crate::{Annotation, FnWithArgs},
    std::collections::*,
};

impl SinglePointDataset {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_background_color(&mut self) -> &mut Vec<String> {
        &mut self.backgroundColor
    }
    pub fn background_color<T: Into<String>>(
        mut self,
        value: impl IntoIterator<Item = T>,
    ) -> SinglePointDataset {
        self.backgroundColor = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn get_bar_percentage(&mut self) -> &mut NumberString {
        &mut self.barPercentage
    }
    pub fn bar_percentage(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.barPercentage = value.into();
        self
    }

    pub fn get_bar_thickness(&mut self) -> &mut NumberString {
        &mut self.barThickness
    }
    pub fn bar_thickness(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.barThickness = value.into();
        self
    }

    pub fn get_base(&mut self) -> &mut NumberString {
        &mut self.base
    }
    pub fn base(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.base = value.into();
        self
    }

    pub fn get_border_color(&mut self) -> &mut String {
        &mut self.borderColor
    }
    pub fn border_color(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.borderColor = value.into();
        self
    }

    pub fn get_border_join_style(&mut self) -> &mut String {
        &mut self.borderJoinStyle
    }
    pub fn border_join_style(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.borderJoinStyle = value.into();
        self
    }

    pub fn get_border_radius(&mut self) -> &mut NumberString {
        &mut self.borderRadius
    }
    pub fn border_radius(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.borderRadius = value.into();
        self
    }

    pub fn get_border_skipped(&mut self) -> &mut String {
        &mut self.borderSkipped
    }
    pub fn border_skipped(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.borderSkipped = value.into();
        self
    }

    pub fn get_border_width(&mut self) -> &mut NumberString {
        &mut self.borderWidth
    }
    pub fn border_width(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.borderWidth = value.into();
        self
    }

    pub fn get_category_percentage(&mut self) -> &mut NumberString {
        &mut self.categoryPercentage
    }
    pub fn category_percentage(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.categoryPercentage = value.into();
        self
    }

    pub fn get_clip(&mut self) -> &mut NumberString {
        &mut self.clip
    }
    pub fn clip(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.clip = value.into();
        self
    }

    pub fn get_data(&mut self) -> &mut Vec<NumberString> {
        &mut self.data
    }
    pub fn data<T: Into<NumberString>>(
        mut self,
        value: impl IntoIterator<Item = T>,
    ) -> SinglePointDataset {
        self.data = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn get_datalabels(&mut self) -> &mut Option<DataLabels> {
        &mut self.datalabels
    }
    pub fn datalabels(mut self, value: impl Into<DataLabels>) -> SinglePointDataset {
        self.datalabels = Some(value.into());
        self
    }

    pub fn get_grouped(&mut self) -> &mut Option<bool> {
        &mut self.grouped
    }
    pub fn grouped(mut self, value: impl Into<bool>) -> SinglePointDataset {
        self.grouped = Some(value.into());
        self
    }

    pub fn get_hover_background_color(&mut self) -> &mut String {
        &mut self.hoverBackgroundColor
    }
    pub fn hover_background_color(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.hoverBackgroundColor = value.into();
        self
    }

    pub fn get_hover_border_color(&mut self) -> &mut String {
        &mut self.hoverBorderColor
    }
    pub fn hover_border_color(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.hoverBorderColor = value.into();
        self
    }

    pub fn get_hover_border_radius(&mut self) -> &mut NumberString {
        &mut self.hoverBorderRadius
    }
    pub fn hover_border_radius(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.hoverBorderRadius = value.into();
        self
    }

    pub fn get_hover_border_width(&mut self) -> &mut NumberString {
        &mut self.hoverBorderWidth
    }
    pub fn hover_border_width(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.hoverBorderWidth = value.into();
        self
    }

    pub fn get_index_axis(&mut self) -> &mut String {
        &mut self.indexAxis
    }
    pub fn index_axis(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.indexAxis = value.into();
        self
    }

    pub fn get_inflate_amount(&mut self) -> &mut NumberString {
        &mut self.inflateAmount
    }
    pub fn inflate_amount(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.inflateAmount = value.into();
        self
    }

    pub fn get_label(&mut self) -> &mut String {
        &mut self.label
    }
    pub fn label(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.label = value.into();
        self
    }

    pub fn get_max_bar_thickness(&mut self) -> &mut NumberString {
        &mut self.maxBarThickness
    }
    pub fn max_bar_thickness(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.maxBarThickness = value.into();
        self
    }

    pub fn get_min_bar_length(&mut self) -> &mut NumberString {
        &mut self.minBarLength
    }
    pub fn min_bar_length(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.minBarLength = value.into();
        self
    }

    pub fn get_order(&mut self) -> &mut NumberString {
        &mut self.order
    }
    pub fn order(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.order = value.into();
        self
    }

    pub fn get_point_background_color(&mut self) -> &mut String {
        &mut self.pointBackgroundColor
    }
    pub fn point_background_color(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.pointBackgroundColor = value.into();
        self
    }

    pub fn get_point_border_color(&mut self) -> &mut String {
        &mut self.pointBorderColor
    }
    pub fn point_border_color(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.pointBorderColor = value.into();
        self
    }

    pub fn get_point_border_width(&mut self) -> &mut NumberString {
        &mut self.pointBorderWidth
    }
    pub fn point_border_width(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.pointBorderWidth = value.into();
        self
    }

    pub fn get_point_hover_background_color(&mut self) -> &mut String {
        &mut self.pointHoverBackgroundColor
    }
    pub fn point_hover_background_color(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.pointHoverBackgroundColor = value.into();
        self
    }

    pub fn get_point_hover_border_width(&mut self) -> &mut NumberString {
        &mut self.pointHoverBorderWidth
    }
    pub fn point_hover_border_width(
        mut self,
        value: impl Into<NumberString>,
    ) -> SinglePointDataset {
        self.pointHoverBorderWidth = value.into();
        self
    }

    pub fn get_point_hover_radius(&mut self) -> &mut NumberOrDateString {
        &mut self.pointHoverRadius
    }
    pub fn point_hover_radius(
        mut self,
        value: impl Into<NumberOrDateString>,
    ) -> SinglePointDataset {
        self.pointHoverRadius = value.into();
        self
    }

    pub fn get_point_radius(&mut self) -> &mut NumberString {
        &mut self.pointRadius
    }
    pub fn point_radius(mut self, value: impl Into<NumberString>) -> SinglePointDataset {
        self.pointRadius = value.into();
        self
    }

    pub fn get_point_style(&mut self) -> &mut String {
        &mut self.pointStyle
    }
    pub fn point_style(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.pointStyle = value.into();
        self
    }

    pub fn get_r_type(&mut self) -> &mut String {
        &mut self.r#type
    }
    pub fn r_type(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.r#type = value.into();
        self
    }

    pub fn get_skip_null(&mut self) -> &mut Option<bool> {
        &mut self.skipNull
    }
    pub fn skip_null(mut self, value: impl Into<bool>) -> SinglePointDataset {
        self.skipNull = Some(value.into());
        self
    }

    pub fn get_stack(&mut self) -> &mut String {
        &mut self.stack
    }
    pub fn stack(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.stack = value.into();
        self
    }

    pub fn get_stepped(&mut self) -> &mut Option<bool> {
        &mut self.stepped
    }
    pub fn stepped(mut self, value: impl Into<bool>) -> SinglePointDataset {
        self.stepped = Some(value.into());
        self
    }

    pub fn get_x_axis_id(&mut self) -> &mut String {
        &mut self.xAxisID
    }
    pub fn x_axis_id(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.xAxisID = value.into();
        self
    }

    pub fn get_y_axis_id(&mut self) -> &mut String {
        &mut self.yAxisID
    }
    pub fn y_axis_id(mut self, value: impl Into<String>) -> SinglePointDataset {
        self.yAxisID = value.into();
        self
    }
}

impl XYDataset {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_background_color(&mut self) -> &mut FnWithArgsOrAny {
        &mut self.backgroundColor
    }
    pub fn background_color(mut self, value: impl Into<FnWithArgsOrAny>) -> XYDataset {
        self.backgroundColor = value.into();
        self
    }

    pub fn get_background_color_array(&mut self) -> &mut Vec<String> {
        &mut self.backgroundColorArray
    }
    pub fn background_color_array<T: Into<String>>(
        mut self,
        value: impl IntoIterator<Item = T>,
    ) -> XYDataset {
        self.backgroundColorArray = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn get_bar_percentage(&mut self) -> &mut NumberString {
        &mut self.barPercentage
    }
    pub fn bar_percentage(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.barPercentage = value.into();
        self
    }

    pub fn get_bar_thickness(&mut self) -> &mut NumberString {
        &mut self.barThickness
    }
    pub fn bar_thickness(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.barThickness = value.into();
        self
    }

    pub fn get_base(&mut self) -> &mut NumberString {
        &mut self.base
    }
    pub fn base(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.base = value.into();
        self
    }

    pub fn get_border_color(&mut self) -> &mut String {
        &mut self.borderColor
    }
    pub fn border_color(mut self, value: impl Into<String>) -> XYDataset {
        self.borderColor = value.into();
        self
    }

    pub fn get_border_dash(&mut self) -> &mut Vec<NumberString> {
        &mut self.borderDash
    }
    pub fn border_dash<T: Into<NumberString>>(
        mut self,
        value: impl IntoIterator<Item = T>,
    ) -> XYDataset {
        self.borderDash = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn get_border_join_style(&mut self) -> &mut String {
        &mut self.borderJoinStyle
    }
    pub fn border_join_style(mut self, value: impl Into<String>) -> XYDataset {
        self.borderJoinStyle = value.into();
        self
    }

    pub fn get_border_radius(&mut self) -> &mut NumberString {
        &mut self.borderRadius
    }
    pub fn border_radius(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.borderRadius = value.into();
        self
    }

    pub fn get_border_skipped(&mut self) -> &mut String {
        &mut self.borderSkipped
    }
    pub fn border_skipped(mut self, value: impl Into<String>) -> XYDataset {
        self.borderSkipped = value.into();
        self
    }

    pub fn get_border_width(&mut self) -> &mut Option<NumberStringOrT<Border>> {
        &mut self.borderWidth
    }
    pub fn border_width(mut self, value: impl Into<NumberStringOrT<Border>>) -> XYDataset {
        self.borderWidth = Some(value.into());
        self
    }

    pub fn get_category_label(&mut self) -> &mut String {
        &mut self.category_label
    }
    pub fn category_label(mut self, value: impl Into<String>) -> XYDataset {
        self.category_label = value.into();
        self
    }

    pub fn get_category_percentage(&mut self) -> &mut NumberString {
        &mut self.categoryPercentage
    }
    pub fn category_percentage(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.categoryPercentage = value.into();
        self
    }

    pub fn get_clip(&mut self) -> &mut NumberString {
        &mut self.clip
    }
    pub fn clip(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.clip = value.into();
        self
    }

    pub fn get_data(&mut self) -> &mut DatasetData {
        &mut self.data
    }
    pub fn data(mut self, value: impl Into<DatasetData>) -> XYDataset {
        self.data = value.into();
        self
    }

    pub fn get_datalabels(&mut self) -> &mut DataLabels {
        &mut self.datalabels
    }
    pub fn datalabels(mut self, value: impl Into<DataLabels>) -> XYDataset {
        self.datalabels = value.into();
        self
    }

    pub fn get_description(&mut self) -> &mut String {
        &mut self.description
    }
    pub fn description(mut self, value: impl Into<String>) -> XYDataset {
        self.description = value.into();
        self
    }

    pub fn get_fill(&mut self) -> &mut String {
        &mut self.fill
    }
    pub fn fill(mut self, value: impl Into<String>) -> XYDataset {
        self.fill = value.into();
        self
    }

    pub fn get_grouped(&mut self) -> &mut Option<bool> {
        &mut self.grouped
    }
    pub fn grouped(mut self, value: impl Into<bool>) -> XYDataset {
        self.grouped = Some(value.into());
        self
    }

    pub fn get_hit_radius(&mut self) -> &mut NumberString {
        &mut self.hitRadius
    }
    pub fn hit_radius(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.hitRadius = value.into();
        self
    }

    pub fn get_hover_background_color(&mut self) -> &mut String {
        &mut self.hoverBackgroundColor
    }
    pub fn hover_background_color(mut self, value: impl Into<String>) -> XYDataset {
        self.hoverBackgroundColor = value.into();
        self
    }

    pub fn get_hover_border_color(&mut self) -> &mut String {
        &mut self.hoverBorderColor
    }
    pub fn hover_border_color(mut self, value: impl Into<String>) -> XYDataset {
        self.hoverBorderColor = value.into();
        self
    }

    pub fn get_hover_border_radius(&mut self) -> &mut NumberString {
        &mut self.hoverBorderRadius
    }
    pub fn hover_border_radius(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.hoverBorderRadius = value.into();
        self
    }

    pub fn get_hover_border_width(&mut self) -> &mut NumberString {
        &mut self.hoverBorderWidth
    }
    pub fn hover_border_width(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.hoverBorderWidth = value.into();
        self
    }

    pub fn get_axis(&mut self) -> &mut String {
        &mut self.axis
    }
    pub fn axis(mut self, value: impl Into<String>) -> XYDataset {
        self.axis = value.into();
        self
    }

    pub fn get_inflate_amount(&mut self) -> &mut NumberString {
        &mut self.inflateAmount
    }
    pub fn inflate_amount(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.inflateAmount = value.into();
        self
    }

    pub fn get_label(&mut self) -> &mut String {
        &mut self.label
    }
    pub fn label(mut self, value: impl Into<String>) -> XYDataset {
        self.label = value.into();
        self
    }

    pub fn get_max_bar_thickness(&mut self) -> &mut NumberString {
        &mut self.maxBarThickness
    }
    pub fn max_bar_thickness(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.maxBarThickness = value.into();
        self
    }

    pub fn get_min_bar_length(&mut self) -> &mut NumberString {
        &mut self.minBarLength
    }
    pub fn min_bar_length(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.minBarLength = value.into();
        self
    }

    pub fn get_order(&mut self) -> &mut NumberString {
        &mut self.order
    }
    pub fn order(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.order = value.into();
        self
    }

    pub fn get_point_background_color(&mut self) -> &mut String {
        &mut self.pointBackgroundColor
    }
    pub fn point_background_color(mut self, value: impl Into<String>) -> XYDataset {
        self.pointBackgroundColor = value.into();
        self
    }

    pub fn get_point_border_color(&mut self) -> &mut String {
        &mut self.pointBorderColor
    }
    pub fn point_border_color(mut self, value: impl Into<String>) -> XYDataset {
        self.pointBorderColor = value.into();
        self
    }

    pub fn get_point_border_width(&mut self) -> &mut NumberString {
        &mut self.pointBorderWidth
    }
    pub fn point_border_width(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.pointBorderWidth = value.into();
        self
    }

    pub fn get_point_hit_radius(&mut self) -> &mut NumberString {
        &mut self.pointHitRadius
    }
    pub fn point_hit_radius(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.pointHitRadius = value.into();
        self
    }

    pub fn get_point_hover_background_color(&mut self) -> &mut String {
        &mut self.pointHoverBackgroundColor
    }
    pub fn point_hover_background_color(mut self, value: impl Into<String>) -> XYDataset {
        self.pointHoverBackgroundColor = value.into();
        self
    }

    pub fn get_point_hover_border_width(&mut self) -> &mut NumberString {
        &mut self.pointHoverBorderWidth
    }
    pub fn point_hover_border_width(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.pointHoverBorderWidth = value.into();
        self
    }

    pub fn get_point_hover_radius(&mut self) -> &mut NumberOrDateString {
        &mut self.pointHoverRadius
    }
    pub fn point_hover_radius(mut self, value: impl Into<NumberOrDateString>) -> XYDataset {
        self.pointHoverRadius = value.into();
        self
    }

    pub fn get_point_radius(&mut self) -> &mut NumberString {
        &mut self.pointRadius
    }
    pub fn point_radius(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.pointRadius = value.into();
        self
    }

    pub fn get_point_style(&mut self) -> &mut String {
        &mut self.pointStyle
    }
    pub fn point_style(mut self, value: impl Into<String>) -> XYDataset {
        self.pointStyle = value.into();
        self
    }

    pub fn get_segment(&mut self) -> &mut Option<Segment> {
        &mut self.segment
    }
    pub fn segment(mut self, value: impl Into<Segment>) -> XYDataset {
        self.segment = Some(value.into());
        self
    }

    pub fn get_skip_null(&mut self) -> &mut Option<bool> {
        &mut self.skipNull
    }
    pub fn skip_null(mut self, value: impl Into<bool>) -> XYDataset {
        self.skipNull = Some(value.into());
        self
    }

    pub fn get_span_gaps(&mut self) -> &mut Option<bool> {
        &mut self.spanGaps
    }
    pub fn span_gaps(mut self, value: impl Into<bool>) -> XYDataset {
        self.spanGaps = Some(value.into());
        self
    }

    pub fn get_stack(&mut self) -> &mut String {
        &mut self.stack
    }
    pub fn stack(mut self, value: impl Into<String>) -> XYDataset {
        self.stack = value.into();
        self
    }

    pub fn get_stepped(&mut self) -> &mut Option<BoolString> {
        &mut self.stepped
    }
    pub fn stepped(mut self, value: impl Into<BoolString>) -> XYDataset {
        self.stepped = Some(value.into());
        self
    }

    pub fn get_tension(&mut self) -> &mut NumberString {
        &mut self.tension
    }
    pub fn tension(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.tension = value.into();
        self
    }

    pub fn get_r_type(&mut self) -> &mut String {
        &mut self.r#type
    }
    pub fn r_type(mut self, value: impl Into<String>) -> XYDataset {
        self.r#type = value.into();
        self
    }

    pub fn get_x_axis_id(&mut self) -> &mut String {
        &mut self.xAxisID
    }
    pub fn x_axis_id(mut self, value: impl Into<String>) -> XYDataset {
        self.xAxisID = value.into();
        self
    }

    pub fn get_y_axis_id(&mut self) -> &mut String {
        &mut self.yAxisID
    }
    pub fn y_axis_id(mut self, value: impl Into<String>) -> XYDataset {
        self.yAxisID = value.into();
        self
    }

    pub fn get_z(&mut self) -> &mut NumberString {
        &mut self.z
    }
    pub fn z(mut self, value: impl Into<NumberString>) -> XYDataset {
        self.z = value.into();
        self
    }
}

impl XYPoint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_x(&mut self) -> &mut NumberOrDateString {
        &mut self.x
    }
    pub fn x(mut self, value: impl Into<NumberOrDateString>) -> XYPoint {
        self.x = value.into();
        self
    }

    pub fn get_y(&mut self) -> &mut NumberString {
        &mut self.y
    }
    pub fn y(mut self, value: impl Into<NumberString>) -> XYPoint {
        self.y = value.into();
        self
    }

    pub fn get_description(&mut self) -> &mut serde_json::Value {
        &mut self.description
    }
    pub fn description(mut self, value: impl Into<serde_json::Value>) -> XYPoint {
        self.description = value.into();
        self
    }
}

impl<A: Annotation> ChartOptions<A> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_animation(&mut self) -> &mut Option<Animation> {
        &mut self.animation
    }
    pub fn animation(mut self, value: impl Into<Animation>) -> ChartOptions<A> {
        self.animation = Some(value.into());
        self
    }

    pub fn get_elements(&mut self) -> &mut Option<ChartElements> {
        &mut self.elements
    }
    pub fn elements(mut self, value: impl Into<ChartElements>) -> ChartOptions<A> {
        self.elements = Some(value.into());
        self
    }

    pub fn get_interaction(&mut self) -> &mut Option<ChartInteraction> {
        &mut self.interaction
    }
    pub fn interaction(mut self, value: impl Into<ChartInteraction>) -> ChartOptions<A> {
        self.interaction = Some(value.into());
        self
    }

    pub fn get_index_axis(&mut self) -> &mut String {
        &mut self.indexAxis
    }
    pub fn index_axis(mut self, value: impl Into<String>) -> ChartOptions<A> {
        self.indexAxis = value.into();
        self
    }

    pub fn get_legend(&mut self) -> &mut Option<ChartLegend> {
        &mut self.legend
    }
    pub fn legend(mut self, value: impl Into<ChartLegend>) -> ChartOptions<A> {
        self.legend = Some(value.into());
        self
    }

    pub fn get_layout(&mut self) -> &mut Option<ChartLayout> {
        &mut self.layout
    }
    pub fn layout(mut self, value: impl Into<ChartLayout>) -> ChartOptions<A> {
        self.layout = Some(value.into());
        self
    }

    pub fn get_maintain_aspect_ratio(&mut self) -> &mut Option<bool> {
        &mut self.maintainAspectRatio
    }
    pub fn maintain_aspect_ratio(mut self, value: impl Into<bool>) -> ChartOptions<A> {
        self.maintainAspectRatio = Some(value.into());
        self
    }

    pub fn get_plugins(&mut self) -> &mut Option<ChartPlugins<A>> {
        &mut self.plugins
    }
    pub fn plugins(mut self, value: impl Into<ChartPlugins<A>>) -> ChartOptions<A> {
        self.plugins = Some(value.into());
        self
    }

    pub fn get_responsive(&mut self) -> &mut Option<bool> {
        &mut self.responsive
    }
    pub fn responsive(mut self, value: impl Into<bool>) -> ChartOptions<A> {
        self.responsive = Some(value.into());
        self
    }

    pub fn get_scales(&mut self) -> &mut Option<HashMap<String, ChartScale>> {
        &mut self.scales
    }
    pub fn scales<T: Into<String>, U: IntoIterator<Item = (T, ChartScale)>>(
        mut self,
        value: U,
    ) -> ChartOptions<A> {
        self.scales = Some(value.into_iter().map(|(k, v)| (k.into(), v)).collect());
        self
    }

    pub fn get_span_gaps(&mut self) -> &mut Option<bool> {
        &mut self.spanGaps
    }
    pub fn span_gaps(mut self, value: impl Into<bool>) -> ChartOptions<A> {
        self.spanGaps = Some(value.into());
        self
    }
}

impl Animation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_duration(&mut self) -> &mut NumberString {
        &mut self.duration
    }
    pub fn duration(mut self, value: impl Into<NumberString>) -> Animation {
        self.duration = value.into();
        self
    }
}

impl<A: Annotation> ChartPlugins<A> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_annotation(&mut self) -> &mut Option<Annotations<A>> {
        &mut self.annotation
    }
    pub fn annotation(mut self, value: impl Into<Annotations<A>>) -> ChartPlugins<A> {
        self.annotation = Some(value.into());
        self
    }

    pub fn get_autocolors(&mut self) -> &mut Option<AutoColors> {
        &mut self.autocolors
    }
    pub fn autocolors(mut self, value: impl Into<AutoColors>) -> ChartPlugins<A> {
        self.autocolors = Some(value.into());
        self
    }

    pub fn get_legend(&mut self) -> &mut Option<PluginLegend> {
        &mut self.legend
    }
    pub fn legend(mut self, value: impl Into<PluginLegend>) -> ChartPlugins<A> {
        self.legend = Some(value.into());
        self
    }

    pub fn get_title(&mut self) -> &mut Option<Title> {
        &mut self.title
    }
    pub fn title(mut self, value: impl Into<Title>) -> ChartPlugins<A> {
        self.title = Some(value.into());
        self
    }

    pub fn get_tooltip(&mut self) -> &mut Option<TooltipPlugin> {
        &mut self.tooltip
    }
    pub fn tooltip(mut self, value: impl Into<TooltipPlugin>) -> ChartPlugins<A> {
        self.tooltip = Some(value.into());
        self
    }
}

impl PluginLegend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_display(&mut self) -> &mut Option<bool> {
        &mut self.display
    }
    pub fn display(mut self, value: impl Into<bool>) -> PluginLegend {
        self.display = Some(value.into());
        self
    }

    pub fn get_labels(&mut self) -> &mut Option<LegendLabel> {
        &mut self.labels
    }
    pub fn labels(mut self, value: impl Into<LegendLabel>) -> PluginLegend {
        self.labels = Some(value.into());
        self
    }

    pub fn get_position(&mut self) -> &mut String {
        &mut self.position
    }
    pub fn position(mut self, value: impl Into<String>) -> PluginLegend {
        self.position = value.into();
        self
    }

    pub fn get_reverse(&mut self) -> &mut Option<bool> {
        &mut self.reverse
    }
    pub fn reverse(mut self, value: impl Into<bool>) -> PluginLegend {
        self.reverse = Some(value.into());
        self
    }
}

impl<A: Annotation> Annotations<A> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_annotations(&mut self) -> &mut Option<HashMap<String, A>> {
        &mut self.annotations
    }
    pub fn annotations<T: Into<String>, U: IntoIterator<Item = (T, A)>>(
        mut self,
        value: U,
    ) -> Annotations<A> {
        self.annotations = Some(value.into_iter().map(|(k, v)| (k.into(), v)).collect());
        self
    }
}

impl AutoColors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_mode(&mut self) -> &mut String {
        &mut self.mode
    }
    pub fn mode(mut self, value: impl Into<String>) -> AutoColors {
        self.mode = value.into();
        self
    }
}

impl TooltipPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_background_color(&mut self) -> &mut String {
        &mut self.backgroundColor
    }
    pub fn background_color(mut self, value: impl Into<String>) -> TooltipPlugin {
        self.backgroundColor = value.into();
        self
    }

    pub fn get_body_align(&mut self) -> &mut String {
        &mut self.bodyAlign
    }
    pub fn body_align(mut self, value: impl Into<String>) -> TooltipPlugin {
        self.bodyAlign = value.into();
        self
    }

    pub fn get_body_color(&mut self) -> &mut String {
        &mut self.bodyColor
    }
    pub fn body_color(mut self, value: impl Into<String>) -> TooltipPlugin {
        self.bodyColor = value.into();
        self
    }

    pub fn get_callbacks(&mut self) -> &mut Option<TooltipCallbacks> {
        &mut self.callbacks
    }
    pub fn callbacks(mut self, value: impl Into<TooltipCallbacks>) -> TooltipPlugin {
        self.callbacks = Some(value.into());
        self
    }

    pub fn get_filter(&mut self) -> &mut FnWithArgs {
        &mut self.filter
    }
    pub fn filter(mut self, value: impl Into<FnWithArgs>) -> TooltipPlugin {
        self.filter = value.into();
        self
    }

    pub fn get_display_colors(&mut self) -> &mut Option<bool> {
        &mut self.displayColors
    }
    pub fn display_colors(mut self, value: impl Into<bool>) -> TooltipPlugin {
        self.displayColors = Some(value.into());
        self
    }

    pub fn get_enabled(&mut self) -> &mut Option<bool> {
        &mut self.enabled
    }
    pub fn enabled(mut self, value: impl Into<bool>) -> TooltipPlugin {
        self.enabled = Some(value.into());
        self
    }

    pub fn get_title_align(&mut self) -> &mut String {
        &mut self.titleAlign
    }
    pub fn title_align(mut self, value: impl Into<String>) -> TooltipPlugin {
        self.titleAlign = value.into();
        self
    }

    pub fn get_title_color(&mut self) -> &mut String {
        &mut self.titleColor
    }
    pub fn title_color(mut self, value: impl Into<String>) -> TooltipPlugin {
        self.titleColor = value.into();
        self
    }

    pub fn get_title_margin_bottom(&mut self) -> &mut NumberString {
        &mut self.titleMarginBottom
    }
    pub fn title_margin_bottom(mut self, value: impl Into<NumberString>) -> TooltipPlugin {
        self.titleMarginBottom = value.into();
        self
    }
}

impl ChartLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_padding(&mut self) -> &mut Option<Padding> {
        &mut self.padding
    }
    pub fn padding(mut self, value: impl Into<Padding>) -> ChartLayout {
        self.padding = Some(value.into());
        self
    }
}

impl TooltipCallbacks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_label(&mut self) -> &mut FnWithArgs {
        &mut self.label
    }
    pub fn label(mut self, value: impl Into<FnWithArgs>) -> TooltipCallbacks {
        self.label = value.into();
        self
    }

    pub fn get_title(&mut self) -> &mut FnWithArgs {
        &mut self.title
    }
    pub fn title(mut self, value: impl Into<FnWithArgs>) -> TooltipCallbacks {
        self.title = value.into();
        self
    }
}

impl ChartScale {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_after_build_ticks(&mut self) -> &mut FnWithArgs {
        &mut self.afterBuildTicks
    }
    pub fn after_build_ticks(mut self, value: impl Into<FnWithArgs>) -> ChartScale {
        self.afterBuildTicks = value.into();
        self
    }

    pub fn get_align_to_pixels(&mut self) -> &mut Option<bool> {
        &mut self.alignToPixels
    }
    pub fn align_to_pixels(mut self, value: impl Into<bool>) -> ChartScale {
        self.alignToPixels = Some(value.into());
        self
    }

    pub fn get_background_color(&mut self) -> &mut String {
        &mut self.backgroundColor
    }
    pub fn background_color(mut self, value: impl Into<String>) -> ChartScale {
        self.backgroundColor = value.into();
        self
    }

    pub fn get_bar_percentage(&mut self) -> &mut NumberString {
        &mut self.barPercentage
    }
    pub fn bar_percentage(mut self, value: impl Into<NumberString>) -> ChartScale {
        self.barPercentage = value.into();
        self
    }

    pub fn get_before_fit(&mut self) -> &mut FnWithArgs {
        &mut self.beforeFit
    }
    pub fn before_fit(mut self, value: impl Into<FnWithArgs>) -> ChartScale {
        self.beforeFit = value.into();
        self
    }

    pub fn get_begin_at_zero(&mut self) -> &mut Option<bool> {
        &mut self.beginAtZero
    }
    pub fn begin_at_zero(mut self, value: impl Into<bool>) -> ChartScale {
        self.beginAtZero = Some(value.into());
        self
    }

    pub fn get_border(&mut self) -> &mut Option<ScaleBorder> {
        &mut self.border
    }
    pub fn border(mut self, value: impl Into<ScaleBorder>) -> ChartScale {
        self.border = Some(value.into());
        self
    }

    pub fn get_bounds(&mut self) -> &mut String {
        &mut self.bounds
    }
    pub fn bounds(mut self, value: impl Into<String>) -> ChartScale {
        self.bounds = value.into();
        self
    }

    pub fn get_category_percentage(&mut self) -> &mut NumberString {
        &mut self.categoryPercentage
    }
    pub fn category_percentage(mut self, value: impl Into<NumberString>) -> ChartScale {
        self.categoryPercentage = value.into();
        self
    }

    pub fn get_display(&mut self) -> &mut Option<bool> {
        &mut self.display
    }
    pub fn display(mut self, value: impl Into<bool>) -> ChartScale {
        self.display = Some(value.into());
        self
    }

    pub fn get_grace(&mut self) -> &mut NumberOrDateString {
        &mut self.grace
    }
    pub fn grace(mut self, value: impl Into<NumberOrDateString>) -> ChartScale {
        self.grace = value.into();
        self
    }

    pub fn get_grid(&mut self) -> &mut Option<Grid> {
        &mut self.grid
    }
    pub fn grid(mut self, value: impl Into<Grid>) -> ChartScale {
        self.grid = Some(value.into());
        self
    }

    pub fn get_grouped(&mut self) -> &mut Option<bool> {
        &mut self.grouped
    }
    pub fn grouped(mut self, value: impl Into<bool>) -> ChartScale {
        self.grouped = Some(value.into());
        self
    }

    pub fn get_max(&mut self) -> &mut NumberOrDateString {
        &mut self.max
    }
    pub fn max(mut self, value: impl Into<NumberOrDateString>) -> ChartScale {
        self.max = value.into();
        self
    }

    pub fn get_min(&mut self) -> &mut NumberOrDateString {
        &mut self.min
    }
    pub fn min(mut self, value: impl Into<NumberOrDateString>) -> ChartScale {
        self.min = value.into();
        self
    }

    pub fn get_offset(&mut self) -> &mut Option<bool> {
        &mut self.offset
    }
    pub fn offset(mut self, value: impl Into<bool>) -> ChartScale {
        self.offset = Some(value.into());
        self
    }

    pub fn get_position(&mut self) -> &mut String {
        &mut self.position
    }
    pub fn position(mut self, value: impl Into<String>) -> ChartScale {
        self.position = value.into();
        self
    }

    pub fn get_reverse(&mut self) -> &mut Option<bool> {
        &mut self.reverse
    }
    pub fn reverse(mut self, value: impl Into<bool>) -> ChartScale {
        self.reverse = Some(value.into());
        self
    }

    pub fn get_stacked(&mut self) -> &mut Option<bool> {
        &mut self.stacked
    }
    pub fn stacked(mut self, value: impl Into<bool>) -> ChartScale {
        self.stacked = Some(value.into());
        self
    }

    pub fn get_suggested_max(&mut self) -> &mut NumberOrDateString {
        &mut self.suggestedMax
    }
    pub fn suggested_max(mut self, value: impl Into<NumberOrDateString>) -> ChartScale {
        self.suggestedMax = value.into();
        self
    }

    pub fn get_suggested_min(&mut self) -> &mut NumberOrDateString {
        &mut self.suggestedMin
    }
    pub fn suggested_min(mut self, value: impl Into<NumberOrDateString>) -> ChartScale {
        self.suggestedMin = value.into();
        self
    }

    pub fn get_ticks(&mut self) -> &mut Option<ScaleTicks> {
        &mut self.ticks
    }
    pub fn ticks(mut self, value: impl Into<ScaleTicks>) -> ChartScale {
        self.ticks = Some(value.into());
        self
    }

    pub fn get_time(&mut self) -> &mut Option<ScaleTime> {
        &mut self.time
    }
    pub fn time(mut self, value: impl Into<ScaleTime>) -> ChartScale {
        self.time = Some(value.into());
        self
    }

    pub fn get_title(&mut self) -> &mut Option<Title> {
        &mut self.title
    }
    pub fn title(mut self, value: impl Into<Title>) -> ChartScale {
        self.title = Some(value.into());
        self
    }

    pub fn get_r_type(&mut self) -> &mut String {
        &mut self.r#type
    }
    pub fn r_type(mut self, value: impl Into<String>) -> ChartScale {
        self.r#type = value.into();
        self
    }

    pub fn get_weight(&mut self) -> &mut NumberString {
        &mut self.weight
    }
    pub fn weight(mut self, value: impl Into<NumberString>) -> ChartScale {
        self.weight = value.into();
        self
    }
}

impl ScaleBorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_color(&mut self) -> &mut String {
        &mut self.color
    }
    pub fn color(mut self, value: impl Into<String>) -> ScaleBorder {
        self.color = value.into();
        self
    }

    pub fn get_dash(&mut self) -> &mut Vec<NumberString> {
        &mut self.dash
    }
    pub fn dash<T: Into<NumberString>>(
        mut self,
        value: impl IntoIterator<Item = T>,
    ) -> ScaleBorder {
        self.dash = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn get_dash_offset(&mut self) -> &mut NumberString {
        &mut self.dashOffset
    }
    pub fn dash_offset(mut self, value: impl Into<NumberString>) -> ScaleBorder {
        self.dashOffset = value.into();
        self
    }

    pub fn get_display(&mut self) -> &mut Option<bool> {
        &mut self.display
    }
    pub fn display(mut self, value: impl Into<bool>) -> ScaleBorder {
        self.display = Some(value.into());
        self
    }

    pub fn get_width(&mut self) -> &mut NumberString {
        &mut self.width
    }
    pub fn width(mut self, value: impl Into<NumberString>) -> ScaleBorder {
        self.width = value.into();
        self
    }

    pub fn get_z(&mut self) -> &mut NumberString {
        &mut self.z
    }
    pub fn z(mut self, value: impl Into<NumberString>) -> ScaleBorder {
        self.z = value.into();
        self
    }
}

impl Grid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_color(&mut self) -> &mut String {
        &mut self.color
    }
    pub fn color(mut self, value: impl Into<String>) -> Grid {
        self.color = value.into();
        self
    }

    pub fn get_display(&mut self) -> &mut Option<bool> {
        &mut self.display
    }
    pub fn display(mut self, value: impl Into<bool>) -> Grid {
        self.display = Some(value.into());
        self
    }

    pub fn get_draw_on_chart_area(&mut self) -> &mut Option<bool> {
        &mut self.drawOnChartArea
    }
    pub fn draw_on_chart_area(mut self, value: impl Into<bool>) -> Grid {
        self.drawOnChartArea = Some(value.into());
        self
    }

    pub fn get_tick_color(&mut self) -> &mut String {
        &mut self.tickColor
    }
    pub fn tick_color(mut self, value: impl Into<String>) -> Grid {
        self.tickColor = value.into();
        self
    }

    pub fn get_z(&mut self) -> &mut NumberString {
        &mut self.z
    }
    pub fn z(mut self, value: impl Into<NumberString>) -> Grid {
        self.z = value.into();
        self
    }
}

impl LineAnnotation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_border_color(&mut self) -> &mut String {
        &mut self.borderColor
    }
    pub fn border_color(mut self, value: impl Into<String>) -> LineAnnotation {
        self.borderColor = value.into();
        self
    }

    pub fn get_border_dash(&mut self) -> &mut Vec<NumberString> {
        &mut self.borderDash
    }
    pub fn border_dash<T: Into<NumberString>>(
        mut self,
        value: impl IntoIterator<Item = T>,
    ) -> LineAnnotation {
        self.borderDash = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn get_border_width(&mut self) -> &mut NumberString {
        &mut self.borderWidth
    }
    pub fn border_width(mut self, value: impl Into<NumberString>) -> LineAnnotation {
        self.borderWidth = value.into();
        self
    }

    pub fn get_draw_time(&mut self) -> &mut String {
        &mut self.drawTime
    }
    pub fn draw_time(mut self, value: impl Into<String>) -> LineAnnotation {
        self.drawTime = value.into();
        self
    }

    pub fn get_r_type(&mut self) -> &mut LineAnnotationType {
        &mut self.r#type
    }
    pub fn r_type(mut self, value: impl Into<LineAnnotationType>) -> LineAnnotation {
        self.r#type = value.into();
        self
    }

    pub fn get_x_max(&mut self) -> &mut NumberOrDateString {
        &mut self.xMax
    }
    pub fn x_max(mut self, value: impl Into<NumberOrDateString>) -> LineAnnotation {
        self.xMax = value.into();
        self
    }

    pub fn get_x_min(&mut self) -> &mut NumberOrDateString {
        &mut self.xMin
    }
    pub fn x_min(mut self, value: impl Into<NumberOrDateString>) -> LineAnnotation {
        self.xMin = value.into();
        self
    }

    pub fn get_y_max(&mut self) -> &mut NumberOrDateString {
        &mut self.yMax
    }
    pub fn y_max(mut self, value: impl Into<NumberOrDateString>) -> LineAnnotation {
        self.yMax = value.into();
        self
    }

    pub fn get_y_min(&mut self) -> &mut NumberOrDateString {
        &mut self.yMin
    }
    pub fn y_min(mut self, value: impl Into<NumberOrDateString>) -> LineAnnotation {
        self.yMin = value.into();
        self
    }

    pub fn get_y_scale_id(&mut self) -> &mut NumberString {
        &mut self.yScaleID
    }
    pub fn y_scale_id(mut self, value: impl Into<NumberString>) -> LineAnnotation {
        self.yScaleID = value.into();
        self
    }
}

impl LineAnnotationType {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BoxAnnotation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_background_color(&mut self) -> &mut String {
        &mut self.backgroundColor
    }
    pub fn background_color(mut self, value: impl Into<String>) -> BoxAnnotation {
        self.backgroundColor = value.into();
        self
    }

    pub fn get_border_color(&mut self) -> &mut String {
        &mut self.borderColor
    }
    pub fn border_color(mut self, value: impl Into<String>) -> BoxAnnotation {
        self.borderColor = value.into();
        self
    }

    pub fn get_border_dash(&mut self) -> &mut Vec<NumberString> {
        &mut self.borderDash
    }
    pub fn border_dash<T: Into<NumberString>>(
        mut self,
        value: impl IntoIterator<Item = T>,
    ) -> BoxAnnotation {
        self.borderDash = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn get_border_width(&mut self) -> &mut NumberString {
        &mut self.borderWidth
    }
    pub fn border_width(mut self, value: impl Into<NumberString>) -> BoxAnnotation {
        self.borderWidth = value.into();
        self
    }

    pub fn get_draw_time(&mut self) -> &mut String {
        &mut self.drawTime
    }
    pub fn draw_time(mut self, value: impl Into<String>) -> BoxAnnotation {
        self.drawTime = value.into();
        self
    }

    pub fn get_r_type(&mut self) -> &mut BoxAnnotationType {
        &mut self.r#type
    }
    pub fn r_type(mut self, value: impl Into<BoxAnnotationType>) -> BoxAnnotation {
        self.r#type = value.into();
        self
    }

    pub fn get_x_max(&mut self) -> &mut NumberString {
        &mut self.xMax
    }
    pub fn x_max(mut self, value: impl Into<NumberString>) -> BoxAnnotation {
        self.xMax = value.into();
        self
    }

    pub fn get_x_min(&mut self) -> &mut NumberString {
        &mut self.xMin
    }
    pub fn x_min(mut self, value: impl Into<NumberString>) -> BoxAnnotation {
        self.xMin = value.into();
        self
    }

    pub fn get_y_max(&mut self) -> &mut NumberString {
        &mut self.yMax
    }
    pub fn y_max(mut self, value: impl Into<NumberString>) -> BoxAnnotation {
        self.yMax = value.into();
        self
    }

    pub fn get_y_min(&mut self) -> &mut NumberString {
        &mut self.yMin
    }
    pub fn y_min(mut self, value: impl Into<NumberString>) -> BoxAnnotation {
        self.yMin = value.into();
        self
    }
}

impl BoxAnnotationType {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ScaleTime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_display_formats(&mut self) -> &mut Option<DisplayFormats> {
        &mut self.displayFormats
    }
    pub fn display_formats(mut self, value: impl Into<DisplayFormats>) -> ScaleTime {
        self.displayFormats = Some(value.into());
        self
    }

    pub fn get_unit(&mut self) -> &mut String {
        &mut self.unit
    }
    pub fn unit(mut self, value: impl Into<String>) -> ScaleTime {
        self.unit = value.into();
        self
    }
}

impl DisplayFormats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_day(&mut self) -> &mut String {
        &mut self.day
    }
    pub fn day(mut self, value: impl Into<String>) -> DisplayFormats {
        self.day = value.into();
        self
    }

    pub fn get_hour(&mut self) -> &mut String {
        &mut self.hour
    }
    pub fn hour(mut self, value: impl Into<String>) -> DisplayFormats {
        self.hour = value.into();
        self
    }

    pub fn get_minute(&mut self) -> &mut String {
        &mut self.minute
    }
    pub fn minute(mut self, value: impl Into<String>) -> DisplayFormats {
        self.minute = value.into();
        self
    }

    pub fn get_month(&mut self) -> &mut String {
        &mut self.month
    }
    pub fn month(mut self, value: impl Into<String>) -> DisplayFormats {
        self.month = value.into();
        self
    }

    pub fn get_quarter(&mut self) -> &mut String {
        &mut self.quarter
    }
    pub fn quarter(mut self, value: impl Into<String>) -> DisplayFormats {
        self.quarter = value.into();
        self
    }

    pub fn get_week(&mut self) -> &mut String {
        &mut self.week
    }
    pub fn week(mut self, value: impl Into<String>) -> DisplayFormats {
        self.week = value.into();
        self
    }

    pub fn get_year(&mut self) -> &mut String {
        &mut self.year
    }
    pub fn year(mut self, value: impl Into<String>) -> DisplayFormats {
        self.year = value.into();
        self
    }
}

impl ScaleTicks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_auto_skip(&mut self) -> &mut Option<bool> {
        &mut self.autoSkip
    }
    pub fn auto_skip(mut self, value: impl Into<bool>) -> ScaleTicks {
        self.autoSkip = Some(value.into());
        self
    }

    pub fn get_align(&mut self) -> &mut String {
        &mut self.align
    }
    pub fn align(mut self, value: impl Into<String>) -> ScaleTicks {
        self.align = value.into();
        self
    }

    pub fn get_callback(&mut self) -> &mut FnWithArgs {
        &mut self.callback
    }
    pub fn callback(mut self, value: impl Into<FnWithArgs>) -> ScaleTicks {
        self.callback = value.into();
        self
    }

    pub fn get_count(&mut self) -> &mut NumberString {
        &mut self.count
    }
    pub fn count(mut self, value: impl Into<NumberString>) -> ScaleTicks {
        self.count = value.into();
        self
    }

    pub fn get_font(&mut self) -> &mut Option<Font> {
        &mut self.font
    }
    pub fn font(mut self, value: impl Into<Font>) -> ScaleTicks {
        self.font = Some(value.into());
        self
    }

    pub fn get_include_bounds(&mut self) -> &mut Option<bool> {
        &mut self.includeBounds
    }
    pub fn include_bounds(mut self, value: impl Into<bool>) -> ScaleTicks {
        self.includeBounds = Some(value.into());
        self
    }

    pub fn get_max_ticks_limit(&mut self) -> &mut NumberString {
        &mut self.maxTicksLimit
    }
    pub fn max_ticks_limit(mut self, value: impl Into<NumberString>) -> ScaleTicks {
        self.maxTicksLimit = value.into();
        self
    }

    pub fn get_padding(&mut self) -> &mut Option<Padding> {
        &mut self.padding
    }
    pub fn padding(mut self, value: impl Into<Padding>) -> ScaleTicks {
        self.padding = Some(value.into());
        self
    }

    pub fn get_precision(&mut self) -> &mut NumberString {
        &mut self.precision
    }
    pub fn precision(mut self, value: impl Into<NumberString>) -> ScaleTicks {
        self.precision = value.into();
        self
    }

    pub fn get_step_size(&mut self) -> &mut NumberString {
        &mut self.stepSize
    }
    pub fn step_size(mut self, value: impl Into<NumberString>) -> ScaleTicks {
        self.stepSize = value.into();
        self
    }
}

impl Title {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_display(&mut self) -> &mut Option<bool> {
        &mut self.display
    }
    pub fn display(mut self, value: impl Into<bool>) -> Title {
        self.display = Some(value.into());
        self
    }

    pub fn get_font(&mut self) -> &mut Option<Font> {
        &mut self.font
    }
    pub fn font(mut self, value: impl Into<Font>) -> Title {
        self.font = Some(value.into());
        self
    }

    pub fn get_text(&mut self) -> &mut String {
        &mut self.text
    }
    pub fn text(mut self, value: impl Into<String>) -> Title {
        self.text = value.into();
        self
    }
}

impl ChartInteraction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_axis(&mut self) -> &mut String {
        &mut self.axis
    }
    pub fn axis(mut self, value: impl Into<String>) -> ChartInteraction {
        self.axis = value.into();
        self
    }

    pub fn get_intersect(&mut self) -> &mut Option<bool> {
        &mut self.intersect
    }
    pub fn intersect(mut self, value: impl Into<bool>) -> ChartInteraction {
        self.intersect = Some(value.into());
        self
    }

    pub fn get_mode(&mut self) -> &mut String {
        &mut self.mode
    }
    pub fn mode(mut self, value: impl Into<String>) -> ChartInteraction {
        self.mode = value.into();
        self
    }
}

impl ChartLegend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_display(&mut self) -> &mut Option<bool> {
        &mut self.display
    }
    pub fn display(mut self, value: impl Into<bool>) -> ChartLegend {
        self.display = Some(value.into());
        self
    }

    pub fn get_labels(&mut self) -> &mut Option<LegendLabel> {
        &mut self.labels
    }
    pub fn labels(mut self, value: impl Into<LegendLabel>) -> ChartLegend {
        self.labels = Some(value.into());
        self
    }

    pub fn get_position(&mut self) -> &mut String {
        &mut self.position
    }
    pub fn position(mut self, value: impl Into<String>) -> ChartLegend {
        self.position = value.into();
        self
    }
}

impl LegendLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_box_height(&mut self) -> &mut Option<usize> {
        &mut self.boxHeight
    }
    pub fn box_height(mut self, value: impl Into<usize>) -> LegendLabel {
        self.boxHeight = Some(value.into());
        self
    }

    pub fn get_box_width(&mut self) -> &mut Option<usize> {
        &mut self.boxWidth
    }
    pub fn box_width(mut self, value: impl Into<usize>) -> LegendLabel {
        self.boxWidth = Some(value.into());
        self
    }

    pub fn get_filter(&mut self) -> &mut FnWithArgs {
        &mut self.filter
    }
    pub fn filter(mut self, value: impl Into<FnWithArgs>) -> LegendLabel {
        self.filter = value.into();
        self
    }

    pub fn get_font(&mut self) -> &mut Option<Font> {
        &mut self.font
    }
    pub fn font(mut self, value: impl Into<Font>) -> LegendLabel {
        self.font = Some(value.into());
        self
    }

    pub fn get_point_style(&mut self) -> &mut String {
        &mut self.pointStyle
    }
    pub fn point_style(mut self, value: impl Into<String>) -> LegendLabel {
        self.pointStyle = value.into();
        self
    }

    pub fn get_point_style_width(&mut self) -> &mut NumberString {
        &mut self.pointStyleWidth
    }
    pub fn point_style_width(mut self, value: impl Into<NumberString>) -> LegendLabel {
        self.pointStyleWidth = value.into();
        self
    }

    pub fn get_use_border_radius(&mut self) -> &mut Option<bool> {
        &mut self.useBorderRadius
    }
    pub fn use_border_radius(mut self, value: impl Into<bool>) -> LegendLabel {
        self.useBorderRadius = Some(value.into());
        self
    }

    pub fn get_use_point_style(&mut self) -> &mut Option<bool> {
        &mut self.usePointStyle
    }
    pub fn use_point_style(mut self, value: impl Into<bool>) -> LegendLabel {
        self.usePointStyle = Some(value.into());
        self
    }
}

impl ChartElements {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_bar(&mut self) -> &mut Option<BarElementConfiguration> {
        &mut self.bar
    }
    pub fn bar(mut self, value: impl Into<BarElementConfiguration>) -> ChartElements {
        self.bar = Some(value.into());
        self
    }

    pub fn get_line(&mut self) -> &mut Option<LineElementConfiguration> {
        &mut self.line
    }
    pub fn line(mut self, value: impl Into<LineElementConfiguration>) -> ChartElements {
        self.line = Some(value.into());
        self
    }

    pub fn get_point(&mut self) -> &mut Option<PointElementConfiguration> {
        &mut self.point
    }
    pub fn point(mut self, value: impl Into<PointElementConfiguration>) -> ChartElements {
        self.point = Some(value.into());
        self
    }
}

impl BarElementConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_border_radius(&mut self) -> &mut NumberString {
        &mut self.borderRadius
    }
    pub fn border_radius(mut self, value: impl Into<NumberString>) -> BarElementConfiguration {
        self.borderRadius = value.into();
        self
    }

    pub fn get_border_width(&mut self) -> &mut NumberString {
        &mut self.borderWidth
    }
    pub fn border_width(mut self, value: impl Into<NumberString>) -> BarElementConfiguration {
        self.borderWidth = value.into();
        self
    }

    pub fn get_fill(&mut self) -> &mut Option<bool> {
        &mut self.fill
    }
    pub fn fill(mut self, value: impl Into<bool>) -> BarElementConfiguration {
        self.fill = Some(value.into());
        self
    }

    pub fn get_hover_border_width(&mut self) -> &mut NumberString {
        &mut self.hoverBorderWidth
    }
    pub fn hover_border_width(mut self, value: impl Into<NumberString>) -> BarElementConfiguration {
        self.hoverBorderWidth = value.into();
        self
    }
}

impl LineElementConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_border_width(&mut self) -> &mut NumberString {
        &mut self.borderWidth
    }
    pub fn border_width(mut self, value: impl Into<NumberString>) -> LineElementConfiguration {
        self.borderWidth = value.into();
        self
    }

    pub fn get_cubic_interpolation_mode(&mut self) -> &mut String {
        &mut self.cubicInterpolationMode
    }
    pub fn cubic_interpolation_mode(
        mut self,
        value: impl Into<String>,
    ) -> LineElementConfiguration {
        self.cubicInterpolationMode = value.into();
        self
    }

    pub fn get_fill(&mut self) -> &mut Option<bool> {
        &mut self.fill
    }
    pub fn fill(mut self, value: impl Into<bool>) -> LineElementConfiguration {
        self.fill = Some(value.into());
        self
    }
}

impl PointElementConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_border_width(&mut self) -> &mut NumberString {
        &mut self.borderWidth
    }
    pub fn border_width(mut self, value: impl Into<NumberString>) -> PointElementConfiguration {
        self.borderWidth = value.into();
        self
    }

    pub fn get_hit_radius(&mut self) -> &mut NumberString {
        &mut self.hitRadius
    }
    pub fn hit_radius(mut self, value: impl Into<NumberString>) -> PointElementConfiguration {
        self.hitRadius = value.into();
        self
    }

    pub fn get_hover_border_width(&mut self) -> &mut NumberString {
        &mut self.hoverBorderWidth
    }
    pub fn hover_border_width(
        mut self,
        value: impl Into<NumberString>,
    ) -> PointElementConfiguration {
        self.hoverBorderWidth = value.into();
        self
    }

    pub fn get_hover_radius(&mut self) -> &mut NumberString {
        &mut self.hoverRadius
    }
    pub fn hover_radius(mut self, value: impl Into<NumberString>) -> PointElementConfiguration {
        self.hoverRadius = value.into();
        self
    }

    pub fn get_radius(&mut self) -> &mut NumberString {
        &mut self.radius
    }
    pub fn radius(mut self, value: impl Into<NumberString>) -> PointElementConfiguration {
        self.radius = value.into();
        self
    }
}

impl DataLabels {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_align(&mut self) -> &mut FnWithArgsOrAny {
        &mut self.align
    }
    pub fn align(mut self, value: impl Into<FnWithArgsOrAny>) -> DataLabels {
        self.align = value.into();
        self
    }

    pub fn get_anchor(&mut self) -> &mut FnWithArgsOrAny {
        &mut self.anchor
    }
    pub fn anchor(mut self, value: impl Into<FnWithArgsOrAny>) -> DataLabels {
        self.anchor = value.into();
        self
    }

    pub fn get_background_color(&mut self) -> &mut FnWithArgsOrAny {
        &mut self.backgroundColor
    }
    pub fn background_color(mut self, value: impl Into<FnWithArgsOrAny>) -> DataLabels {
        self.backgroundColor = value.into();
        self
    }

    pub fn get_border_radius(&mut self) -> &mut NumberString {
        &mut self.borderRadius
    }
    pub fn border_radius(mut self, value: impl Into<NumberString>) -> DataLabels {
        self.borderRadius = value.into();
        self
    }

    pub fn get_clamp(&mut self) -> &mut Option<bool> {
        &mut self.clamp
    }
    pub fn clamp(mut self, value: impl Into<bool>) -> DataLabels {
        self.clamp = Some(value.into());
        self
    }

    pub fn get_clip(&mut self) -> &mut Option<bool> {
        &mut self.clip
    }
    pub fn clip(mut self, value: impl Into<bool>) -> DataLabels {
        self.clip = Some(value.into());
        self
    }

    pub fn get_color(&mut self) -> &mut String {
        &mut self.color
    }
    pub fn color(mut self, value: impl Into<String>) -> DataLabels {
        self.color = value.into();
        self
    }

    pub fn get_display(&mut self) -> &mut BoolString {
        &mut self.display
    }
    pub fn display(mut self, value: impl Into<BoolString>) -> DataLabels {
        self.display = value.into();
        self
    }

    pub fn get_draw_time(&mut self) -> &mut NumberString {
        &mut self.drawTime
    }
    pub fn draw_time(mut self, value: impl Into<NumberString>) -> DataLabels {
        self.drawTime = value.into();
        self
    }

    pub fn get_font(&mut self) -> &mut Option<Font> {
        &mut self.font
    }
    pub fn font(mut self, value: impl Into<Font>) -> DataLabels {
        self.font = Some(value.into());
        self
    }

    pub fn get_formatter(&mut self) -> &mut FnWithArgs {
        &mut self.formatter
    }
    pub fn formatter(mut self, value: impl Into<FnWithArgs>) -> DataLabels {
        self.formatter = value.into();
        self
    }

    pub fn get_offset(&mut self) -> &mut FnWithArgsOrAny {
        &mut self.offset
    }
    pub fn offset(mut self, value: impl Into<FnWithArgsOrAny>) -> DataLabels {
        self.offset = value.into();
        self
    }

    pub fn get_opacity(&mut self) -> &mut NumberString {
        &mut self.opacity
    }
    pub fn opacity(mut self, value: impl Into<NumberString>) -> DataLabels {
        self.opacity = value.into();
        self
    }

    pub fn get_padding(&mut self) -> &mut Option<Padding> {
        &mut self.padding
    }
    pub fn padding(mut self, value: impl Into<Padding>) -> DataLabels {
        self.padding = Some(value.into());
        self
    }

    pub fn get_z(&mut self) -> &mut NumberString {
        &mut self.z
    }
    pub fn z(mut self, value: impl Into<NumberString>) -> DataLabels {
        self.z = value.into();
        self
    }
}

impl Border {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_bottom(&mut self) -> &mut NumberString {
        &mut self.bottom
    }
    pub fn bottom(mut self, value: impl Into<NumberString>) -> Border {
        self.bottom = value.into();
        self
    }

    pub fn get_left(&mut self) -> &mut NumberString {
        &mut self.left
    }
    pub fn left(mut self, value: impl Into<NumberString>) -> Border {
        self.left = value.into();
        self
    }

    pub fn get_right(&mut self) -> &mut NumberString {
        &mut self.right
    }
    pub fn right(mut self, value: impl Into<NumberString>) -> Border {
        self.right = value.into();
        self
    }

    pub fn get_top(&mut self) -> &mut NumberString {
        &mut self.top
    }
    pub fn top(mut self, value: impl Into<NumberString>) -> Border {
        self.top = value.into();
        self
    }
}

impl Padding {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_bottom(&mut self) -> &mut NumberString {
        &mut self.bottom
    }
    pub fn bottom(mut self, value: impl Into<NumberString>) -> Padding {
        self.bottom = value.into();
        self
    }

    pub fn get_left(&mut self) -> &mut NumberString {
        &mut self.left
    }
    pub fn left(mut self, value: impl Into<NumberString>) -> Padding {
        self.left = value.into();
        self
    }

    pub fn get_right(&mut self) -> &mut NumberString {
        &mut self.right
    }
    pub fn right(mut self, value: impl Into<NumberString>) -> Padding {
        self.right = value.into();
        self
    }

    pub fn get_top(&mut self) -> &mut NumberString {
        &mut self.top
    }
    pub fn top(mut self, value: impl Into<NumberString>) -> Padding {
        self.top = value.into();
        self
    }
}

impl Font {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_family(&mut self) -> &mut String {
        &mut self.family
    }
    pub fn family(mut self, value: impl Into<String>) -> Font {
        self.family = value.into();
        self
    }

    pub fn get_line_height(&mut self) -> &mut NumberString {
        &mut self.lineHeight
    }
    pub fn line_height(mut self, value: impl Into<NumberString>) -> Font {
        self.lineHeight = value.into();
        self
    }

    pub fn get_size(&mut self) -> &mut NumberString {
        &mut self.size
    }
    pub fn size(mut self, value: impl Into<NumberString>) -> Font {
        self.size = value.into();
        self
    }

    pub fn get_style(&mut self) -> &mut NumberString {
        &mut self.style
    }
    pub fn style(mut self, value: impl Into<NumberString>) -> Font {
        self.style = value.into();
        self
    }

    pub fn get_weight(&mut self) -> &mut NumberString {
        &mut self.weight
    }
    pub fn weight(mut self, value: impl Into<NumberString>) -> Font {
        self.weight = value.into();
        self
    }
}

impl Segment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_border_color(&mut self) -> &mut FnWithArgs {
        &mut self.borderColor
    }
    pub fn border_color(mut self, value: impl Into<FnWithArgs>) -> Segment {
        self.borderColor = value.into();
        self
    }

    pub fn get_border_dash(&mut self) -> &mut FnWithArgs {
        &mut self.borderDash
    }
    pub fn border_dash(mut self, value: impl Into<FnWithArgs>) -> Segment {
        self.borderDash = value.into();
        self
    }
}
