#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8001")?;

	// hc.do_get("/index.html").await?.print().await?;

	// Example for login.
	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "user1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	// Example for create_tasks for rpc-json.
	let req_create_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_task",
			"params": {
				"data": {
					"title": "task AAA"
				}
			}
		}),
	);
	// req_create_task.await?.print().await?;

	// Example for update_tasks for rpc-json.
	let req_update_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_task",
			"params": {
				"id": 1001, // Hardcore the task id.
				"data": {
					"title": "task CCC"
				}
			}
		}),
	);
	req_update_task.await?.print().await?;

	// Example for delete_tasks for rpc-json.
	let req_deletete_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_task",
			"params": {
				"id": 1001, // Hardcore the task id.
			}
		}),
	);
	// req_deletete_task.await?.print().await?;

	// Example for list_tasks for rpc-json.
	let req_list_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks",
		}),
	);
	req_list_tasks.await?.print().await?;

	// Example for logoff.
	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	// req_logoff.await?.print().await?;

	// hc.do_get("/hello").await?.print().await?;

	Ok(())
}
