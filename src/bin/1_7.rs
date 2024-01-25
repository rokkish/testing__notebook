use rusqlite::{params, Connection, Result};
use std::io::{self, Write};

#[derive(Debug)]
#[allow(dead_code)] // ignore unused warning
struct Weapon {
    id: i32,
    name: String,
    power: i32,
    hand_type: String,
    enhancable: u8, // 0: false, 1: true
    enhance_num: i32,
}

// return union of io::Error, rusqlite::Error
type CustomResult<T> = Result<T, CustomErrors>;
#[derive(Debug)]
enum CustomErrors {
    Io(io::Error),
    Sqlite(rusqlite::Error),
}

fn main() {
    // create a connection to a new in-memory database
    let conn = Connection::open_in_memory().unwrap();

    // create weapons
    let ws = create_weapons();

    // create a table and insert some data
    init_database(&conn).unwrap();

    // insert weapons using copy
    for w in &ws {
        insert_weapon_data(&conn, w).unwrap();
    }

    // enhance weapons
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for w in ws {
        enhance_weapon(&mut stdout, &conn, w).unwrap();
    }

    // print out inserted rows
    let mut stmt = conn
        .prepare("SELECT id, name, power, hand_type, enhancable, enhance_num FROM weapons")
        .unwrap();
    let weapon_iter = stmt
        .query_map([], |row| {
            Ok(Weapon {
                id: row.get(0)?,
                name: row.get(1)?,
                power: row.get(2)?,
                hand_type: row.get(3)?,
                enhancable: row.get(4)?,
                enhance_num: row.get(5)?,
            })
        })
        .unwrap();
    for weapon in weapon_iter {
        println!("Found weapon {:?}", weapon.unwrap());
    }

    // Ok(())
}

fn enhance_weapon<W: Write>(writer: &mut W, conn: &Connection, w: Weapon) -> CustomResult<()> {
    // update power if weapon is enhancable
    if w.enhancable == 0 {
        match writeln!(writer, "can not enhancable") {
            Ok(_) => (),
            Err(e) => return Err(CustomErrors::Io(e)),
        }
        return Ok(());
    }

    // enhance is limited to 1 time
    if w.enhance_num >= 1 {
        match writeln!(writer, "can not enhancable more than 1 time") {
            Ok(_) => (),
            Err(e) => return Err(CustomErrors::Io(e)),
        }
        return Ok(()); // return early
    }
    let result: Result<usize, rusqlite::Error> = conn.execute(
        "UPDATE weapons SET power = ?1, enhance_num = ?2 WHERE id = ?3",
        params![w.power + 10, w.enhance_num + 1, w.id],
    );
    match result {
        Ok(_) => (),
        Err(e) => return Err(CustomErrors::Sqlite(e)),
    }
    match writeln!(writer, "enhanced!") {
        Ok(_) => (),
        Err(e) => return Err(CustomErrors::Io(e)),
    }
    Ok(())
}

fn init_database(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE weapons (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  power           INTEGER NOT NULL,
                  hand_type       TEXT NOT NULL,
                  enhancable      INTEGER NOT NULL,
                  enhance_num     INTEGER NOT NULL
                  )",
        [],
    )?;
    Ok(())
}

fn create_weapons() -> Vec<Weapon> {
    let ws: Vec<Weapon> = vec![
        Weapon {
            id: 0,
            name: "ZelcovSword".to_string(),
            power: 5,
            hand_type: "one".to_string(),
            enhancable: 0,
            enhance_num: 0,
        },
        Weapon {
            id: 1,
            name: "CarrotSword".to_string(),
            power: 20,
            hand_type: "one".to_string(),
            enhancable: 0,
            enhance_num: 0,
        },
        Weapon {
            id: 2,
            name: "WoodSword".to_string(),
            power: 25,
            hand_type: "two".to_string(),
            enhancable: 0,
            enhance_num: 0,
        },
        Weapon {
            id: 3,
            name: "DragonSword".to_string(),
            power: 50,
            hand_type: "two".to_string(),
            enhancable: 1,
            enhance_num: 0,
        },
        Weapon {
            id: 4,
            name: "KusanagiSword".to_string(),
            power: 90,
            hand_type: "one".to_string(),
            enhancable: 1,
            enhance_num: 0,
        },
    ];
    ws
}

fn insert_weapon_data(conn: &Connection, w: &Weapon) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO weapons (id, name, power, hand_type, enhancable, enhance_num)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            w.id,
            w.name,
            w.power,
            w.hand_type,
            w.enhancable,
            w.enhance_num
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // not enhancable
    #[case(Weapon { id: 0, name: "ZelcovSword".to_string(), power: 5, hand_type: "one".to_string(), enhancable: 0, enhance_num: 0 }, "can not enhancable")]
    // enhancable
    #[case(Weapon { id: 3, name: "DragonSword".to_string(), power: 50, hand_type: "two".to_string(), enhancable: 1, enhance_num: 0 }, "enhanced!")]
    // already enhanced
    #[case(Weapon { id:3, name: "DragonSword".to_string(), power: 50, hand_type: "two".to_string(), enhancable: 1, enhance_num: 1 }, "can not enhancable more than 1 time")]
    fn test_enhance_weapon(#[case] w: Weapon, #[case] expected_stdout: String) {
        let conn = Connection::open_in_memory().unwrap();
        init_database(&conn).unwrap();
        // insert weapons
        insert_weapon_data(&conn, &w).unwrap();

        // capture println message
        let mut buf = Vec::<u8>::new();

        // enhance weapons
        enhance_weapon(&mut buf, &conn, w).unwrap();

        buf.pop(); // remove end of line
        assert_eq!(buf, expected_stdout.as_bytes());
    }
}
