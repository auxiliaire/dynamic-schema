use core::result::Result::{self, Err};
use diesel::result::Error;
use diesel::sql_types::{Integer, Text};
use diesel::*;
use diesel_dynamic_schema::table;

pub fn establish_connection() -> diesel::SqliteConnection {
    use diesel::*;

    SqliteConnection::establish(":memory:").unwrap()
}

pub fn get_users(conn: &mut diesel::SqliteConnection) -> Result<Vec<(i32, String)>, Error> {
    let users = table("users");
    let id = users.column::<Integer, _>("id");
    let name = users.column::<Text, _>("name");

    users
        .select((id, name))
        .filter(name.eq("James"))
        .load::<(i32, String)>(conn)
}

fn main() {
    println!("Dynamic Schema with Diesel");

    let conn = &mut establish_connection();
    match get_users(conn) {
        Ok(users) => {
            for (id, name) in users {
                println!("User: {} - '{}'", id, name);
            }
        }
        Err(e) => panic!("Error: {e:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(conn: &mut diesel::SqliteConnection) {
        use diesel::*;

        diesel::sql_query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL DEFAULT '', hair_color TEXT)")
            .execute(conn)
            .unwrap();

        diesel::sql_query(
            "INSERT INTO users (name, hair_color) VALUES ('James', 'brown'), ('Maria', 'blond')",
        )
        .execute(conn)
        .unwrap();
    }

    #[test]
    fn check_users() {
        let conn = &mut establish_connection();
        setup(conn);

        let users = get_users(conn).unwrap();

        assert_eq!(users[0].1, "James");
    }
}
