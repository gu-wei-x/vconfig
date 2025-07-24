mod index;

use rocket::Route;
use rocket::routes;

pub(crate) fn routes() -> Vec<Route> {
    routes![index::index]
}
