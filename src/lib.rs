use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};
use std::collections::HashMap;

hayashi_plugin!();

/// 1. bcb_selic(series_code, start_date, end_date)
/// Get Selic rate from BCB (Banco Central do Brasil)
/// series_code: BCB time series code (e.g., 432 for Selic meta)
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn bcb_selic(_series_code: i64, _start_date: String, _end_date: String) -> HashMap<String, Vec<f64>> {
    // Simplified implementation - returns mock data
    // In production, would use BCB API: https://api.bcb.gov.br/dados/serie/bcdata.sgs.{code}/dados
    let n_days = 30; // Mock: 30 days of data
    let dates: Vec<String> = (0..n_days).map(|i| {
        // Generate mock dates
        format!("2024-01-{:02}", i + 1)
    }).collect();
    
    let values: Vec<f64> = (0..n_days).map(|i| {
        // Mock Selic rates around 10.5%
        10.5 + (i as f64 * 0.01 - 0.15)
    }).collect();
    
    let mut result = HashMap::new();
    result.insert("date".to_string(), dates.iter().map(|d| d.parse::<f64>().unwrap_or(0.0)).collect());
    result.insert("value".to_string(), values);
    result
}

/// 2. bcb_pib(series_code, start_date, end_date)
/// Get GDP (PIB) data from BCB
/// series_code: BCB time series code (e.g., 21911 for PIB acumulado)
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn bcb_pib(_series_code: i64, _start_date: String, _end_date: String) -> HashMap<String, Vec<f64>> {
    // Simplified implementation - returns mock quarterly GDP data
    let n_quarters = 8; // Mock: 8 quarters of data
    let quarters: Vec<String> = (0..n_quarters).map(|i| {
        format!("2024-Q{}", (i % 4) + 1)
    }).collect();
    
    let values: Vec<f64> = (0..n_quarters).map(|i| {
        // Mock GDP in billions BRL, growing trend
        1000.0 + (i as f64 * 50.0)
    }).collect();
    
    let mut result = HashMap::new();
    result.insert("quarter".to_string(), quarters.iter().map(|q| q.parse::<f64>().unwrap_or(0.0)).collect());
    result.insert("pib_brl".to_string(), values);
    result
}

/// 3. bcb_reservas_internacionais(start_date, end_date)
/// Get international reserves from BCB
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn bcb_reservas_internacionais(_start_date: String, _end_date: String) -> HashMap<String, Vec<f64>> {
    // Simplified implementation - returns mock reserves data
    let n_days = 30;
    let dates: Vec<String> = (0..n_days).map(|i| {
        format!("2024-01-{:02}", i + 1)
    }).collect();
    
    let values: Vec<f64> = (0..n_days).map(|i| {
        // Mock reserves around $350 billion USD
        350.0 + (i as f64 * 0.1 - 1.5)
    }).collect();
    
    let mut result = HashMap::new();
    result.insert("date".to_string(), dates.iter().map(|d| d.parse::<f64>().unwrap_or(0.0)).collect());
    result.insert("reserves_usd".to_string(), values);
    result
}

/// 4. ibge_pib_municipal(uf, year)
/// Get municipal GDP data from IBGE
/// uf: state code (e.g., 43 for RS)
/// year: year of data
#[hayashi_fn]
pub fn ibge_pib_municipal(_uf: i64, _year: i64) -> HashMap<String, Vec<f64>> {
    // Simplified implementation - returns mock municipal GDP data
    let n_municipalities = 10; // Mock: 10 municipalities
    let municipalities: Vec<String> = (0..n_municipalities).map(|i| {
        format!("Municipio_{}", i + 1)
    }).collect();
    
    let values: Vec<f64> = (0..n_municipalities).map(|i| {
        // Mock GDP in millions BRL
        100.0 + (i as f64 * 50.0)
    }).collect();
    
    let mut result = HashMap::new();
    result.insert("municipality".to_string(), municipalities.iter().map(|m| m.parse::<f64>().unwrap_or(0.0)).collect());
    result.insert("pib_milhoes".to_string(), values);
    result
}

/// 5. ibge_inflacao_ipc_a12(start_date, end_date)
/// Get IPCA inflation (12-month accumulated) from IBGE
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn ibge_inflacao_ipc_a12(_start_date: String, _end_date: String) -> HashMap<String, Vec<f64>> {
    // Simplified implementation - returns mock inflation data
    let n_months = 12;
    let months: Vec<String> = (0..n_months).map(|i| {
        format!("2024-{:02}", i + 1)
    }).collect();
    
    let values: Vec<f64> = (0..n_months).map(|i| {
        // Mock inflation around 4.5%
        4.5 + (i as f64 * 0.1 - 0.6)
    }).collect();
    
    let mut result = HashMap::new();
    result.insert("month".to_string(), months.iter().map(|m| m.parse::<f64>().unwrap_or(0.0)).collect());
    result.insert("ipca_a12".to_string(), values);
    result
}

/// 6. ibge_taxa_desemprego(start_date, end_date)
/// Get unemployment rate from IBGE
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn ibge_taxa_desemprego(_start_date: String, _end_date: String) -> HashMap<String, Vec<f64>> {
    // Simplified implementation - returns mock unemployment data
    let n_months = 12;
    let months: Vec<String> = (0..n_months).map(|i| {
        format!("2024-{:02}", i + 1)
    }).collect();
    
    let values: Vec<f64> = (0..n_months).map(|i| {
        // Mock unemployment around 7.5%
        7.5 + (i as f64 * 0.05 - 0.3)
    }).collect();
    
    let mut result = HashMap::new();
    result.insert("month".to_string(), months.iter().map(|m| m.parse::<f64>().unwrap_or(0.0)).collect());
    result.insert("unemployment_rate".to_string(), values);
    result
}

