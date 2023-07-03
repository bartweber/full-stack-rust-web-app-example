use static_files::{NpmBuild, resource_dir};

fn main() -> std::io::Result<()> {
    NpmBuild::new("../frontend")
        .install()?
        .run("build")?
        .target("../frontend/dist")
        .change_detection()
        .to_resource_dir()
        .build()
}
