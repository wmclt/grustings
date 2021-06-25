use diesel::{self, prelude::*, result::QueryResult};
use rocket::serde::Serialize;

mod schema {
    table! {
        users {
            id -> Integer,
            username -> Text,
            password-> Text,
        }
    }
}

use self::schema::users;
use self::schema::users::dsl::users as all_users;

use crate::DbConn;


#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,

}

#[derive(Debug, FromForm, Insertable)]
#[table_name="users"]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<User>> {
        conn.run(|c| {
            all_users.order(users::id.desc()).load::<User>(c)
        }).await
    }

    pub async fn exists(username: String, password: String, conn: &DbConn) -> bool {
        conn.run(move |c| {
            users::table
                .filter(users::username.eq(username))
                .filter(users::password.eq(password))
                .first::<User>(c).is_ok()
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn insert(create_user: CreateUser, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::insert_into(users::table).values(&create_user).execute(c)
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(all_users.find(id)).execute(c)).await
    }

    /// Returns the number of affected rows.
    #[cfg(test)]
    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_users).execute(c)).await
    }
}