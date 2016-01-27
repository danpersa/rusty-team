extern crate rustc_serialize;
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate nickel;

mod team;

use nickel::{ Nickel, MediaType, HttpRouter, QueryString };
use nickel::status::StatusCode::BadRequest;
use rustc_serialize::json;
use team::Team;

//static ACCESS_TOKEN_INVALID: &'static str = "Access token invalid.";
static MISSING_MEMBER_NAME: &'static str = "Missing member name.";

fn invalid_request<S: Into<String>>(err: S) -> String {
    format!("{{\"error\":\"invalid_request\",\"error_description\":\"{}\"}}", err.into())
}

fn main() {
    env_logger::init().unwrap();

    let mut server = Nickel::new();
    info!("Welcome to rusty-team");

    server.get("/api/teams", middleware! { |req, mut res|
        res.set(MediaType::Json);
        let member = match req.query().get("member") {
            Some(member) => member.to_string(),
            None => {
                res.set(BadRequest);
                return res.send(invalid_request(MISSING_MEMBER_NAME))
            }
        };

        debug!("Request member: {:?}", member);

        let teams = match Team::from_query_param(&member) {
            Ok(teams) => teams,
            Err(err) => {
                res.set(BadRequest);
                return res.send(invalid_request(err));
            }
        };

        debug!("Teams: {:?}", teams);
        json::encode(&teams).unwrap()
    });

    server.listen("0.0.0.0:6768");
}
