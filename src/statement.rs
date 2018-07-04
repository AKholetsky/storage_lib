use input_buffer::InputBuffer;

enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub enum PreparedResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

pub struct Statement {
    statement_type: StatementType,
}

fn insert_statement() -> Option<Statement> {
    Some(Statement {
        statement_type: StatementType::StatementInsert
    })
}

fn select_statement() -> Option<Statement> {
    Some(Statement {
        statement_type: StatementType::StatementSelect
    })
}

pub fn prepare_statement (input_buffer: &InputBuffer) -> (PreparedResult, Option<Statement>) {
    match input_buffer.print_buffer() {
        insert if insert.starts_with("insert") => (PreparedResult::PrepareSuccess, insert_statement()),
        select if select.starts_with("select") => (PreparedResult::PrepareSuccess, select_statement()),
        _ => (PreparedResult::PrepareUnrecognizedStatement, None)
    }
} 

pub fn execute_statement(st: Option<Statement>) {
    match st {
        Some(value) => {
            match value.statement_type {
                StatementType::StatementInsert => {
                    println!("Statement insert");
                },
                StatementType::StatementSelect => {
                    println!("Statement select");
                }
            }
        },
        None => println!("None statement"),
    }
}