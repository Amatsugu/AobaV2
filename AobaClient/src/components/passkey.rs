use std::any;

use crate::{
	components::basic::Button,
	contexts::AuthContext,
	rpc::{
		aoba::{
			PasskeyAssertionOptions, PasskeyCredentialCreateOptions, PasskeyLoginRequest,
			PasskeyRegistrationCredentials,
		},
		get_account_rpc_client, get_auth_rpc_client, login,
	},
};
use anyhow::{Result, anyhow};
use dioxus::{html::KeyCode::N, prelude::*};
use js_sys::{
	Array, JSON, Object, Reflect, Uint8Array,
	futures::JsFuture,
	wasm_bindgen::{JsCast, JsValue},
};
use tonic_web_wasm_client::options;
use web_sys::{
	CredentialCreationOptions, CredentialRequestOptions, DomException, PublicKeyCredentialCreationOptions,
	PublicKeyCredentialRequestOptions, PublicKeyCredentialRpEntity, PublicKeyCredentialUserEntity, window,
};

#[component]
pub fn PasskeyRegistrationButton() -> Element
{
	rsx! {
		Button{
			text: "Register Passkey",
			onclick: move |_| {
				spawn(async {
					match start_passkey_registration().await {
						Ok(_) => info!("success"),
						Err(msg) => error!("{}", msg.to_string()),
					};
				});
			}
		}
	}
}

async fn start_passkey_registration() -> Result<(), anyhow::Error>
{
	use crate::rpc::aoba::passkey_creation_response::Result;
	let mut rpc = get_account_rpc_client();
	let response = rpc.register_passkey(()).await?.into_inner();
	let Some(opts) = response.result.map(|r| match r
	{
		Result::Options(opts) => Ok(opts),
		Result::Error(err) => Err(anyhow!("{}", err.message)),
	})
	else
	{
		return Err(anyhow!("Failed to load credential ops"));
	};

	let cred = create_credential(opts?).await?;
	rpc.complete_passkey_registration(cred).await?.into_inner();
	Ok(())
}

async fn create_credential(
	req_opts: PasskeyCredentialCreateOptions,
) -> Result<PasskeyRegistrationCredentials, anyhow::Error>
{
	if let Some(credentials) = window().map(|w| w.navigator().credentials())
		&& let Some(opts) = get_credential_creation_opts(req_opts)
	{
		let opts_string = JSON::stringify(&opts)
			.ok()
			.and_then(|s| s.as_string())
			.unwrap_or_default();
		info!("Opts: {:?}", opts_string);
		info!("Opts converted");
		let promise = credentials.create_with_options(&opts).map_err(js_error)?;
		info!("Creds created");
		let cred = JsFuture::from(promise).await.map_err(js_error)?;
		info!("Promise resolved");
		let cred_id = Reflect::get(&cred, &"id".into())
			.map_err(js_error)?
			.as_string()
			.unwrap_or_default();
		info!("credential created - {:?}", cred_id);

		let response = Reflect::get(&cred, &"response".into()).map_err(js_error)?;
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
		return Ok(PasskeyRegistrationCredentials {
			id: Reflect::get(&cred, &"id".into())
				.map_err(js_error)?
				.as_string()
				.unwrap_or_default(),
			raw_id: jsvalue_to_vec(&Reflect::get(&cred, &"rawId".into()).map_err(js_error)?),
			client_data_json: jsvalue_to_vec(&Reflect::get(&response, &"clientDataJSON".into()).map_err(js_error)?),
			//Todo
			authenticator_data: jsvalue_to_vec(
				&Reflect::get(&response, &"authenticatorData".into()).map_err(js_error)?,
			),
			signature: jsvalue_to_vec(&Reflect::get(&response, &"signature".into()).map_err(js_error)?),
			user_handle,
			// user_handle
			..Default::default()
		});
	}
	Err(anyhow!("Failed to start credential creation"))
}

