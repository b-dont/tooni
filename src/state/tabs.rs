use anyhow::Result;

#[derive(Default, Clone)]
pub struct TabState {
    offset: usize,
    selected: Option<usize>,
    tab_menu: Vec<SheetTab>
}

impl TabState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected(&self) -> Result<Option<usize>> {
        Ok(self.selected)
    }

    pub fn select(&mut self, index: Option<usize>) -> Result<()> {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
        Ok(())
    }
}

#[derive(Clone)]
enum SheetTab {
    Details,
    Features,
    Spells
}
