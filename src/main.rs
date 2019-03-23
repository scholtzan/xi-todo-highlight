//! A plugin to highlight todos in xi editor.
extern crate xi_core_lib as xi_core;
extern crate xi_plugin_lib;
extern crate xi_rope;

use regex::Regex;
use std::path::Path;

use crate::xi_core::ConfigTable;
use crate::xi_core::annotations::{AnnotationSlice, AnnotationType, AnnotationRange};
use xi_plugin_lib::{mainloop, ChunkCache, Plugin, View};
use xi_rope::interval::Interval;
use xi_rope::rope::RopeDelta;

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
        delta: Option<&RopeDelta>,
        _edit_type: String,
        _author: String,
    ) {
        if let Some(delta) = delta {
            let (iv, _) = delta.summary();
            self.find_todos(view, iv);
        }
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

        let mut ranges: Vec<AnnotationRange> = Vec::new();

        for line_nr in start_line..=end_line {
            if let Ok(line) = view.get_line(line_nr) {
                if let Some(mat) = self.regex.find(line) {
                    ranges.push(AnnotationRange {
                        start_line: line_nr,
                        start_col: mat.start(),
                        end_line: line_nr,
                        end_col: mat.end()
                    });
                }
            }
        }

        let annotation_type = AnnotationType::Other("todo".to_string());

        if ranges.len() > 0 {
            let a = view.line_of_offset(interval.start).unwrap();
            let b = view.line_of_offset(interval.end).unwrap() + 1;
            let start = view.offset_of_line(a).unwrap();
            let end = view.offset_of_line(b).unwrap() - start;

            view.update_annotations(start, end, &vec![AnnotationSlice::new(annotation_type, ranges, None)]);
        }
    }
}

fn main() {
    let mut plugin = TodoHighlightPlugin::new();
    mainloop(&mut plugin).unwrap();
}
