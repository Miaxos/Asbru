use std::fs;
use std::io::Write;
use std::{
    cell::{RefCell, RefMut},
    path::{Path, PathBuf},
    rc::Rc,
};

use codegen::{Function, Scope};

use crate::codegen::render::render::Render;

/// Structure to manage the main.rs generated file
pub struct MainFile {
    path: PathBuf,
    scope: Rc<RefCell<Scope>>,
    main_function: Rc<RefCell<Function>>,
}

impl MainFile {
    pub fn new<P: AsRef<Path>>(path: &P) -> Self {
        let main_scope = Rc::new(RefCell::new(Scope::new()));
        let main_function = Function::new("main")
            .set_async(true)
            .attr("tokio::main(flavor = \"multi_thread\")")
            .ret("anyhow::Result<()>")
            .clone();

        let path = path.as_ref().to_owned();
        MainFile {
            path,
            scope: main_scope,
            main_function: Rc::new(RefCell::new(main_function)),
        }
    }

    /// Builder arround the main file
    pub fn main_scope(&self) -> RefMut<'_, Scope> {
        self.scope.borrow_mut()
    }

    /// Builder arround the main function
    pub fn main_function(&self) -> RefMut<'_, Function> {
        self.main_function.borrow_mut()
    }

    /// Finalize the main file
    pub fn finalize(&self) -> String {
        let mut scope = self.scope.borrow_mut();
        scope.push_fn(self.main_function.borrow_mut().clone());

        scope.to_string()
    }
}

impl Render for MainFile {
    fn generate(&self) -> Result<(), crate::codegen::generate::GenericErrors> {
        let output = &self.path;
        let content = self.finalize();

        let mut f = fs::File::create(&output)?;
        f.write_all(&(content.as_bytes()))?;
        Ok(())
    }
}
