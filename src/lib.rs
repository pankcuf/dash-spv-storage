use std::ops::DerefMut;
use diesel::{Insertable, QueryResult, RunQueryDsl, Table};
use crate::connection_manager::get_pooled_connection;

mod schema;
mod test;
pub mod models;
pub mod connection_manager;

#[macro_use] extern crate diesel;


///
/// Common methods
///

pub fn delete<T>(source: T) -> QueryResult<usize>
    where
        T: diesel::query_builder::IntoUpdateTarget + diesel::associations::HasTable,
        T::Table: diesel::query_builder::QueryId + diesel::QuerySource,
        T::WhereClause: diesel::query_builder::QueryId + diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
        <T::Table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite> {
    let mut pooled_conn = get_pooled_connection();
    diesel::delete(source)
        .execute(pooled_conn.deref_mut())
}

pub fn create<T, U>(target: T, records: U) -> QueryResult<usize>
    where
        T: Table + diesel::QuerySource,
        T::FromClause: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
        U: Insertable<T>,
        U::Values: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite> + diesel::insertable::CanInsertInSingleQuery<diesel::sqlite::Sqlite> {
    let mut pooled_conn = get_pooled_connection();
    diesel::insert_into(target)
        .values(records)
        .execute(pooled_conn.deref_mut())
}

// pub fn select_row<T, U, V>(target: T) -> diesel::helper_types::Select<V, U>
//     where
//         U: Expression,
//         V: methods::SelectDsl<U>,
//         T: Table + methods::SelectDsl<U> + Table<AllColumns = U>,
//         T::AllColumns: SelectableExpression<T>
// {
//
//     //diesel::select(T::all_columns())
//     target.select(T::all_columns())
// }
//
// pub fn read<P, T, U>(target: T, predicate: P) -> QueryResult<Vec<U>>
//     where
//         T: Table + diesel::QuerySource,
//         T::AllColumns: SelectableExpression<T>,
//         U: Expression {
//
//     let mut pooled_conn = get_pooled_connection();
//     let selection = <T as Table>::all_columns();
//     let selected = target.select(selection);
//
//     let filtered = selected.filter(predicate);
//     filtered.get_results(pooled_conn.deref_mut())
// }
