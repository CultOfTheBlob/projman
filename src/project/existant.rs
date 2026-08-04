use crate::{
    app_state::AppState,
    prelude::*,
    project::{Existant, Project},
    template::Template,
};

impl Project<Existant> {
    pub fn run(&self, app_state: &AppState) -> Result<()> {
        app_state.get_template(&self.template_name)?.run(&self.path)
    }

    pub fn template<'a>(&self, app_state: &'a AppState) -> Result<&'a Template> {
        app_state.get_template(&self.template_name)
    }
}
