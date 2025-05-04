fn main() -> Result<(), Box<dyn std::error::Error>> {
	tonic_build::configure()
		.build_server(false)
		.build_client(true)
		.compile_protos(
			&["..\\AobaServer\\Proto\\Aoba.proto", "..\\AobaServer\\Proto\\Auth.proto"],
			&["..\\AobaServer\\Proto\\"],
		)?;

	Ok(())
}
