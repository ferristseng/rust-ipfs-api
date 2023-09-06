use ipfs_api::IpfsApi;

// If this compiles, the test has passed, as unwrap() requires the Debug trait.
#[allow(unused)]
async fn test_use_client<C: IpfsApi>(client: &C) {
    // Validate that all variants of the Backend trait's Error type (Backend::Error) implement the Debug trait.
    client.version().await.unwrap();
}
