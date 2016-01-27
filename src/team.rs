use rustc_serialize::{Encodable, Encoder};

static MEMBER_FORMAT: &'static str = "The member param should have the format `user~1`.";
static MEMBER_START_ERR: &'static str = "The member param does not start with `user~`.";
static TEAMS_MISSING: &'static str = "No teams defined";

#[derive(Debug)]
pub struct Team {
    id: String,
    team_type: String
}

impl Encodable for Team {

    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Team", 1, |encoder| {
            try!(encoder.emit_struct_field( "id", 0, |encoder| self.id.encode(encoder)));
            try!(encoder.emit_struct_field( "type", 1, |encoder| self.team_type.encode(encoder)));
            Ok(())
        })
    }
}

impl Team {
    fn new(user_number: &str) -> Vec<Team> {
        vec![
            Team { id: "team".to_string() + user_number, team_type: "official".to_string() },
            Team { id: "other_team".to_string() + user_number, team_type: "virtual".to_string() }
        ]
    }

    pub fn from_query_param(param: &str) -> Result<Vec<Team>, String> {
        let parts: Vec<&str> = param.split("~").collect();
        if parts[0] != "user" {
            return Err(format!("{} {}", MEMBER_START_ERR, MEMBER_FORMAT));
        }

        let teams = match parts.len() {
            1 => {
                warn!("{}", TEAMS_MISSING);
                vec![]
            },
            _ => {
                Team::new(parts[1])
            }
        };

        Ok(teams)
    }
}

#[cfg(test)]
mod tests {
    use super::{Team};
    use rustc_serialize::json;

    #[test]
    fn team_to_json_test() {
        let team = Team {
            id: "team1".to_string(),
            team_type: "official".to_string()
        };

        assert_eq!("{\"id\":\"team1\",\"type\":\"official\"}",
                   json::encode(&team).unwrap());
    }

    #[test]
    fn team_from_query_param_test() {
        let team = &Team::from_query_param("user~1").unwrap();
        assert_eq!("[{\"id\":\"team1\",\"type\":\"official\"},{\"id\":\"other_team1\",\"type\":\"virtual\"}]",
                   json::encode(team).unwrap());
    }

    #[test]
    fn teams_to_json_test() {
        let team1 = Team {
            id: "team1".to_string(),
            team_type: "official".to_string()
        };

        let team2 = Team {
            id: "team2".to_string(),
            team_type: "virtual".to_string()
        };

        let teams = vec![&team1, &team2];

        assert_eq!("[{\"id\":\"team1\",\"type\":\"official\"},{\"id\":\"team2\",\"type\":\"virtual\"}]",
                   json::encode(&teams).unwrap());
    }
}
