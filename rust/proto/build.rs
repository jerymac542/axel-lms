fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "contracts/proto/apexos/common/v1/common.proto",
        "contracts/proto/apexos/auth/v1/auth.proto",
        "contracts/proto/apexos/tenant/v1/tenant.proto",
        "contracts/proto/apexos/course/v1/course.proto",
        "contracts/proto/apexos/enrollment/v1/enrollment.proto",
        "contracts/proto/apexos/assignment/v1/assignment.proto",
        "contracts/proto/apexos/media/v1/media.proto",
        "contracts/proto/apexos/search/v1/search.proto",
        "contracts/proto/apexos/realtime/v1/realtime.proto",
        "contracts/proto/apexos/ai/v1/ai.proto",
        "contracts/proto/apexos/analytics/v1/analytics.proto",
        "contracts/proto/apexos/lti/v1/lti.proto",
        "contracts/proto/apexos/scorm/v1/scorm.proto",
    ];

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src")
        .compile(&protos, &["contracts/proto"])?;

    Ok(())
}