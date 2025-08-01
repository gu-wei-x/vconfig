Use vconfig in Axum apps
===========================
[<img alt="github" src="https://img.shields.io/badge/github-guweix/vconfig_axum-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/gu-wei-x/vconfig)
[<img alt="crates.io" src="https://img.shields.io/crates/v/vconfig_axum.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/vconfig_axum)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-vconfig_axum-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/vconfig_axum)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gu-wei-x/vconfig/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/gu-wei-x/vconfig/actions?query=branch%3Amain)

vconfig_axum is a wrap crate on vconfig which has macro and context to leverage vconfig in Axum apps.

## Axum apps!

Let's write first Axum application leveraging vconfig! Start by creating a new binary-based
Cargo project and changing into the new directory. Code could be found at [vconfig: axum-example](https://github.com/gu-wei-x/vconfig/tree/main/rust/examples/web/axum/src)

```sh
cargo new axum-example --bin
cd axum-example
cargo add axum
cargo add vconfig_axum
```

Create a folder with files like bellow:
```
#:.
│   Cargo.toml
│
└───src
    │   main.rs
    │
    ├───app_extensions
    │   │   mod.rs
    │   │
    │   └───variants_processors
    │           browser.rs
    │           mod.rs
    │
    ├───configs
    │       index.toml
    │
    └───handlers
            index.rs
            mod.rs


```

We will add configuration file with content for diffent browser brands, an variants processor to detect browser brand, a context with configs and variants processors stored as Axum extension, a handler to show the configed content based on browser brand from request context.

### configs: index.toml
```
# welcom message based on browser
welcome_msg&browser:edge = "Hello! you are using Edge browser"
welcome_msg&browser:chrome = "Hello! you are using Chrome browser"
welcome_msg = "Hello! you are not using Chromium-based browser"
```

### app_extensions/variants_processors: browser.rs
```
use axum::http::request::Parts;
use vconfig_axum::VariantsProcessor;
use vconfig_axum::Variants;

pub(crate) struct BrowserVariants;

impl Default for BrowserVariants {
    fn default() -> Self {
        Self {}
    }
}

impl VariantsProcessor for BrowserVariants {
    fn process(&self, request: &Parts, variants: &mut dyn Variants) {
        match request.headers.get("sec-ch-ua") {
            Some(sec_ch_ua_value) => {
                if let Ok(value_str) = sec_ch_ua_value.to_str() {
                    let lowwe_cased_value_str = value_str.to_lowercase();
                    if lowwe_cased_value_str.contains("microsoft edge") {
                        _ = variants.add("browser", "edge");
                    } else if lowwe_cased_value_str.contains("google chrome") {
                        _ = variants.add("browser", "chrome");
                    }
                }
            }
            _ => {
                if let Some(user_agent_value) = request.headers.get("user-agent") {
                    if let Ok(value_str) = user_agent_value.to_str() {
                        let lowwe_cased_value_str = value_str.to_lowercase();
                        if lowwe_cased_value_str.contains("chrome/") {
                            match lowwe_cased_value_str.contains("edg/") {
                                true => {
                                    _ = variants.add("browser", "edge");
                                }
                                false => {
                                    _ = variants.add("browser", "chrome");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

### handlers: index.rs
```
use vconfig_axum::serde::Deserialize;
use vconfig_axum::vconfig;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_axum::serde")]
#[vconfig("index")] // with configs in config store.
//#[vconfig("./src/configs/index.toml")] // relative path to wroking directory.
//#[vconfig(file = "./src/configs/index.toml")] // relative path to wroking directory.
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> String {
    index_config.welcome_msg
}

```

### src: main.rs
```
mod app_extensions;
mod handlers;
use axum::{Extension, Router, routing::get};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // create variants context.
    let vconfig_context =
        app_extensions::create_vconfig_context(&std::path::Path::new("./src/configs")).unwrap();
    let app = Router::new()
        .route("/", get(handlers::index))
        // add the variants context as an extension
        .layer(Extension(Arc::new(vconfig_context)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### run the app and navigate to the url with Chrome/Edge
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running `axum-example.exe`
```