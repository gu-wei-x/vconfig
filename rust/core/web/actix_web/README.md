Use vconfig in Actix Web apps
===========================
[<img alt="github" src="https://img.shields.io/badge/github-guweix/vconfig_actix_web-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/gu-wei-x/vconfig)
[<img alt="crates.io" src="https://img.shields.io/crates/v/vconfig_actix_web.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/vconfig_actix_web)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-vconfig_actix_web-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/vconfig_actix_web/)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gu-wei-x/vconfig/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/gu-wei-x/vconfig/actions?query=branch%3Amain)

vconfig_actix_web is a wrap crate on vconfig which has macro and context to leverage vconfig in Actix Web apps.

## Actix web!

Let's write first Actix Web application leveraging vconfig! Start by creating a new binary-based
Cargo project and changing into the new directory. Code could be found at [vconfig: actix-web-example](https://github.com/gu-wei-x/vconfig/tree/main/rust/examples/web/actix-web/src)

```sh
cargo new actix-web-example --bin
cd actix-web-example
cargo add actix_web
cargo add vconfig_actix_web
```

Create a folder with files like bellow:
```
#:.
│   Cargo.toml
│
└───src
    │   main.rs
    │
    ├───app_state
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

We will add configuration file with content for diffent browser brands, an variants processor to detect browser brand, a context with configs and variants processors stored in app data as singleton, a handler to show the configed content based on browser brand from request context.

### configs: index.toml
```
# welcom message based on browser
welcome_msg&browser:edge = "Hello! you are using Edge browser"
welcome_msg&browser:chrome = "Hello! you are using Chrome browser"
welcome_msg = "Hello! you are not using Chromium-based browser"
```

### variants_processors: browser.rs
```
use actix_web::http::header::USER_AGENT;
pub(crate) struct Browservariants;

impl Default for Browservariants {
    fn default() -> Self {
        Self {}
    }
}

impl vconfig_actix_web::VariantsProcessor for Browservariants {
    fn process(
        &self,
        request: &actix_web::HttpRequest,
        variants: &mut dyn vconfig_actix_web::Variants,
    ) {
        match request.headers().get("sec-ch-ua") {
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
                if let Some(user_agent_value) = request.headers().get(USER_AGENT) {
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
use actix_web::Responder;
use vconfig_actix_web::serde::Deserialize;
use vconfig_actix_web::vconfig;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_actix_web::serde")]
#[vconfig("index")] // with configs in config store.
//#[vconfig("./src/configs/index.toml")] // relative path to wroking directory.
//#[vconfig(file = "./src/configs/index.toml")] // relative path to wroking directory.
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> impl Responder {
    index_config.welcome_msg
}
```

### src: main.rs
```
mod app_state;
mod handlers;
use actix_web::{App, HttpServer, web};
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new({
                let mut vconfig_context =
                    vconfig_actix_web::VConfigContext::new(&Path::new("src/configs")).unwrap();
                vconfig_context
                    .with_processor(app_state::variants_processors::Browservariants::default());
                vconfig_context
            }))
            .route("/", web::get().to(handlers::index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
```

### run the app and navigate to the url with Chrome/Edge
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running `actix-web-example.exe`
```