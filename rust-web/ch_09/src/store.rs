use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use tracing::Level;

use crate::types::account::{Account, AccountId, NewAccount};
use crate::types::answer::NewAnswer;
use crate::types::question::NewQuestion;
use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};

use handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(err) => {
                panic!("Could't establish DB connection: {}", err)
            }
        };

        Store {
            connection: db_pool,
        }
    }
}

impl Store {
    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, Error> {
        match sqlx::query("select * from questions limit $1 offset $2")
            .bind(limit)
            .bind(offset)
            .map(|row| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);
                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        match sqlx::query("insert into questions (title,content,tags) values ($1,$2,$3) returning id,title,content,tags")
                .bind(new_question.title)
                .bind(new_question.content)
                .bind(new_question.tags)
                .map(|row|{
                    Question{
                        id: QuestionId(row.get("id")) ,
                        title: row.get("title") ,
                        content: row.get("content") ,
                        tags: row.get("tags") ,
                    }
                })
                .fetch_one(&self.connection)
                .await {
            Ok(question) => {Ok(question)},
            Err(err) => {
                tracing::event!(tracing::Level::ERROR,"{:?}",err);
                Err(Error::DatabaseQueryError(err))
            },
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, Error> {
        match sqlx::query("update questions set title = $1 ,content = $2,tags = $3 where id = $4 returning id,title,content,tags")
                .bind(question.title)
                .bind(question.content)
                .bind(question.tags)
                .bind(question_id)
                .map(|row|{
                    Question{
                        id: QuestionId(row.get("id")),
                        title: row.get("title"),
                        content: row.get("content"),
                        tags: row.get("tags")
                    }
                })
                .fetch_one(&self.connection)
                .await {
            Ok(question) => {Ok(question)},
            Err(err) => {
                tracing::event!(tracing::Level::ERROR,"{:?}",err);
                Err(Error::DatabaseQueryError(err))
            },
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<bool, Error> {
        match sqlx::query("delete from questions where id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);
                Err(Error::DatabaseQueryError(err))
            }
        }
    }
}

impl Store {
    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        match sqlx::query("insert into answers (content, question_id) values ($1, $2) returning id,content,question_id")
                .bind(new_answer.content)
                .bind(new_answer.question_id.0)
                .map(|row| {
                    Answer{
                        id: AnswerId(row.get("id") ),
                        content: row.get("content") ,
                        question_id: QuestionId(row.get("question_id") ),
                    }
                })
                .fetch_one(&self.connection)
                .await {
            Ok(answer) => {Ok(answer)},
            Err(err) => {
                tracing::event!(tracing::Level::ERROR,"{:?}",err);
                Err(Error::DatabaseQueryError(err))
            },
        }
    }
}

impl Store {
    pub async fn add_account(&self, account: NewAccount) -> Result<bool, Error> {
        match sqlx::query("insert into accounts (nickname,email,password) values ($1,$2,$3)")
            .bind(account.nickname)
            .bind(account.email)
            .bind(account.password)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                let code = err
                    .as_database_error()
                    .unwrap()
                    .code()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let db_message = err.as_database_error().unwrap().message();

                let constraint = err.as_database_error().unwrap().constraint();

                tracing::event!(
                    tracing::Level::ERROR,
                    code = code,
                    db_message = db_message,
                    constraint = constraint
                );

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub(crate) async fn get_account(&self, email: String) -> Result<Account, Error> {
        match sqlx::query("select * from account where email = $1")
            .bind(email)
            .fetch_one(&self.connection)
            .await
        {
            Ok(row) => Ok(Account {
                id: Some(AccountId(row.get("id"))),
                nickname: row.get("nickname"),
                email: row.get("email"),
                password: row.get("password"),
            }),
            Err(e) => {
                tracing::event!(Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}
