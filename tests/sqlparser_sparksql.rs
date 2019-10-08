#![feature(box_syntax, box_patterns)]

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

            // match query.body {
            //     SetExpr::Query(boxed_query) => {
            //         match *boxed_query {
            //             // SetExpr::Select(boxed_select) => {
            //             //     println!("boxed_select: {:#?}", boxed_select);
            //             // }
            //             Query {
            //                 body,
            //                 ..
            //             } => {
            //                 match body {
            //                     SetExpr::Select(boxed_s) => {
            //                         println!("boxed: {:#?}", boxed_s.projection);
            //                     },
            //                     _ => unreachable!(),
            //                 }
            //             }
            //         }
            //     },
            //     _ => unreachable!(),
            // }

            if let SetExpr::Query(box Query { body, .. }) = query.body {
                // println!("NNNNNNN");
                if let SetExpr::Select(box Select { projection, .. }) = body {
                    // println!("proj: {:#?}", projection);

                    for proj_exp in projection {
                        // println!("expr: {:#?}", expr);
                        match proj_exp {
                            SelectItem::UnnamedExpr(e) => {

                            },
                            SelectItem::ExprWithAlias{ expr, .. } => {
                                println!("expr: {:#?}", expr);
                                if let Expr::Function(Function{ name, args, .. }) = expr {
                                    println!("name: {:#?}", name);
                                    println!("args: {:#?}", args);
                                }
                            },
                            _ => unreachable!(),
                        }
                    }
                }
            } else {
                // println!("FALSE");
            }
        },
        _ => unreachable!(),
    }
}


#[test]
fn parse_spark_set() {
    let sql = "set hivevar:BLAH=2001-01-01";

    //match sparksql().verified_stmt(sql) {
    match sparksql().parse_sql_statements(sql).unwrap().pop().unwrap() {
        Statement::SetVariable {
            // local,
            variable,
            value,
            ..
        } => {
            // assert!(local);
            assert_eq!(variable, "hivevar:BLAH");
            assert_eq!(value, SetVariableValue::Literal(Value::Null));
        },
        _ => unreachable!(),
    }
}

#[test]
fn parse_spark_set_2() {
    let sql = "set hivevar:BLAH=2001-01-01;set hivevar:BLAH=2001-01-01";

    //match sparksql().verified_stmt(sql) {
    match sparksql().parse_sql_statements(sql).unwrap().pop().unwrap() {
        Statement::SetVariable {
            // local,
            variable,
            value,
            ..
        } => {
            // assert!(local);
            assert_eq!(variable, "hivevar:BLAH");
            assert_eq!(value, SetVariableValue::Literal(Value::Null));
        },
        _ => unreachable!(),
    }
}

#[test]
fn parse_sets() {
    let sql = r#"
set hivevar:BLAH1=2001-01-01;
set hivevar:BLAH2=2002-01-01;

CREATE OR REPLACE TEMPORARY VIEW v AS (
    SELECT
        cast('${hivevar:BLAH1}' AS DATE) AS BLAH_DATE1,
        cast('${hivevar:BLAH2}' AS DATE) AS BLAH_DATE2
);
"#;


    match sparksql().parse_sql_statements(sql).unwrap().pop().unwrap() {
        Statement::SetVariable {
            // local,
            variable,
            value,
            ..
        } => {
            // assert!(local);
            assert_eq!(variable, "hivevar:BLAH");
            assert_eq!(value, SetVariableValue::Literal(Value::Null));
        },
        Statement::CreateView {
            ..
        } => {},
        _ => unreachable!(),
    }

}
