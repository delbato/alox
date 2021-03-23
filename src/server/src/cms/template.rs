use std::{
    sync::{
        Arc,
        RwLock
    },
    path::Path
};

use serde_json::Value;
use tera::{
    Tera,
    Context
};

#[derive(Debug, Clone)]
pub struct TemplateService {
    tera_arc: Arc<RwLock<Tera>>
}

impl TemplateService {
    pub fn new(template_dir: &Path) -> Result<Self, ()> {
        let template_dir = template_dir.join("**/*");
        let dir_str = template_dir.to_str().ok_or(())?;
        let mut tera = Tera::new(dir_str)
            .map_err(|_| ())?;
        tera.autoescape_on(vec![]);
        let tera_arc = Arc::new(RwLock::new(tera));
        Ok(Self{
            tera_arc
        })
    }

    pub fn render(&self, template: &str, params: Value) -> Result<String, ()> {
        let context = Context::from_value(params).map_err(|_| ())?;
        let tera_guard = self.tera_arc.read().map_err(|_| ())?;
        tera_guard.render(template, &context)
            .map_err(|_| ())
    }
}