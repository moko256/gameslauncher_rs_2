use crate::features::launcher::{
    launch_item::LaunchItem, launch_item_repository::LaunchItemRepository,
};

#[derive(Debug, Default)]
pub struct LauncherState {
    pub loaded: bool,
    pub data: Vec<LaunchItem>,
    pub data_filtered: Vec<LaunchItem>,
    pub selected_index: Option<usize>,
    pub filter_query: String,
}

impl LauncherState {
    pub fn fetch_data(&mut self) {
        self.data = LaunchItemRepository::get_items();
        self.data_filtered = self.data.clone();

        self.reset_selected();

        self.loaded = true;
    }

    pub fn update_filter(&mut self, new_filter: String) {
        self.filter_query = new_filter;
        self.data_filtered = self
            .data
            .iter()
            .filter(|v| Self::query_matches_index_or_name(&self.filter_query, v.id, &v.name))
            .map(|v| v.clone())
            .collect();

        self.reset_selected();
    }

    fn query_matches_index_or_name(query: &str, target_index: usize, target_name: &str) -> bool {
        let query_trimmed = query.trim();

        if let Ok(query_num) = query_trimmed.parse::<usize>() {
            if query_num == target_index {
                return true;
            }
        }

        if target_name.contains(query_trimmed) {
            return true;
        }

        false
    }

    fn reset_selected(&mut self) {
        if self.data_filtered.len() > 0 {
            self.selected_index = Some(0);
        } else {
            self.selected_index = None;
        }
    }

    pub fn update_selected(&mut self, index: Option<usize>) {
        self.selected_index = index;
    }

    
    pub fn get_selected(&mut self) -> Option<LaunchItem> {
        if let Some(index) = self.selected_index {
            if let Some(target) = self.data_filtered.get(index) {
                return Some(target.clone())
            }
        }

        None
    }
}
