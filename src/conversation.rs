use rocket::serde::Serialize;
use diesel::{self, result::QueryResult, prelude::*};

mod schema {
    table! {
        conversations {
            id -> Integer,
            user1 -> Integer,
            user2 -> Integer,
        }
    }/* 
    table! {
        message {
            id -> Integer,
            conversation -> Integer,
            sender -> Integer,
            text -> Text,
        }
    } */
}

use self::schema::conversations;
use self::schema::conversations::dsl::{conversations as all_convs};

use crate::DbConn;

#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Conversation {
    pub id: i32,
    pub user1: i32,
    pub user2: i32
}

#[derive(Debug, FromForm, Insertable)]
#[table_name="conversations"]
pub struct CreateConversation {
    pub user1: i32,
    pub user2: i32
}
/* 
#[derive(Debug, FromForm)]
pub struct NewMessage {
    pub conversation: i32,
    pub sender: i32,
    pub text: String,
} */

impl Conversation {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Conversation>> {
        conn.run(|c| {
            all_convs.order(conversations::id.desc()).load::<Conversation>(c)
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn insert(create_conv: CreateConversation, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::insert_into(conversations::table).values(&create_conv).execute(c)
        }).await
    }

    /// Returns the number of affected rows: 1.
    // pub async fn toggle_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
    //     conn.run(move |c| {
    //         let task = all_convs.find(id).get_result::<Conversation>(c)?;
    //         let new_status = !task.completed;
    //         let updated_task = diesel::update(all_convs.find(id));
    //         updated_task.set(task_completed.eq(new_status)).execute(c)
    //     }).await
    // }

    /// Returns the number of affected rows: 1.
    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(all_convs.find(id)).execute(c)).await
    }

    /// Returns the number of affected rows.
    #[cfg(test)]
    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(all_convs).execute(c)).await
    }
}
