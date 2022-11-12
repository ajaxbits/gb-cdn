use git2::Repository;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_stream_to_file=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // save files to a separate directory to not override files in the current directory
    tokio::fs::create_dir(UPLOADS_DIRECTORY)
        .await
        .expect("failed to create `uploads` directory");

    let app = Router::new()
        .route("/", get(show_form).post(accept_form))
        .route("/file/:file_name", post(save_request_body));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn clone_repo() {
    let url = "git@github.com:ajaxbits/grace-bobber-web.git";
    let repo = match Repository::clone(url, "./web") {
        Ok(repo) => repo,
        Err(e) => panic!("could not open repo"),
    };
}
