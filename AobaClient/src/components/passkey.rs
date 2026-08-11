use crate::{
	components::basic::Button,
	rpc::{
		aoba::{PasskeyCredentialCreateOptions, PasskeyRegistrationCredentials},
		get_account_rpc_client,
	},
};
use anyhow::{Result, anyhow};
use dioxus::prelude::*;
use js_sys::{
	Array, JSON, Object, Reflect, Uint8Array,
	futures::JsFuture,
	wasm_bindgen::{JsCast, JsValue},
};
use web_sys::{
	CredentialCreationOptions, DomException, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
	PublicKeyCredentialUserEntity, window,
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
		&& let Some(opts) = opts_from_rpc(req_opts)
	{
		let opts_string = JSON::stringify(&opts)
			.ok()
			.and_then(|s| s.as_string())
			.unwrap_or_default();
		info!("Opts: {:?}", opts_string);
		info!("Opts converted");
		let promise = credentials.create_with_options(&opts).map_err(js_error_stringyfy)?;
		info!("Creds created");
		let cred = JsFuture::from(promise).await.map_err(js_error_stringyfy)?;
		info!("Promise resolved");
		let cred_id = Reflect::get(&cred, &"id".into())
			.map_err(js_error_stringyfy)?
			.as_string()
			.unwrap_or_default();
		info!("credential created - {:?}", cred_id);

		let response = Reflect::get(&cred, &"response".into()).map_err(js_error_stringyfy)?;
		return Ok(PasskeyRegistrationCredentials {
			id: Reflect::get(&cred, &"id".into())
				.map_err(js_error_stringyfy)?
				.as_string()
				.unwrap_or_default(),
			raw_id: jsvalue_to_vec(&Reflect::get(&cred, &"rawId".into()).map_err(js_error_stringyfy)?),
			client_data_json: jsvalue_to_vec(
				&Reflect::get(&response, &"clientDataJSON".into()).map_err(js_error_stringyfy)?,
			),
			//Todo
			// authenticator_data: jsvalue_to_vec(&Reflect::get(&response, &"authenticatorData".into()).map_err(js_error_stringyfy)?),
			// signature: jsvalue_to_vec(&Reflect::get(&response, &"signature".into()).map_err(js_error_stringyfy)?),
			// user_handle
			..Default::default()
		});
	}
	Err(anyhow!("Failed to start credential creation"))
}

fn js_error_stringyfy(value: JsValue) -> anyhow::Error
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

fn opts_from_rpc(rpc_opts: PasskeyCredentialCreateOptions) -> Option<CredentialCreationOptions>
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
	_ = Reflect::set(&auth_selection, &"residentKey".into(), &"required".into());
	_ = Reflect::set(&auth_selection, &"userVerification".into(), &"required".into());

	_ = Reflect::set(&pub_key_opts, &"authenticatorSelection".into(), &auth_selection);
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
pub fn PasskeyLoginButton() -> Element
{
	rsx! {
		Button{
			text: "Login with Passkey"
		}
	}
}
