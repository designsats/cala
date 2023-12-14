use async_graphql::{types::connection::*, *};

use super::account::*;
use crate::app::CalaApp;

// use timestamp::*;

// use crate::app::CalaApp;

pub struct Query;

#[Object]
impl Query {
    async fn accounts(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> Result<Connection<AccountByNameCursor, Account, EmptyFields, EmptyFields>> {
        let app = ctx.data_unchecked::<CalaApp>();
        query(
            after,
            None,
            Some(first),
            None,
            |after, _, first, _| async move {
                let first = first.expect("First always exists");
                let result = app
                    .ledger()
                    .accounts()
                    .list(cala_types::query::PaginatedQueryArgs {
                        first: usize::try_from(first)?,
                        after: after.map(cala_types::query::AccountByNameCursor::from),
                    })
                    .await?;
                let mut connection = Connection::new(false, result.has_next_page);
                connection
                    .edges
                    .extend(result.entities.into_iter().map(|entity| {
                        let values = entity.values;
                        let cursor = AccountByNameCursor::from(&values);
                        Edge::new(cursor, Account::from(values))
                    }));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn hello(&self) -> String {
        "Hello, world!".to_string()
    }
}