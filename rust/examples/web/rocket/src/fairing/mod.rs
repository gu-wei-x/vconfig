mod variants_processors;

use rocket::fairing::{self, Fairing, Info, Kind};
use rocket::figment::value::magic::RelativePathBuf;
use rocket::{Build, Orbit, Rocket};
use variants_rocket::VariantsContext;

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
        #[cfg(debug_assertions)]
        let kind = kind | Kind::Request;

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
                if let Some(mut variants_context) = VariantsContext::new(&dir) {
                    // add all processor here.
                    variants_context
                        .with_processor(variants_processors::browser::BrowserVariants::default());
                    Ok(rocket.manage(variants_context))
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
            .state::<VariantsContext>()
            .expect("VariantsContext registered in on_ignite");
    }

    #[cfg(debug_assertions)]
    async fn on_request(&self, _req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        // todo: can process variants and store in request.local_cache? not sure
    }
}
