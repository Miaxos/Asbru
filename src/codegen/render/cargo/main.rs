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
            .doc("Asbru auto-generated project")
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
        self.main_scope().import("async_graphql", "Schema");
        self.main_scope().import("async_graphql", "EmptyMutation");
        self.main_scope()
            .import("async_graphql", "EmptySubscription");
        self.main_scope().import("warp", "Filter");
        self.main_scope().import("tower::make", "Shared");
        self.main_scope().import("tower", "ServiceBuilder");

        self.main_scope().import("domain::query", "Query");
        self.main_function().line(
            r#"
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .finish();

    let cors = warp::cors()
        .allow_methods(vec!["POST"])
        .allow_header("content-type")
        .allow_any_origin()
        .build();

    let graphql_post = warp::post()
        .and(warp::path("graphql"))
        .and(async_graphql_warp::graphql(schema))
        .and_then(
            |(schema, request): (
                Schema<Query, EmptyMutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                Ok::<_, std::convert::Infallible>(async_graphql_warp::Response::from(
                    schema
                        .execute(request)
                        .await,
                ))
            },
        );

    let filters = graphql_post
            .with(cors)
            .with(warp::trace::request());

    let service = ServiceBuilder::new().timeout(std::time::Duration::from_secs(10)).service(warp::service(filters));

    let service = Shared::new(service);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = std::net::TcpListener::bind(addr).unwrap();

    warp::hyper::Server::from_tcp(listener).unwrap().serve(service).await?;

    Ok(())
        "#,
        );
        let content = self.finalize();

        let mut f = fs::File::create(&output)?;
        f.write_all(&(content.as_bytes()))?;
        Ok(())
    }
}
