#[derive(Debug, Default)]
pub struct LauncherState {
    pub loaded: bool,
    pub data: Vec<String>,
    pub data_filtered: Vec<String>,
    pub selected_index: Option<usize>,
    pub filter_query: String,
    pub launched: bool,
}

impl LauncherState {
    pub fn fetch_data(&mut self) {
        self.data = Vec::from(["aaaa".to_string(), "aaaa".to_string()]);
        self.data_filtered = self.data.clone();

        if self.data_filtered.len() > 0 {
            self.selected_index = Some(0);
        } else {
            self.selected_index = None;
        }

        self.loaded = true;
    }

    pub fn update_filter(&mut self, new_filter: String) {
        self.filter_query = new_filter;
        self.data_filtered = self
            .data
            .iter()
            .filter(|n| n.starts_with(&self.filter_query))
            .map(|s| s.clone())
            .collect();
    }

    pub fn update_selected(&mut self, index: Option<usize>) {
        self.selected_index = index;
    }

    pub fn launch_selected(&mut self) {
        panic!("Launch {}", self.selected_index.unwrap_or(0));
    }
}
