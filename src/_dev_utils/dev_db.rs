use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

use crate::{
	ctx::Ctx,
	model::{
		user::{User, UserBmc},
		ModelManager,
	},
};

type Db = Pool<Postgres>;

// NOTE: Hardcode to prevent deployed system db update.
const PG_DEV_POSTGRES_URL: &str =
	"postgresql://postgres:welcome@localhost:5432/postgres";
const PG_DEV_APP_URL: &str =
	"postgresql://app_user:dev_only_pwd@localhost:5432/app_db";

// SQL Files.
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-reacreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const USER_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
	info!("{:<12} - init_dev_db", "FOR-DEV-ONLY");

	// -- Create the app db/app_user with postgres user.
	{
		let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
		pexec(&root_db, SQL_RECREATE_DB).await?;
	}

	// -- Get SQL files.
	let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
		.filter_map(|entry| entry.ok().map(|e| e.path()))
		.collect();
	// -- Sort the paths.
	paths.sort();

	// -- SQL Execute each file.
	let app_db = new_db_pool(PG_DEV_APP_URL).await?;
	for path in paths {
		if let Some(path) = path.to_str() {
			let path = path.replace('\\', "/"); // For Windows.

			// Only take .sql and skip the SQL_RECREATE_DB.
			if path.ends_with(".sql") && path != SQL_RECREATE_DB {
				pexec(&app_db, &path).await?;
			}
		}
	}

	// -- Init model layer.
	let mm = ModelManager::new().await?;
	let ctx = Ctx::root_ctx();

	// -- Set user1 pwd.
	let user1_user: User = UserBmc::first_by_username(&ctx, &mm, "user1")
		.await?
		.unwrap();
	UserBmc::update_pwd(&ctx, &mm, user1_user.id, USER_PWD).await?;
	info!("{:<12} - init_dev_db - set user1 pwd", "FOR-DEV-ONLY" );

	Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
	info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

	// - Read the file.
	let content = fs::read_to_string(file)?;

	// FIXME: Make the split more sql proof.
	let sqls: Vec<&str> = content.split(';').collect();

	for sql in sqls {
		sqlx::query(sql).execute(db).await?;
	}

	Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(500))
		.connect(db_con_url)
		.await
}
