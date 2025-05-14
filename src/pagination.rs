pub struct Pagination {
    pub items_per_page: usize,
    pub current_page: usize,
    pub total_items: usize,
}

impl Pagination {
    pub fn new(items_per_page: usize, total_items: usize) -> Self {
        Self {
            items_per_page,
            current_page: 0,
            total_items,
        }
    }

    pub fn next_page(&mut self) {
        if (self.current_page + 1) * self.items_per_page < self.total_items {
            self.current_page += 1;
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
        }
    }

    pub fn first_page(&mut self) {
        self.current_page = 0;
    }

    pub fn last_page(&mut self) {
        let total = self.total_pages();
        self.current_page = if total > 0 { total - 1 } else { 0 };
    }

    pub fn total_pages(&self) -> usize {
        if self.total_items == 0 {
            1
        } else {
            (self.total_items + self.items_per_page - 1) / self.items_per_page
        }
    }

    pub fn page_info(&self) -> String {
        if self.total_items == 0 {
            return format!("Page 1/1 (No items)");
        }
        
        format!(
            "Page {}/{} (Items {}-{} of {})",
            self.current_page + 1,
            self.total_pages(),
            self.current_page * self.items_per_page + 1,
            std::cmp::min((self.current_page + 1) * self.items_per_page, self.total_items),
            self.total_items
        )
    }

    pub fn visible_items_range(&self) -> (usize, usize) {
        if self.total_items == 0 {
            return (0, 0);
        }
        
        let start = self.current_page * self.items_per_page;
        if start >= self.total_items {
            return (0, 0);
        }
        
        let end = std::cmp::min(start + self.items_per_page, self.total_items);
        (start, end)
    }

    pub fn get_visible_items<'a, T>(&self, items: &'a [T]) -> &'a [T] {
        if items.is_empty() {
            return &[];
        }
        
        let (start, end) = self.visible_items_range();
        if start == 0 && end == 0 {
            return &[];
        }
        
        &items[start..end]
    }
}