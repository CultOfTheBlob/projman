pub struct ProjectContext<'a> {
    pub name: &'a str,
    pub repo: &'a str,
    pub license: &'a str,
}

impl ProjectContext<'_> {
    pub fn format(&self, target: &str) -> String {
        target
            .replace("#{name}", self.name)
            .replace("#{repo}", self.repo)
            .replace("#{license}", self.license)
    }
}
