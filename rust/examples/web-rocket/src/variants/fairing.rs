use crate::variants::config::VaraintsConfig;
use rocket::fairing::{self, Fairing, Info, Kind};
use rocket::figment::value::magic::RelativePathBuf;
use rocket::{Build, Orbit, Rocket};

pub(crate) struct VaraintsFairing {}

impl Default for VaraintsFairing {
    fn default() -> Self {
        VaraintsFairing {}
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
        let configured_dir = rocket
            .figment()
            .extract_inner::<RelativePathBuf>("config_dir")
            .map(|path| path.relative());

        match configured_dir {
            Ok(dir) => {
                if let Some(config) = VaraintsConfig::new(&dir) {
                    Ok(rocket.manage(config))
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
            .state::<VaraintsConfig>()
            .expect("VaraintsConfig registered in on_ignite");
    }

    #[cfg(debug_assertions)]
    async fn on_request(&self, _req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        // todo: can process varaints and store in request.local_cache? not sure
    }
}
