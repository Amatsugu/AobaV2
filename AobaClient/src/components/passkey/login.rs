use crate::{
	components::{
		basic::Button,
		passkey::prelude::{bytes_to_uint8array, js_error, jsvalue_to_vec},
	},
	contexts::AuthContext,
	rpc::{
		aoba::{PasskeyAssertionOptions, PasskeyLoginRequest},
		get_auth_rpc_client,
	},
};
use anyhow::{Result, anyhow};
use dioxus::prelude::*;
use js_sys::{Reflect, futures::JsFuture};
use web_sys::{CredentialRequestOptions, PublicKeyCredentialRequestOptions, window};

#[component]
pub fn PasskeyLoginButton(error: Signal<Option<String>>) -> Element
{
	let mut auth_context = use_context::<AuthContext>();
	let mut disabled = use_signal(|| false);
	rsx! {
		Button{
			text: "Login with Passkey",
			disabled: disabled(),
			onclick: move |_|{
				disabled.set(true);
				spawn(async move {
					match start_passkey_auth().await{
						Ok(jwt) => auth_context.login(jwt),
						Err(err) => error.set(Some(err.to_string())),
					}
					disabled.set(false);
				});
			}
		}
	}
}

async fn start_passkey_auth() -> Result<String, anyhow::Error>
{
	info!("Passkey Auth Start");
	let mut rpc = get_auth_rpc_client();
	let Some(opts) = rpc.get_assertion_options(()).await.map(|r| {
		r.into_inner().result.map(|r| {
			use crate::rpc::aoba::passkey_assertion_response::Result;
			match r
			{
				Result::Options(opts) => Ok(opts),
				Result::ErrorMessage(err) => Err(anyhow!("{}", err)),
			}
		})
	})?
	else
	{
		return Err(anyhow!("Failed to get assertion options"));
	};

	let login_request = authenticate_passkey(opts?).await?;
	info!(
		"Sending Login: {}",
		login_request.ceremony_id.clone().unwrap_or_default().value
	);
	let result = rpc.login_passkey(login_request).await?.into_inner().result;

	let Some(result) = result
	else
	{
		return Err(anyhow!("Failed to complete login"));
	};
	use crate::rpc::aoba::login_response::Result;
	match result
	{
		Result::Jwt(jwt) => Ok(jwt.token),
		Result::Error(login_error) => Err(anyhow!(login_error.message)),
	}
}

async fn authenticate_passkey(opts: PasskeyAssertionOptions) -> Result<PasskeyLoginRequest, anyhow::Error>
{
	let Some(credentials) = window().map(|w| w.navigator().credentials())
	else
	{
		return Err(anyhow!("Failed to get Credentials Container"));
	};
	let ceremony_id = opts.ceremony_id.clone();

	let result = credentials
		.get_with_options(&get_credential_request_options(opts)?)
		.map_err(js_error)?;

	let credentials = JsFuture::from(result).await.map_err(js_error)?;
	let response = Reflect::get(&credentials, &"response".into()).map_err(js_error)?;
	let user_handle = Reflect::get(&response, &"userHandle".into())
		.map_err(js_error)
		.map(|h| {
			if h.is_null() || h.is_undefined()
			{
				None
			}
			else
			{
				Some(jsvalue_to_vec(&h))
			}
		})?;

	Ok(PasskeyLoginRequest {
		id: Reflect::get(&credentials, &"id".into())
			.map_err(js_error)?
			.as_string()
			.unwrap_or_default(),
		raw_id: jsvalue_to_vec(&Reflect::get(&credentials, &"rawId".into()).map_err(js_error)?),
		client_data_json: jsvalue_to_vec(&Reflect::get(&response, &"clientDataJSON".into()).map_err(js_error)?),
		authenticator_data: jsvalue_to_vec(&Reflect::get(&response, &"authenticatorData".into()).map_err(js_error)?),
		signature: jsvalue_to_vec(&Reflect::get(&response, &"signature".into()).map_err(js_error)?),
		ceremony_id,
		user_handle,
	})
}

fn get_credential_request_options(assert: PasskeyAssertionOptions) -> Result<CredentialRequestOptions, anyhow::Error>
{
	let opts = CredentialRequestOptions::new();

	info!("Challenge: {:?}", assert.challenge);
	let pub_key = PublicKeyCredentialRequestOptions::new_with_u8_array(&bytes_to_uint8array(&assert.challenge));
	pub_key.set_rp_id(assert.rp_id());
	pub_key.set_timeout(60000);
	pub_key.set_user_verification(web_sys::UserVerificationRequirement::Required);

	opts.set_public_key(&pub_key);

	Ok(opts)
}
