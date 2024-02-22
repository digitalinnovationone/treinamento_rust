
#[cfg(test)]
mod tests {
    use tokio;
    
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct ApiResponse {
        pub valido: bool,
    }

    #[tokio::test]
    async fn test_index_endpoint() {
        let client = reqwest::Client::new();
        let res = client.get("http://localhost:8000/")
            .send()
            .await
            .expect("Failed to send request");
        assert!(res.status().is_success());
        let body = res.text().await.expect("Failed to read response text");
        assert!(body.contains("Api valida CPF"));
    }

    #[tokio::test]
    async fn test_valida_cpf_endpoint() {
        let client = reqwest::Client::new();
        let cpf = "12345678909"; // Use um CPF que você espera que seja válido/inválido conforme o teste
        let url = format!("http://localhost:8000/valida_cpf?cpf={}", cpf);
        let res = client.get(&url)
            .send()
            .await
            .expect("Failed to send request");
        assert!(res.status().is_success());
        let json: ApiResponse = res.json().await.expect("Failed to parse JSON");
        assert_eq!(json.valido, true); // ou false, conforme esperado
    }
}
