use anyhow::Context as _;
use clap::Parser as _;
use futures_util::stream::TryStreamExt as _;
use sqlx::Column as _;
use sqlx::ConnectOptions as _;
use sqlx::Row as _;
use sqlx::TypeInfo as _;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(short, long)]
    host: String,
    #[clap(short = 'P', long)]
    port: u16,
    #[clap(short, long)]
    username: String,
    #[clap(short, long, env = "MYSQL_PWD")]
    password: String,
    #[clap(short = 'D', long)]
    database: String,
    #[clap(short, long)]
    table: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let mut conn = sqlx::mysql::MySqlConnectOptions::new()
        .host(&args.host)
        .port(args.port)
        .username(&args.username)
        .password(&args.password)
        .database(&args.database)
        .connect()
        .await
        .with_context(|| {
            format!(
                "failed to connect to mysql://{}@{}:{}/{}",
                args.username, args.host, args.port, args.database
            )
        })?;

    let query = build_select_query(&mut conn, &args.table).await?;
    let mut rows = sqlx::query(&query).fetch(&mut conn);
    while let Some(row) = rows
        .try_next()
        .await
        .with_context(|| format!("failed to read row from {}", args.table))?
    {
        let record = to_json(&row).context("failed to convert to JSON from MySQL row")?;
        println!(
            "{}",
            serde_json::to_string(&record).context("failed to serialize row data into JSON")?
        );
    }

    Ok(())
}

async fn build_select_query<'c, E>(executor: E, table: &str) -> anyhow::Result<String>
where
    E: sqlx::Executor<'c, Database = sqlx::MySql>,
{
    let test_query = format!("select * from `{}` limit 1", table);
    let row = sqlx::query(&test_query)
        .fetch_optional(executor)
        .await
        .with_context(|| format!("failed to fetch test query result from table: {}", table))?;
    if let Some(row) = row {
        let mut column_names = Vec::with_capacity(row.columns().len());
        for col in row.columns() {
            let name = if col.type_info().name() == "GEOMETRY" {
                // XXX: Redshift doesn't support loading geometry value with JSON format.
                // https://docs.aws.amazon.com/redshift/latest/dg/copy-usage_notes-spatial-data.html
                // So dump geometry data as text format
                format!("ST_AsText(`{}`) as `{}`", col.name(), col.name())
            } else {
                format!("`{}`", col.name())
            };
            column_names.push(name);
        }
        Ok(format!(
            "select {} from `{}`",
            column_names.join(", "),
            table
        ))
    } else {
        // No rows
        Ok(format!("select * from `{}`", table))
    }
}

