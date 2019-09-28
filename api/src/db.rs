#[database("mysql_main")]
pub struct DbConn(diesel::MysqlConnection);
