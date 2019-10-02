use sqlparser::ast::*;
use sqlparser::dialect::{GenericDialect, SparkSqlDialect};
use sqlparser::parser::ParserError;
use sqlparser::test_utils::*;

fn sparksql() -> TestedDialects {
    TestedDialects {
        dialects: vec![Box::new(SparkSqlDialect {})],
    }
}

#[test]
fn parse_create_or_replace_temporary_view() {
    let sql = "CREATE OR REPLACE TEMPORARY VIEW v AS SELECT foo FROM bar";

    match sparksql().verified_stmt(sql) {
        Statement::CreateView {
            name,
            columns,
            query,
            materialized,
            do_replace,
            temporary,
            with_options,
        } => {
            assert_eq!("v", name.to_string());
            assert_eq!(Vec::<Ident>::new(), columns);
            assert_eq!("SELECT foo FROM bar", query.to_string());
            assert!(!materialized);
            assert!(do_replace);
            assert_eq!(temporary, TemporaryOption::Long);
            assert_eq!(with_options, vec![]);
        },
        _ => unreachable!(),
    }
}

#[test]
fn parse_create_or_replace_temp_view() {
    let sql = "CREATE OR REPLACE TEMP VIEW v AS SELECT foo FROM bar";

    match sparksql().verified_stmt(sql) {
        Statement::CreateView {
            name,
            columns,
            query,
            materialized,
            do_replace,
            temporary,
            with_options,
        } => {
            assert_eq!("v", name.to_string());
            assert_eq!(Vec::<Ident>::new(), columns);
            assert_eq!("SELECT foo FROM bar", query.to_string());
            assert!(!materialized);
            assert!(do_replace);
            assert_eq!(temporary, TemporaryOption::Short);
            assert_eq!(with_options, vec![]);
        },
        _ => unreachable!(),
    }
}


#[test]
fn parse_create_or_replace_temporary_view_named_struct() {
    let sql = "CREATE OR REPLACE TEMPORARY VIEW v AS (
SELECT ID, 
named_struct (
'KEY1', 'VALUE1', 
'KEY2', 'VALUE2'
) as NS
FROM fake_table)";

    match sparksql().parse_sql_statements(sql).unwrap().pop().unwrap() {
        Statement::CreateView {
            name,
            columns,
            query,
            materialized,
            do_replace,
            temporary,
            with_options,
            ..
        } => {
            assert_eq!("v", name.to_string());
            assert_eq!(Vec::<Ident>::new(), columns);
            // assert_eq!("SELECT foo FROM bar", query.to_string());
            assert!(!materialized);
            assert!(do_replace);
            assert_eq!(temporary, TemporaryOption::Long);
            assert_eq!(with_options, vec![]);

            //sparksql().parse_sql_statements(*query).unwrap();
            // sparksql().verified_query(&query.to_string());

            println!("query.body: {:#?}", query.body);

            match query.body {
                SetExpr::Query(boxed_query) => {
                    match *boxed_query {
                        // SetExpr::Select(boxed_select) => {
                        //     println!("boxed_select: {:#?}", boxed_select);
                        // }
                        Query {
                            body,
                            ..
                        } => {
                            match body {
                                SetExpr::Select(boxed_s) => {
                                    println!("boxed: {:#?}", boxed_s.projection);
                                },
                                _ => unreachable!(),
                            }
                        }
                    }
                },
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}
