#![allow(dead_code, unused_imports)]
extern crate variants as variantslib;
use crate::variants::browser::BrowserVaraints;
use rocket::Request;
use rocket::fairing::{self, Fairing, Info, Kind};
use rocket::figment::{Source, value::magic::RelativePathBuf};
use rocket::{Build, Orbit, Rocket};

pub struct VaraintsFairing {}

impl Default for VaraintsFairing {
    fn default() -> Self {
        Self {}
    }
}

#[rocket::async_trait]
impl Fairing for VaraintsFairing {
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
        /*let mut variants_builder = crate::variants::builder::VariantsBuilder::<
            '_,
            variantslib::default::DefaultVariants,
        >::default();
        variants_builder.config();

        Ok(rocket.manage(variants_builder))*/
       Ok(rocket)
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        // todo: add a state to rocket rocket.state::<VaraintsConfigManager>()
    }

    #[cfg(debug_assertions)]
    async fn on_request(&self, _req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        /*let varaints_builder = req
        .rocket()
        .state::<VariantsBuilder>()
        .expect("VariantsBuilder registered in on_ignite");*/
        // todo: move following to rocket state.

        // todo: put in the request state.
        /*let variants = req.local_cache::<variantslib::default::DefaultVariants, _>(|| {
            let mut varaints_builder = variantslib::default::VariantsBuilder::<
                Request<'_>,
                variantslib::default::DefaultVariants,
            >::default();
            varaints_builder.with_processor(Box::new(BrowserVaraints::default()));

            let mut varaints = variantslib::default::DefaultVariants::default();
            varaints_builder.process_variants(req, &mut varaints);
            varaints
        });

        req.local_cache(|| "hello world");*/
    }
}
