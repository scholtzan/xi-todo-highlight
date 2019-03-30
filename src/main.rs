//! A plugin to highlight todos in xi editor.
extern crate xi_core_lib as xi_core;
extern crate xi_plugin_lib;
extern crate xi_rope;

use regex::Regex;
use std::path::Path;

use crate::xi_core::ConfigTable;
use crate::xi_core::annotations::AnnotationType;
use xi_plugin_lib::{mainloop, ChunkCache, Plugin, View};
use crate::xi_core::plugin_rpc::DataSpan;
use xi_rope::interval::Interval;
use xi_rope::rope::RopeDelta;
use serde_json::json;



struct TodoHighlightPlugin {
    regex: Regex
}

impl Plugin for TodoHighlightPlugin {
    type Cache = ChunkCache;

    fn new_view(&mut self, view: &mut View<Self::Cache>) {
        self.find_todos(view, Interval::new(0, view.get_buf_size()));
    }

    fn did_close(&mut self, _view: &View<Self::Cache>) {}

    fn did_save(&mut self, _view: &mut View<Self::Cache>, _old: Option<&Path>) {}

    fn config_changed(&mut self, _view: &mut View<Self::Cache>, _changes: &ConfigTable) {}

    fn update(
        &mut self,
        view: &mut View<Self::Cache>,
        _delta: Option<&RopeDelta>,
        _edit_type: String,
        _author: String,
    ) {
        self.find_todos(view, Interval::new(0, view.get_buf_size()));
    }
}

impl TodoHighlightPlugin {
    fn new() -> TodoHighlightPlugin {
        TodoHighlightPlugin {
            regex: Regex::new(r".*(todo|fixme|note).*").unwrap()
        }
    }

    fn find_todos(&mut self, view: &mut View<ChunkCache>, interval: Interval) {
        let start_line = view.line_of_offset(0).expect("Error getting line");
        let end_line = view.line_of_offset(view.get_buf_size()).expect("Error getting line");

        let mut spans: Vec<DataSpan> = Vec::new();

        for line_nr in start_line..=end_line {
            let line_offset = view.offset_of_line(line_nr).unwrap();
            let line = view.get_line(line_nr).unwrap();
            if let Some(mat) = self.regex.find(line) {
                let start = line_offset + mat.start();
                let end = line_offset + mat.end();
                spans.push(DataSpan {
                    start,
                    end,
                    data: json!(null)
                });
            }
        }

        let annotation_type = AnnotationType::Other("todo".to_string());

        if spans.len() > 0 {
            view.update_annotations(interval.start(), interval.end(),spans, annotation_type);
        }
    }
}

fn main() {
    let mut plugin = TodoHighlightPlugin::new();
    mainloop(&mut plugin).unwrap();
}
