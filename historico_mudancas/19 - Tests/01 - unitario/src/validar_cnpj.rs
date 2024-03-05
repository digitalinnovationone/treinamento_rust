// Prompt chat GPT = https://chat.openai.com/share/66823aa7-99ac-4e9f-9aa9-704ea68a7b46

pub fn validar(cnpj: &str) -> bool {
    let cnpj: Vec<u8> = cnpj
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    if cnpj.len() != 14 || cnpj.iter().all(|&digit| digit == cnpj[0]) {
        return false;
    }

    let mut soma = 0;
    let mut peso = 5;

    for i in 0..12 {
        soma += cnpj[i] as i32 * peso;
        peso = if peso == 2 { 9 } else { peso - 1 };
    }

    let resto = soma % 11;
    let primeiro_digito = if resto < 2 { 0 } else { 11 - resto };

    if primeiro_digito != cnpj[12] as i32 {
        return false;
    }

    soma = 0;
    peso = 6;

    for i in 0..13 {
        soma += cnpj[i] as i32 * peso;
        peso = if peso == 2 { 9 } else { peso - 1 };
    }

    let resto = soma % 11;
    let segundo_digito = if resto < 2 { 0 } else { 11 - resto };

    segundo_digito == cnpj[13] as i32
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cnpj_valido(){
        // Arrage
        let cnpj = "31.065.853/0001-35";

        // Act
        let resultado = validar(cnpj);

        // Assert
        assert!(resultado) // verdadeiro
    }

    #[test]
    fn test_cnpj_valido_sem_formatacao(){
        // Arrage
        let cnpj = "31065853000135";

        // Act
        let resultado = validar(cnpj);

        // Assert
        assert!(resultado) // verdadeiro
    }

    #[test]
    fn test_cnpj_invalido(){
        // Arrage
        let cnpj = "31.065.853/0001-37";

        // Act
        let resultado = validar(cnpj);

        // Assert
        assert!(!resultado) // false
    }

    #[test]
    fn test_cnpj_invalido_sem_formatacao(){
        // Arrage
        let cnpj = "31065853000137";

        // Act
        let resultado = validar(cnpj);

        // Assert
        assert!(!resultado) // false
    }

    #[test]
    fn test_cnpj_vazio(){
        // Arrage
        let cnpj = "";

        // Act
        let resultado = validar(cnpj);

        // Assert
        assert!(!resultado) // false
    }

    #[test]
    fn test_cnpj_00000000000000(){
        // Arrage
        let cnpj = "00000000000000";

        // Act
        let resultado = validar(cnpj);

        // Assert
        assert!(!resultado) // false
    }

    #[test]
    fn test_cnpj_11111111111111(){
        // Arrage
        let cnpj = "11111111111111";

        // Act
        let resultado = validar(cnpj);

        // Assert
        assert!(!resultado) // false
    }

    #[test]
    fn test_cnpj_10000000000000(){
        // Arrage
        let cnpj = "10000000000000";

        // Act
        let resultado = validar(cnpj);

        // Assert
        assert!(!resultado) // false
    }
}
