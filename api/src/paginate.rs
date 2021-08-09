pub struct Paginate {
    pub limit: u32,
    pub offset: u32
}

impl Paginate {
    pub fn new(per_page: Option<u32>, page: Option<u32>) -> Self {
        let per_page = per_page.unwrap_or(50);
        let mut page = page.unwrap_or(1);

        if page == 0 {
            page = 1;
        }

        Self {
            limit: per_page,
            offset: (page - 1) * per_page
        }
    }
}
