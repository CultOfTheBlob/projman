use crate::{
    app_state::AppState,
    prelude::*,
    project::{Existant, Project},
    template::Template,
};
use std::sync::Arc;

impl Project<Existant> {
    pub fn run(project: &Arc<Self>, app_state: &Arc<AppState>) -> Result<()> {
        app_state
            .get_template(&project.template_name)?
            .run(&project.path)
    }

    pub fn template<'a>(&self, app_state: &'a AppState) -> Result<&'a Template> {
        app_state.get_template(&self.template_name)
    }
}