fn to_json(row: &sqlx::mysql::MySqlRow) -> anyhow::Result<serde_json::Value> {
    let mut record = serde_json::Map::with_capacity(row.columns().len());
    for col in row.columns() {
        let json_value = match col.type_info().name() {
            "BOOLEAN" => {
                let val: Option<bool> = row.try_get(col.ordinal()).with_context(|| {
                    format!("failed to deserialize row data of {} column", col.name())
                })?;
                val.map(serde_json::Value::from)
            }
            "TINYINT" | "SMALLINT" | "MEDIUMINT" | "INT" | "BIGINT" => {
                let val: Option<i64> = row.try_get(col.ordinal()).with_context(|| {
                    format!("failed to deserialize row data of {} column", col.name())
                })?;
                val.map(serde_json::Value::from)
            }
            "TINYINT UNSIGNED" | "SMALLINT UNSIGNED" | "MEDIUMINT UNSIGNED" | "INT UNSIGNED"
            | "BIGINT UNSIGNED" => {
                let val: Option<u64> = row.try_get(col.ordinal()).with_context(|| {
                    format!("failed to deserialize row data of {} column", col.name())
                })?;
                val.map(serde_json::Value::from)
            }
            "FLOAT" => {
                let val: Option<f32> = row.try_get(col.ordinal()).with_context(|| {
                    format!("failed to deserialize row data of {} column", col.name())
                })?;
                val.map(serde_json::Value::from)
            }
            "DOUBLE" => {
                let val: Option<f64> = row.try_get(col.ordinal()).with_context(|| {
                    format!("failed to deserialize row data of {} column", col.name())
                })?;
                val.map(serde_json::Value::from)
            }
            "DATE" => {
                let val: Option<chrono::NaiveDate> =
                    row.try_get(col.ordinal()).with_context(|| {
                        format!("failed to deserialize row data of {} column", col.name())
                    })?;
                val.map(|d| serde_json::Value::from(d.format("%Y-%m-%d").to_string()))
            }
            "TIME" => {
                let val: Option<chrono::NaiveTime> =
                    row.try_get(col.ordinal()).with_context(|| {
                        format!("failed to deserialize row data of {} column", col.name())
                    })?;
                val.map(|t| serde_json::Value::from(t.format("%H:%M:%S").to_string()))
            }
            "DATETIME" => {
                let val: Option<chrono::NaiveDateTime> =
                    row.try_get(col.ordinal()).with_context(|| {
                        format!("failed to deserialize row data of {} column", col.name())
                    })?;
                val.map(|t| serde_json::Value::from(t.format("%Y-%m-%d %H:%M:%S").to_string()))
            }
            "TIMESTAMP" => {
                let val: Option<chrono::DateTime<chrono::Utc>> =
                    row.try_get(col.ordinal()).with_context(|| {
                        format!("failed to deserialize row data of {} column", col.name())
                    })?;
                val.map(|t| serde_json::Value::from(t.format("%Y-%m-%d %H:%M:%S").to_string()))
            }
            "CHAR" | "VARCHAR" | "ENUM" | "TINYTEXT" | "TEXT" | "MEDIUMTEXT" | "LONGTEXT" => {
                let val: Option<String> = row.try_get(col.ordinal()).with_context(|| {
                    format!("failed to deserialize row data of {} column", col.name())
                })?;
                val.map(serde_json::Value::from)
            }
            "JSON" => {
                let val: Option<serde_json::Value> =
                    row.try_get(col.ordinal()).with_context(|| {
                        format!("failed to deserialize row data of {} column", col.name())
                    })?;
                val
            }
            "BINARY" | "VARBINARY" | "TINYBLOB" | "BLOB" | "MEDIUMBLOB" | "LONGBLOB" => {
                // XXX: Redshift doesn't support loading varbyte value with JSON format.
                // https://docs.aws.amazon.com/redshift/latest/dg/copy-usage-varbyte.html
                None
            }
            "GEOMETRY" => {
                panic!("BUG: GEOMETRY type must be converted with ST_AsText()")
            }
            type_name => anyhow::bail!(
                "Unsupported MySQL data type is found in {} column: {}",
                col.name(),
                type_name
            ),
        };
        record.insert(
            // All Redshift column names are lower case.
            col.name().to_ascii_lowercase(),
            json_value.unwrap_or(serde_json::Value::Null),
        );
    }
    Ok(serde_json::Value::Object(record))
}

#[cfg(test)]
mod tests {
    use sqlx::ConnectOptions as _;
    use sqlx::Executor as _;

