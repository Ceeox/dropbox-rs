#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Token
{
	code: String,
	grant_type: String,
	client_id: String,
	redirect_uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenResponse
{
	access_token: String,
	token_type: String,
	account_id: String,
	uid: Option<String>,
}
