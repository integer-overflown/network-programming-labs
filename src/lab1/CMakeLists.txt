add_subdirectory(client)

corrosion_import_crate(MANIFEST_PATH server/Cargo.toml)
add_custom_target(run-server COMMAND lab1-server)
