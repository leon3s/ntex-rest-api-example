# Rest Api In Rust

Hey, today i wanted to share my knowledge on how i write Rest API in Rust.

Before starting make sure you have [rust](https://www.rust-lang.org) installed.

Let's start by initing a new project using `cargo init`.

```sh
cargo init my-rest-api
cd my-rest-api
```

That should produce the following directory tree:

```console
├── Cargo.toml
└── src
    └── main.rs
```

You can use `rustfmt` for formatting, to do so, create a `rustfmt.toml` file with the following content:

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

I'm personnaly using vscode, optionally you can add this config in your `.vscode/settings.json`:

```json
{
  "editor.rulers": [
    80
  ],
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

Your new directory tree should look like this:

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

We are going to use [ntex](https://ntex.rs) for our http framework.
We can install Rust dependencies by running `cargo add`.
Note that when using ntex, we have tha ability to choose our `runtime`.
To quickly summurize, the `runtime` will manage your `async|await` patern if you are familliar with `nodejs` runtime it's kinda have the same usage.<br/>
For this tutorial we are going to use `tokio` as it's seems to me the more popular choice.
Let's add ntex as dependencies:

```sh
cargo add ntex --features tokio
```

Then we are going to update our `main.rs` with the following content:

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

We can run our project by using:

```sh
cargo run
```

This will compile our code and run it.
You should see the following output:

```console
Finished dev [unoptimized + debuginfo] target(s) in 17.38s
Running `target/debug/my-rest-api`
```

We can test our server with curl:

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

Congratz you have your first http server in `Rust`!

Now let's create our first `rest endpoints`.

For the directory architectures, it's kinda how you feel.<br />
On `ntex` we use a `.service()` method to add new `endpoints` so i choosed to create a directory `services`
to add my `endpoints` in it.

Let's create our directory:

```sh
mkdir src/services
touch src/services/mod.rs
```

Note that by default `rust` try to import a `mod.rs` file from our directories.

Let's create our default `endpoints` inside `services/mod.rs`

```rust
use ntex::web;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}
```

Now we need to notify that we want to use this module in our `main.rs`:

```rust
use ntex::web;

mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new().default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
```

Now for any unregistered `endpoints` we will have a 404.

Before continuing we will add four dependencies, `serde` and `serde_json` for `JSON` serialization, and `utoipa` with `utoipa-swagger-ui` to have an `OpenApi` swagger.

```sh
cargo add serde --features derive
cargo add serde_json utoipa utoipa-swagger-ui
```

Next we are going to create our own `HttpError` type as helpers.
Create a file under `src/error.rs` with the following content:

```rust
use ntex::web;
use ntex::http;
use serde::{Serialize, Deserialize};

/// An http error response
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    web::App::new().default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
```

I think we are ready to write some `endpoints` examples,
let's simulate a todo list, create a new file under `src/services/todo.rs`:

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

in our `src/models/mod.rs` we are going to import `todo.rs`:

```rust
pub mod todo;
```

And inside `src/models/todo.rs` we are going to add some `data structure`:

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

You can notice that we use `serde` and `utoipa` derive macro to enable `JSON` serialization and conversion to `OpenApi Schema`.

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
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
```
