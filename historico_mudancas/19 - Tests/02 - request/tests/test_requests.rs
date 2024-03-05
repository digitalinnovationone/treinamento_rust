
#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct ApiResponse {
        pub valido: bool,
    }

    #[tokio::test]
    async fn test_home() {
        let client = reqwest::Client::new();
        let res = client.get("http://localhost:8000/")
            .send()
            .await
            .expect("Failed to send request");
        assert!(res.status().is_success());
        let body = res.text().await.expect("Failed to read response text");
        assert_eq!("{\"mensagem\":\"Api para validar CNPJ - /validar_cnpj?cnpj=123567\"}", body);
    }

    #[tokio::test]
    async fn test_valida_cnpj_endpoint() {
        // Arrange
        let client = reqwest::Client::new();
        let cnpj = "31065853000135";
        let url = format!("http://localhost:8000/validar_cnpj?cnpj={}", cnpj);

        // Act
        let res = client.get(&url)
            .send()
            .await
            .expect("Failed to send request");

        // Assert
        assert!(res.status().is_success());
        let json: ApiResponse = res.json().await.expect("Failed to parse JSON");
        assert_eq!(json.valido, true); // ou false, conforme esperado
    }

    #[tokio::test]
    async fn test_valida_cnpj_invalido_endpoint() {
        // Arrange
        let client = reqwest::Client::new();
        let cnpj = "31065853000131";
        let url = format!("http://localhost:8000/validar_cnpj?cnpj={}", cnpj);

        // Act
        let res = client.get(&url)
            .send()
            .await
            .expect("Failed to send request");

        // Assert
        assert!(res.status().is_success());
        let json: ApiResponse = res.json().await.expect("Failed to parse JSON");
        assert_eq!(json.valido, false); // ou false, conforme esperado
    }
}
