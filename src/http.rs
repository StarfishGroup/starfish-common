use anyhow::{ensure, Result};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

pub async fn get_json<P: AsRef<str>>(
	path: P,
	headers: Vec<(String, String)>,
	params: Vec<(String, String)>,
) -> Result<Value> {
	let params = serde_urlencoded::to_string(params)?;
	let url = if params.len() == 0 {
		format!("{}", path.as_ref())
	} else {
		format!("{}?{}", path.as_ref(), params)
	};
	let mut request = Client::new().get(url);
	for (key, value) in headers {
		request = request.header(key, value);
	}
	let response = request.send().await?;
	ensure!(response.status().as_u16() == 200, "response error : {}", response.status());
	let data: Value = response.json().await?;
	return Ok(data);
}

pub async fn post_json<P: AsRef<str>, B: Serialize + ?Sized>(
	path: P,
	headers: Vec<(String, String)>,
	body: &B,
) -> Result<Value> {
	let mut request = Client::new().post(path.as_ref()).json(body);
	for (key, value) in headers {
		request = request.header(key, value);
	}
	let response = request.send().await?;
	ensure!(response.status().as_u16() == 200, "response error : {}", response.status());
	let data: Value = response.json().await?;
	Ok(data)
}
