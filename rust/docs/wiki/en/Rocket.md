# Use vconfig in Rocket apps

vconfig_rocket is a wrap crate on vconfig which has macro and context to leverage vconfig in Rocket apps.

## Rocket app!

Let's write first Rocket application leveraging vconfig! Start by creating a new binary-based
Cargo project and changing into the new directory. Code could be found at [vconfig: rocket-example](../../../examples/web/rocket)

```sh
cargo new rocket-example --bin
cd rocket-example
cargo add rocket
cargo add vconfig_rocket
```

Create a folder with files like bellow:
```
#:.
│   Cargo.toml
│   Rocket.toml
│
└───src
    │   main.rs
    │
    ├───configs
    │       index.toml
    │
    ├───fairing
    │   │   mod.rs
    │   │
    │   └───variants_processors
    │           browser.rs
    │           mod.rs
    │
    └───handlers
            index.rs
            mod.rs

```

We will add configuration file with content for diffent browser brands, an variants processor to detect browser brands, a context with configs and variants processors stored in app state as singleton, a handler to show the configed content based on browser brand from request context.

### configs: index.toml
```
# welcom message based on browser
welcome_msg&browser:edge = "Hello! you are using Edge browser"
welcome_msg&browser:chrome = "Hello! you are using Chrome browser"
welcome_msg = "Hello! you are not using Chromium-based browser"
```

### variants_processors: browser.rs
```
use rocket::Request;
pub(crate) struct Browservariants;

impl Default for Browservariants {
    fn default() -> Self {
        Self {}
    }
}

impl vconfig_rocket::VariantsProcessor for Browservariants {
    fn process<'r>(
        &self,
        request: &'r Request<'_>,
        variants: &mut dyn vconfig_rocket::Variants,
    ) {
        match request.headers().get_one("sec-ch-ua") {
            Some(sec_ch_ua_value) => {
                let lowwe_cased_value_str = sec_ch_ua_value.to_lowercase();
                if lowwe_cased_value_str.contains("microsoft edge") {
                    _ = variants.add("browser", "edge");
                } else if lowwe_cased_value_str.contains("google chrome") {
                    _ = variants.add("browser", "chrome");
                }
            }
            _ => {
                if let Some(user_agent_value) = request.headers().get_one("user-agent") {
                    let lowwe_cased_value_str = user_agent_value.to_lowercase();
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
```
### fairing: mod.rs
```
mod variants_processors;

use rocket::fairing::{self, Fairing, Info, Kind};
use rocket::figment::value::magic::RelativePathBuf;
use rocket::{Build, Orbit, Rocket};
use vconfig_rocket::VConfigContext;

pub(crate) struct VariantsConfigFairing;

impl Default for VariantsConfigFairing {
    fn default() -> Self {
        VariantsConfigFairing {}
    }
}

#[rocket::async_trait]
impl Fairing for VariantsConfigFairing {
    fn info(&self) -> Info {
        let kind = Kind::Ignite | Kind::Liftoff;
        Info {
            kind,
            name: "variants",
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let configured_dir = rocket
            .figment()
            .extract_inner::<RelativePathBuf>("config_dir")
            .map(|path| path.relative());

        match configured_dir {
            Ok(dir) => {
                if let Some(mut vconfig_context) = VConfigContext::new(&dir) {
                    // add all processor here.
                    vconfig_context
                        .with_processor(variants_processors::browser::Browservariants::default());
                    Ok(rocket.manage(vconfig_context))
                } else {
                    // todo: log error.
                    Err(rocket)
                }
            }
            _ => Ok(rocket),
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let _config = rocket
            .state::<VConfigContext>()
            .expect("VConfigContext registered in on_ignite");
    }
}
```

### handlers: index.rs
```
use rocket::get;
use vconfig_rocket::serde::Deserialize;
use vconfig_rocket::vconfig;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_rocket::serde")]
#[vconfig("index")] // with configs in config store.
//#[vconfig("./src/configs/index.toml")] // relative path to wroking directory.
//#[vconfig(file = "./src/configs/index.toml")] // relative path to wroking directory.
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

#[get("/")]
pub(crate) async fn index(index_config: IndexConfig) -> String {
    index_config.welcome_msg
}
```

### src: main.rs
```
mod fairing;
mod handlers;

use crate::fairing::VariantsConfigFairing;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket: rocket::Rocket<rocket::Ignite> = rocket::build()
        .mount("/", handlers::routes())
        .attach(VariantsConfigFairing::default())
        .launch()
        .await?;
    Ok(())
}
```

### run the app
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `rocket-example.exe`
Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 20
   >> max blocking threads: 512
   >> ident: Rocket
   >> IP header: X-Real-IP
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
Routes:
   >> (index) GET /
Fairings:
   >> Shield (liftoff, response, singleton)
   >> variants (ignite, liftoff, request)
Shield:
   >> Permissions-Policy: interest-cohort=()
   >> X-Content-Type-Options: nosniff
   >> X-Frame-Options: SAMEORIGIN
Rocket has launched from http://127.0.0.1:8000
```

### navigate to the url with Chrome and Edge.
* Chrome
>> <img src="./images/web_chrome.jpg" width="900">

* Edge
>> <img src="./images/web_edge.jpg" width="900">

---