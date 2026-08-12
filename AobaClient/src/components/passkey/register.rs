use crate::{
	components::{
		basic::Button,
		passkey::prelude::{bytes_to_uint8array, js_error, jsvalue_to_vec},
	},
	rpc::{
		aoba::{PasskeyCredentialCreateOptions, PasskeyRegistrationCredentials},
		get_account_rpc_client,
	},
};
use anyhow::{Result, anyhow};
use dioxus::prelude::*;
use js_sys::{Array, Object, Reflect, futures::JsFuture, wasm_bindgen::JsValue};
use web_sys::{CredentialCreationOptions, PublicKeyCredentialUserEntity, window};
use web_sys::{PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity};

#[component]
pub fn PasskeyRegistrationButton() -> Element
{
	let mut disabled = use_signal(|| false);
	rsx! {
		Button{
			text: "Register Passkey",
			disabled: disabled(),
			onclick: move |_| {
				disabled.set(true);
				spawn(async move {
					match start_passkey_registration().await {
						Ok(_) => info!("success"),
						Err(msg) => error!("{}", msg.to_string()),
					};
					disabled.set(false);
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
		let promise = credentials.create_with_options(&opts).map_err(js_error)?;
		let cred = JsFuture::from(promise).await.map_err(js_error)?;

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
			attestation_object: jsvalue_to_vec(
				&Reflect::get(&response, &"attestationObject".into()).map_err(js_error)?,
			),
			authenticator_data: jsvalue_to_vec(
				&Reflect::get(&response, &"authenticatorData".into()).map_err(js_error)?,
			),
			signature: jsvalue_to_vec(&Reflect::get(&response, &"signature".into()).map_err(js_error)?),
			user_handle,
		});
	}
	Err(anyhow!("Failed to start credential creation"))
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
