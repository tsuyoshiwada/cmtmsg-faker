use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method};

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS Header",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, req: &Request, res: &mut Response) {
        if req.method() == Method::Get {
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            res.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
            res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        }
    }
}


