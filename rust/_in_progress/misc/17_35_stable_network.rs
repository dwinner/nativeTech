// 17_35_stable_network.rs
#[cfg(test)]
mod stable_network_tests {
    // Simulating a function that makes a network call
    fn fetch_data_from_api_dummy(base_url: &str, endpoint: &str) -> Result<String, String> {
        // In reality, this would use reqwest or hyper
        if base_url.contains("127.0.0.1") && endpoint == "/test" {
            Ok("{\"status\": \"success from mock\"}".to_string())
        } else {
            Err("Connection failed".to_string())
        }
    }

    #[test]
    fn test_fetch_data_success_mocked() {
        // 1. Start a local mock server
        let mut server = mockito::Server::new();

        // 2. Define the expected behavior
        let mock = server.mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"status\": \"success from mock\"}")
            .create();

        // 3. Call the function using the mock server's URL
        let response = fetch_data_from_api_dummy(&server.url(), "/test");

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), "{\"status\": \"success from mock\"}");
        
        mock.assert(); // Confirm the endpoint was actually hit
    }
}