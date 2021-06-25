use rocket::serde::Serialize;
use diesel::{self, result::QueryResult, prelude::*};
use uuid::Uuid;

mod schema {
    table! {
        conversations {
            id -> uuid::Uuid,
            user1 -> uuid::Uuid,
            user2 -> uuid::Uuid,
        }
    }
    table! {
        users {
            id -> Uuid,
            username -> String,
            password-> String,
        }
    }
    table! {
        message {
            id -> uuid::Uuid,
            conversation -> uuid::Uuid,
            sender -> uuid::Uuid,
            text -> String,
        }
    }
}

use self::schema::conversations;
use self::schema::conversations::dsl::{conversations as all_convs, completed as conv_completed};

use crate::DbConn;

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name="conversations"]
pub struct Conversation {
    pub id: Uuid,
    pub user1: Uuid,
    pub user2: Uuid
}

#[derive(Debug, FromForm)]
pub struct NewMessage {
    pub conversation: Uuid,
    pub sender: Uuid,
    pub text: String,
}

impl Conversation {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Conversation>> {
        conn.run(|c| {
            all_convs.order(tasks::id.desc()).load::<Conversation>(c)
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn insert(new_message: NewMessage, conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| {
            let t = Conversation { id: None, description: new_message.description, completed: false };
            diesel::insert_into(tasks::table).values(&t).execute(c)
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn toggle_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let task = all_convs.find(id).get_result::<Conversation>(c)?;
            let new_status = !task.completed;
            let updated_task = diesel::update(all_convs.find(id));
            updated_task.set(task_completed.eq(new_status)).execute(c)
        }).await
    }

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
