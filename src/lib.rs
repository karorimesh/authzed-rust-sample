use tonic::{
    metadata::{MetadataValue},
    transport::Channel,
    Request,
    Response
};
use v1::permissions_service_client::PermissionsServiceClient;
use v1::consistency::Requirement;
use v1::{CheckPermissionRequest, CheckPermissionResponse, Consistency, ObjectReference, SubjectReference};


#[cfg(feature = "v1")]
pub mod v1 {
    tonic::include_proto!("authzed.api.v1");
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the access token for authentication
    let access_token = "mylocal".to_string();
    let authorization_header = format!("Bearer {}", access_token);
    let authorization_header_value = MetadataValue::try_from(&authorization_header)?;

    // Set up the channel and client for the gRPC request
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = PermissionsServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", authorization_header_value.clone());
        Ok(req)
    });

    // Set up the request message and metadata
    let trsn_resource: Option<ObjectReference> = Some(ObjectReference {
        object_type: "resource/transaction".to_string(),
        object_id: "trsnc2".to_string(),
    });
    let consistency: Option<Consistency> = Some(Consistency {
        requirement: Some(Requirement::FullyConsistent(true)),
    });
    let trsn_subject: Option<SubjectReference> = Some(SubjectReference {
        object: Some(ObjectReference {
            object_type: "merchant_admin".to_string(),
            object_id: "adm2".to_string(),
        }),
        optional_relation: "".to_string(),
    });

    let request = Request::new(CheckPermissionRequest {
        consistency: consistency,
        resource: trsn_resource,
        permission: "view".to_string(),
        subject: trsn_subject,
    });

    // Send the request and process the response
    let response: Response<CheckPermissionResponse> = client.check_permission(request).await?;
    let data = response.into_inner();
    println!("Response: {:?}", data.permissionship);

    Ok(())
}