fn js_error(value: JsValue) -> anyhow::Error
{
	if let Some(dom_exception) = value.dyn_ref::<DomException>()
	{
		return anyhow!("{}: {}", dom_exception.name(), dom_exception.message());
	}
	if let Some(err) = value.dyn_ref::<js_sys::Error>()
	{
		return anyhow!("{}", err.to_string());
	}
	if let Some(s) = value.as_string()
	{
		return anyhow!(s);
	}
	anyhow!(
		"{}",
		JSON::stringify(&value)
			.ok()
			.and_then(|s| s.as_string())
			.filter(|s| s != "{}")
			.unwrap_or_else(|| format!("{value:?}"))
	)
}

fn get_credential_creation_opts(rpc_opts: PasskeyCredentialCreateOptions) -> Option<CredentialCreationOptions>
{
	let Some(opt_user) = &rpc_opts.user
	else
	{
		return None;
	};
	let Some(opt_rp) = &rpc_opts.rp
	else
	{
		return None;
	};

	let opts = CredentialCreationOptions::new();
	let rp = PublicKeyCredentialRpEntity::new(&opt_rp.name);
	rp.set_id(&opt_rp.id);

	let user = PublicKeyCredentialUserEntity::new_with_u8_array(
		&opt_user.name,
		&opt_user.display_name,
		&bytes_to_uint8array(&opt_user.id),
	);
	let params = rpc_opts.pubkey_params.iter().map(|p| {
		let param = Object::new();
		_ = Reflect::set(&param, &"type".into(), &p.r#type.clone().into());
		_ = Reflect::set(&param, &"alg".into(), &JsValue::from(p.alg));
		JsValue::from(param)
	});
	let pub_key_opts = PublicKeyCredentialCreationOptions::new_with_u8_array(
		&bytes_to_uint8array(&rpc_opts.challenge),
		&Array::of(&params.collect::<Vec<JsValue>>()),
		&rp,
		&user,
	);

	let auth_selection = Object::new();
	if Reflect::set(&auth_selection, &"residentKey".into(), &"required".into()).is_err()
	{
		return None;
	}
	if Reflect::set(&auth_selection, &"userVerification".into(), &"required".into()).is_err()
	{
		return None;
	}

	if Reflect::set(&pub_key_opts, &"authenticatorSelection".into(), &auth_selection).is_err()
	{
		return None;
	}
	pub_key_opts.set_attestation(web_sys::AttestationConveyancePreference::None);
	pub_key_opts.set_timeout(60_000);
	opts.set_public_key(&pub_key_opts);
	Some(opts)
}

fn bytes_to_uint8array(bytes: &[u8]) -> Uint8Array
{
	let arr = Uint8Array::new_with_length(bytes.len() as u32);
	arr.copy_from(bytes);
	arr
}

fn jsvalue_to_vec(val: &JsValue) -> Vec<u8>
{
	Uint8Array::new(val).to_vec()
}

#[component]
pub fn PasskeyLoginButton(error: Signal<Option<String>>) -> Element
{
	let mut auth_context = use_context::<AuthContext>();
	rsx! {
		Button{
			text: "Login with Passkey",
			onclick: move |_|{
				spawn(async move {
					match start_passkey_auth().await{
						Ok(jwt) => auth_context.login(jwt),
						Err(err) => error.set(Some(err.to_string())),
					}
				});
			}
		}
	}
}

async fn start_passkey_auth() -> Result<String, anyhow::Error>
{
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

fn get_credential_request_options(
	mut assert: PasskeyAssertionOptions,
) -> Result<CredentialRequestOptions, anyhow::Error>
{
	let opts = CredentialRequestOptions::new();

	let pub_key = PublicKeyCredentialRequestOptions::new_with_u8_slice(&mut assert.challenge);
	pub_key.set_rp_id(assert.rp_id());
	pub_key.set_timeout(60000);
	pub_key.set_user_verification(web_sys::UserVerificationRequirement::Required);

	opts.set_public_key(&pub_key);

	Ok(opts)
}