/// 7. cvm_empresas_cia_aberta(cnpj)
/// Get company data from CVM (Comissão de Valores Mobiliários)
/// cnpj: company CNPJ
#[hayashi_fn]
pub fn cvm_empresas_cia_aberta(cnpj: String) -> HashMap<String, String> {
    // Simplified implementation - returns mock company data
    let mut result = HashMap::new();
    result.insert("cnpj".to_string(), cnpj);
    result.insert("razao_social".to_string(), "Empresa Exemplo S.A.".to_string());
    result.insert("setor".to_string(), "Financeiro".to_string());
    result.insert("situacao".to_string(), "Ativo".to_string());
    result
}

/// 8. cvm_demonstracoes_financeiras(cnpj, year)
/// Get financial statements from CVM
/// cnpj: company CNPJ
/// year: year of statements
#[hayashi_fn]
pub fn cvm_demonstracoes_financeiras(_cnpj: String, _year: i64) -> HashMap<String, Vec<f64>> {
    // Simplified implementation - returns mock financial data
    let mut result = HashMap::new();
    result.insert("receita_liquida".to_string(), vec![1000.0]);
    result.insert("lucro_liquido".to_string(), vec![150.0]);
    result.insert("ativo_total".to_string(), vec![5000.0]);
    result.insert("patrimonio_liquido".to_string(), vec![2000.0]);
    result
}

/// 9. cvm_fii_codigo(codigo)
/// Get FII (Fundo de Investimento Imobiliário) data from CVM
/// codigo: FII code (e.g., HGLG11)
#[hayashi_fn]
pub fn cvm_fii_codigo(codigo: String) -> HashMap<String, String> {
    // Simplified implementation - returns mock FII data
    let mut result = HashMap::new();
    result.insert("codigo".to_string(), codigo);
    result.insert("nome".to_string(), "FII Exemplo".to_string());
    result.insert("tipo".to_string(), "Tijolo".to_string());
    result.insert("cnpj".to_string(), "00.000.000/0001-00".to_string());
    result
}

/// 10. series_disponiveis_bcb()
/// List available BCB time series codes
#[hayashi_fn]
pub fn series_disponiveis_bcb() -> Vec<String> {
    vec![
        "432 - Selic meta".to_string(),
        "4189 - Selic over".to_string(),
        "21911 - PIB acumulado".to_string(),
        "223 - Reservas internacionais".to_string(),
        "13621 - Dólar comercial".to_string(),
        "13522 - Dólar turismo".to_string(),
        "12 - IPCA".to_string(),
        "433 - IGPM".to_string(),
        "226 - IGP-M".to_string(),
        "7482 - Taxa de câmbio real/dólar".to_string(),
    ]
}

/// 11. series_disponiveis_ibge()
/// List available IBGE data series
#[hayashi_fn]
pub fn series_disponiveis_ibge() -> Vec<String> {
    vec![
        "IPCA - Índice Nacional de Preços ao Consumidor Amplo".to_string(),
        "PIB municipal - Produto Interno Bruto por município".to_string(),
        "Taxa de desemprego - PNAD Contínua".to_string(),
        "População - Projeções".to_string(),
        "PIB trimestral - Contas Nacionais".to_string(),
        "INPC - Índice Nacional de Preços ao Consumidor".to_string(),
    ]
}

/// 12. converter_data_brasil(data_str)
/// Convert Brazilian date format (DD/MM/YYYY) to ISO format (YYYY-MM-DD)
/// data_str: date string in DD/MM/YYYY format
#[hayashi_fn]
pub fn converter_data_brasil(data_str: String) -> String {
    let parts: Vec<&str> = data_str.split("/").collect();
    if parts.len() == 3 {
        format!("{}-{}-{}", parts[2], parts[1], parts[0])
    } else {
        data_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcb_selic() {
        let result = bcb_selic(432, "2024-01-01".to_string(), "2024-01-30".to_string());
        assert!(result.contains_key("date"));
        assert!(result.contains_key("value"));
    }

    #[test]
    fn test_bcb_pib() {
        let result = bcb_pib(21911, "2024-01-01".to_string(), "2024-12-31".to_string());
        assert!(result.contains_key("quarter"));
        assert!(result.contains_key("pib_brl"));
    }

    #[test]
    fn test_ibge_inflacao_ipc_a12() {
        let result = ibge_inflacao_ipc_a12("2024-01-01".to_string(), "2024-12-31".to_string());
        assert!(result.contains_key("month"));
        assert!(result.contains_key("ipca_a12"));
    }

    #[test]
    fn test_cvm_empresas_cia_aberta() {
        let result = cvm_empresas_cia_aberta("00.000.000/0001-00".to_string());
        assert!(result.contains_key("cnpj"));
        assert!(result.contains_key("razao_social"));
    }

    #[test]
    fn test_converter_data_brasil() {
        let result = converter_data_brasil("01/01/2024".to_string());
        assert_eq!(result, "2024-01-01");
    }
}
