![background](/background.png)

Hey, today I wanted to share my knowledge on how to write a Rest API in Rust. It may be easier than you think!
We won't showcase database connectivity in this article. Instead, we focused on demonstrating how to generate `OpenAPI` specifications and serve a `Swagger UI`.

You can find the full code source on [github](https://github.com/leon3s/ntex-rest-api-example).

Before starting, make sure you have [Rust](https://www.rust-lang.org) installed.

Let's start by initializing a new project using `cargo init`.

```sh
cargo init my-rest-api
cd my-rest-api
```

That should produce the following directory structure:

```console
├── Cargo.toml
└── src
    └── main.rs
```

You can use `rustfmt` for formatting. To do so, create a `rustfmt.toml` file with the following content:

```toml
indent_style = "Block"
max_width = 80
tab_spaces = 2
reorder_imports = false
reorder_modules = false
force_multiline_blocks = true
brace_style = "PreferSameLine"
control_brace_style = "AlwaysSameLine"
```

I personally use VSCode. Optionally, you can add this configuration in your `.vscode/settings.json`:

```json
{
  "editor.rulers": [80],
  "editor.tabSize": 2,
  "editor.detectIndentation": false,
  "editor.trimAutoWhitespace": true,
  "editor.formatOnSave": true,
  "files.insertFinalNewline": true,
  "files.trimTrailingWhitespace": true,
  "rust-analyzer.showUnlinkedFileNotification": false,
  "rust-analyzer.checkOnSave": true,
  "rust-analyzer.check.command": "clippy"
}
```

Your new directory structure should look like this:

```console
├── .gitignore
├── .vscode
│   └── settings.json
├── Cargo.lock
├── Cargo.toml
├── rustfmt.toml
└── src
    └── main.rs
```

We are going to use [ntex](https://ntex.rs) as our HTTP framework.<br/>
We can install Rust dependencies by running `cargo add`.<br/>
Note that when using `ntex`, we have the ability to choose our `runtime`.<br/>
To quickly summarize, the `runtime` will manage your `async|await` pattern.<br/>
If you are familiar with the `nodejs runtime`, it's kind of similar in usage.

For this tutorial, we are going to use tokio as it seems to be the more popular choice. Let's add ntex as a dependency:

```sh
cargo add ntex --features tokio
```

Then we are going to update our `main.rs` file with the following content:

```rust
use ntex::web;

#[web::get("/")]
async fn index() -> &'static str {
  "Hello world!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| web::App::new().service(index))
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
  Ok(())
}
```

We can run our project by using the following command:

```sh
cargo run
```

This command will compile our code and run it.<br/>
You should see the following output:

```console
Finished dev [unoptimized + debuginfo] target(s) in 17.38s
Running `target/debug/my-rest-api`
```

We can test our server using curl:

```
curl -v localhost:8080
```

```console
*   Trying 127.0.0.1:8080...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET / HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.68.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 12
< content-type: text/plain; charset=utf-8
< date: Fri, 26 May 2023 11:43:01 GMT
<
* Connection #0 to host localhost left intact
Hello world!%
```

Congratulations! You now have your first HTTP server in `Rust`!

Now let's create our first `REST endpoints`.

Regarding the directory architecture, it's up to personal preference. In `ntex`, we use the `.service()` method to add new `endpoints`. Therefore, I have chosen to create a directory called `services` to house my endpoints.

Let's create the directory:

```sh
mkdir src/services
touch src/services/mod.rs
```

Note that by default, `Rust` tries to import a `mod.rs` file from our directories.

Let's create our default `endpoints` inside `services/mod.rs`:

```rust
use ntex::web;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}
```

Now we need to indicate that we want to use this module in our main.rs:

```rust
use ntex::web;

mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      // Default endpoint for unregisterd endpoints
      .default_service(web::route().to(services::default)
    )
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
```

Now, for any unregistered `endpoints`, we will have a 404 error.

Before continuing, let's add four dependencies: `serde` and `serde_json` for JSON serialization, and `utoipa` with `utoipa-swagger-ui` to have an `OpenAPI` swagger.

```sh
cargo add serde --features derive
cargo add serde_json utoipa utoipa-swagger-ui
```

Next, we are going to create our own `HttpError` type as helpers. Create a file under `src/error.rs` with the following content:

```rust
use ntex::web;
use ntex::http;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

/// An http error response
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HttpError {
  /// The error message
  pub msg: String,
  /// The http status code, skipped in serialization
  #[serde(skip)]
  pub status: http::StatusCode,
}

/// Helper function to display an HttpError
impl std::fmt::Display for HttpError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.status, self.msg)
  }
}

/// Implement standard error for HttpError
impl std::error::Error for HttpError {}

/// Helper function to convert an HttpError into a ntex::web::HttpResponse
impl web::WebResponseError for HttpError {
  fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
    web::HttpResponse::build(self.status).json(&self)
  }
}
```

We need to import our error module in our `main.rs` let update it:

```rust
use ntex::web;

mod error;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      // Default endpoint for unregisterd endpoints
      .default_service(web::route().to(services::default)
    )
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
```

I think we are ready to write some example `endpoints`. Let's simulate a todo list and create a new file under `src/services/todo.rs`:

```rust
use ntex::web;

#[web::get("/todos")]
pub async fn get_todos() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[web::post("/todos")]
pub async fn create_todo() -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}

#[web::get("/todos/{id}")]
pub async fn get_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[web::put("/todos/{id}")]
pub async fn update_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[web::delete("/todos/{id}")]
pub async fn delete_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_todos);
  cfg.service(create_todo);
  cfg.service(get_todo);
  cfg.service(update_todo);
  cfg.service(delete_todo);
}
```

We need to update our `src/services/mod.rs` to import our `todo.rs`:

```rust
pub mod todo;

use ntex::web;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}
```

In our `main.rs`:

```rust
use ntex::web;

mod error;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      // Register todo endpoints
      .configure(services::todo::ntex_config)
      // Default endpoint for unregisterd endpoints
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
```

Let's create some data structure for our `Todo`.
We are going to create a new directory `src/models` with his `mod.rs` and a `todo.rs`

```sh
mkdir src/models
touch src/models/mod.rs
touch src/models/todo.rs
```

In our `src/models/mod.rs` we are going to import `todo.rs`:

```rust
pub mod todo;
```

And inside `src/models/todo.rs`, we are going to add some `data structure`:

```rust
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

/// Todo model
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Todo {
  /// The todo id
  pub id: i32,
  /// The todo title
  pub title: String,
  /// The todo completed status
  pub completed: bool,
}

/// Partial Todo model
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoPartial {
  /// The todo title
  pub title: String,
}
```

You may notice that we use the `serde` and `utoipa` derive macros to enable `JSON` serialization and conversion to `OpenAPI Schema`.

Don't forget to update your `main.rs` to import our `models`:

```rust
use ntex::web;

mod error;
mod models;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      // Register todo endpoints
      .configure(services::todo::ntex_config)
      // Default endpoint for unregisterd endpoints
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
```

With the models in place, we can now generate type-safe endpoints with their documentation. Let's update our `endpoints` inside `src/services/todo.rs`:

```rust
use ntex::web;

use crate::models::todo::TodoPartial;

/// List all todos
#[utoipa::path(
  get,
  path = "/todos",
  responses(
    (status = 200, description = "List of Todo", body = [Todo]),
  ),
)]
#[web::get("/todos")]
pub async fn get_todos() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// Create a new todo
#[utoipa::path(
  post,
  path = "/todos",
  request_body = TodoPartial,
  responses(
    (status = 201, description = "Todo created", body = Todo),
  ),
)]
#[web::post("/todos")]
pub async fn create_todo(
  _todo: web::types::Json<TodoPartial>,
) -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}

/// Get a todo by id
#[utoipa::path(
  get,
  path = "/todos/{id}",
  responses(
    (status = 200, description = "Todo found", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
#[web::get("/todos/{id}")]
pub async fn get_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// Update a todo by id
#[utoipa::path(
  put,
  path = "/todos/{id}",
  request_body = TodoPartial,
  responses(
    (status = 200, description = "Todo updated", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
#[web::put("/todos/{id}")]
pub async fn update_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// Delete a todo by id
#[utoipa::path(
  delete,
  path = "/todos/{id}",
  responses(
    (status = 200, description = "Todo deleted", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
#[web::delete("/todos/{id}")]
pub async fn delete_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_todos);
  cfg.service(create_todo);
  cfg.service(get_todo);
  cfg.service(update_todo);
  cfg.service(delete_todo);
}
```

With utoipa, we will be able to serve our Swagger documentation.

Let's create a new file under `src/services/openapi.rs`:

```rust
use std::sync::Arc;

use ntex::web;
use ntex::http;
use ntex::util::Bytes;
use utoipa::OpenApi;

use crate::error::HttpError;
use crate::models::todo::{Todo, TodoPartial};

use super::todo;

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
  paths(
    todo::get_todos,
    todo::create_todo,
    todo::get_todo,
    todo::update_todo,
    todo::delete_todo,
  ),
  components(schemas(Todo, TodoPartial, HttpError))
)]
pub(crate) struct ApiDoc;

#[web::get("/{tail}*")]
async fn get_swagger(
  tail: web::types::Path<String>,
  openapi_conf: web::types::State<Arc<utoipa_swagger_ui::Config<'static>>>,
) -> Result<web::HttpResponse, HttpError> {
  if tail.as_ref() == "swagger.json" {
    let spec = ApiDoc::openapi().to_json().map_err(|err| HttpError {
      status: http::StatusCode::INTERNAL_SERVER_ERROR,
      msg: format!("Error generating OpenAPI spec: {}", err),
    })?;
    return Ok(
      web::HttpResponse::Ok()
        .content_type("application/json")
        .body(spec),
    );
  }
  let conf = openapi_conf.as_ref().clone();
  match utoipa_swagger_ui::serve(&tail, conf.into()).map_err(|err| {
    HttpError {
      msg: format!("Error serving Swagger UI: {}", err),
      status: http::StatusCode::INTERNAL_SERVER_ERROR,
    }
  })? {
    None => Err(HttpError {
      status: http::StatusCode::NOT_FOUND,
      msg: format!("path not found: {}", tail),
    }),
    Some(file) => Ok({
      let bytes = Bytes::from(file.bytes.to_vec());
      web::HttpResponse::Ok()
        .content_type(file.content_type)
        .body(bytes)
    }),
  }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  let swagger_config = Arc::new(
    utoipa_swagger_ui::Config::new(["/explorer/swagger.json"])
      .use_base_layout(),
  );
  config.service(
    web::scope("/explorer/")
      .state(swagger_config)
      .service(get_swagger),
  );
}
```

Don't forget to update `src/services/mod.rs` to import `src/services/openapi.rs`:

```rust
pub mod todo;
pub mod openapi;

use ntex::web;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}
```

Then we can update our `main.rs` to register our explorer endpoints:

```rust
use ntex::web;

mod error;
mod models;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      // Register swagger endpoints
      .configure(services::openapi::ntex_config)
      // Register todo endpoints
      .configure(services::todo::ntex_config)
      // Default endpoint for unregisterd endpoints
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
```

We are good to go. Let's run our server:

```sh
cargo run
```

Then we should be able to access to our [explorer](http://localhost:8080/explorer/) on [http://localhost:8080/explorer/](http://localhost:8080/explorer/)

![swagger](/swagger.png)

I Hope you will try to write your next REST API in Rust !

Don't forget to take a look at the dependencies documentation:

- [ntex](https://ntex.rs)
- [serde](https://serde.rs)
- [serde_json](https://github.com/serde-rs/json)
- [utoipa](https://github.com/juhaku/utoipa)

## Bonus

Create a production docker image !
Add a `Dockerfile` in your project directory with the following content:

```Dockerfile
# Builder
FROM rust:1.69.0-alpine3.17 as builder

WORKDIR /app

## Install build dependencies
RUN apk add alpine-sdk musl-dev build-base upx

## Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

## Build release binary
RUN cargo build --release --target x86_64-unknown-linux-musl
## Pack release binary with UPX (optional)
RUN upx --best --lzma /app/target/x86_64-unknown-linux-musl/release/my-rest-api

# Runtime
FROM scratch

## Copy release binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my-rest-api /app

ENTRYPOINT ["/app"]
```

Optionally you can add this `release` profile in your `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"
codegen-units = 1
strip = true
lto = true
```

This will optimise the release binary to be as small as possible. Additionally with [upx](https://upx.github.io/) we can create really small docker image !

Build your image:

```sh
docker build -t my-rest-api:0.0.1 -f Dockerfile .
```

![docker_image_ls](/docker_image_ls.png)

If you want to see a more real world usecase i invite you to take a look at my opensource project [Nanocl](https://github.com/nxthat/nanocl). That try to simplify the development and deployment of micro services, with containers or virtual machines !

Happy coding !