    #[tokio::test]
    async fn it_works() {
        let mut conn = sqlx::mysql::MySqlConnectOptions::new()
            .host("localhost")
            .port(3306)
            .username(std::env::var("MYSQL_USER").as_ref().unwrap())
            .password(std::env::var("MYSQL_PWD").as_ref().unwrap())
            .database(std::env::var("MYSQL_DATABASE").as_ref().unwrap())
            .connect()
            .await
            .unwrap();
        conn.execute(include_str!("./test_setup.sql"))
            .await
            .unwrap();

        const TABLE: &str = "tests";
        let query = super::build_select_query(&mut conn, TABLE).await.unwrap();
        let row = sqlx::query(&query).fetch_one(&mut conn).await.unwrap();
        let mut record = super::to_json(&row).unwrap();
        let record = record.as_object_mut().unwrap();

        // numeric data types
        assert_eq!(record.remove("col_boolean"), Some(serde_json::json!(true)));
        assert_eq!(record.remove("col_tinyint"), Some(serde_json::json!(2i64)));
        assert_eq!(record.remove("col_smallint"), Some(serde_json::json!(3i64)));
        assert_eq!(
            record.remove("col_mediumint"),
            Some(serde_json::json!(4i64))
        );
        assert_eq!(record.remove("col_int"), Some(serde_json::json!(5i64)));
        assert_eq!(record.remove("col_bigint"), Some(serde_json::json!(6i64)));
        assert_eq!(record.remove("col_float"), Some(serde_json::json!(7.1f32)));
        assert_eq!(record.remove("col_double"), Some(serde_json::json!(8.2f64)));
        assert_eq!(record.remove("col_utinyint"), Some(serde_json::json!(9u64)));
        assert_eq!(
            record.remove("col_usmallint"),
            Some(serde_json::json!(10u64))
        );
        assert_eq!(
            record.remove("col_umediumint"),
            Some(serde_json::json!(11u64))
        );
        assert_eq!(record.remove("col_uint"), Some(serde_json::json!(12u64)));
        assert_eq!(record.remove("col_ubigint"), Some(serde_json::json!(13u64)));
        assert_eq!(
            record.remove("col_ufloat"),
            Some(serde_json::json!(14.3f32))
        );
        assert_eq!(
            record.remove("col_udouble"),
            Some(serde_json::json!(15.4f64))
        );

        // date and time data types
        assert_eq!(
            record.remove("col_date"),
            Some(serde_json::json!("2022-05-19"))
        );
        assert_eq!(
            record.remove("col_time"),
            Some(serde_json::json!("01:52:06"))
        );
        assert_eq!(
            record.remove("col_datetime"),
            Some(serde_json::json!("2022-05-19 01:53:32"))
        );
        assert_eq!(
            record.remove("col_timestamp"),
            Some(serde_json::json!("2022-05-19 01:54:11"))
        );

        // string data types (binary types are not supported)
        assert_eq!(record.remove("col_char"), Some(serde_json::json!("20")));
        assert_eq!(record.remove("col_varchar"), Some(serde_json::json!("21")));
        assert_eq!(record.remove("col_binary"), Some(serde_json::json!(null)));
        assert_eq!(
            record.remove("col_varbinary"),
            Some(serde_json::json!(null))
        );
        assert_eq!(record.remove("col_tinyblob"), Some(serde_json::json!(null)));
        assert_eq!(record.remove("col_blob"), Some(serde_json::json!(null)));
        assert_eq!(
            record.remove("col_mediumblob"),
            Some(serde_json::json!(null))
        );
        assert_eq!(record.remove("col_longblob"), Some(serde_json::json!(null)));
        assert_eq!(record.remove("col_tinytext"), Some(serde_json::json!("28")));
        assert_eq!(record.remove("col_text"), Some(serde_json::json!("29")));
        assert_eq!(
            record.remove("col_mediumtext"),
            Some(serde_json::json!("30"))
        );
        assert_eq!(record.remove("col_longtext"), Some(serde_json::json!("31")));
        assert_eq!(record.remove("col_enum"), Some(serde_json::json!("e2")));
        assert_eq!(record.remove("col_set"), Some(serde_json::json!("s1,s3")));

        // spatial data types (converted to text format)
        assert_eq!(
            record.remove("col_geometry"),
            Some(serde_json::json!("POINT(34 0)"))
        );

        // JSON data types
        assert_eq!(
            record.remove("col_json"),
            Some(serde_json::json!({ "values": [35] }))
        );
    }
}
