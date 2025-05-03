fn main() -> Result<(), Box<dyn std::error::Error>> {
	tonic_build::configure()
		.build_server(false)
		.compile_protos(&["..\\AobaServer\\Proto\\Aoba.proto"], &["..\\AobaServer\\Proto\\"])?;

	Ok(())
}
