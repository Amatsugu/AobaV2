use anyhow::anyhow;
use js_sys::{
	JSON, Uint8Array,
	wasm_bindgen::{JsCast, JsValue},
};
use web_sys::DomException;

pub fn js_error(value: JsValue) -> anyhow::Error
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
pub fn bytes_to_uint8array(bytes: &[u8]) -> Uint8Array
{
	let arr = Uint8Array::new_with_length(bytes.len() as u32);
	arr.copy_from(bytes);
	arr
}

pub fn jsvalue_to_vec(val: &JsValue) -> Vec<u8>
{
	Uint8Array::new(val).to_vec()
}
