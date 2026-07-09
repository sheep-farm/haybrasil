#![allow(clippy::not_unsafe_ptr_arg_deref)]
use chrono::{Datelike, NaiveDate};
use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};
use serde::Deserialize;
use std::collections::HashMap;

hayashi_plugin!();

// BCB API structures
#[derive(Debug, Deserialize)]
struct BCBDataPoint {
    data: String,
    valor: String,
}

// IBGE API structures
#[derive(Debug, Deserialize)]
struct IBGEDataPoint {
    #[serde(rename = "D1C")]
    date: String,
    #[serde(rename = "V")]
    value: f64,
}

/// 1. bcb_selic(series_code, start_date, end_date)
/// Get Selic rate from BCB (Banco Central do Brasil)
/// series_code: BCB time series code (e.g., 432 for Selic meta)
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn bcb_selic(
    series_code: i64,
    start_date: String,
    end_date: String,
) -> HashMap<String, Vec<f64>> {
    let url = format!(
        "https://api.bcb.gov.br/dados/serie/bcdata.sgs.{}/dados?formato=json&dataInicial={}&dataFinal={}",
        series_code, start_date, end_date
    );

    match fetch_bcb_data(&url) {
        Ok(data) => {
            let dates: Vec<f64> = data
                .iter()
                .filter_map(|d| parse_date_to_float(&d.data))
                .collect();
            let values: Vec<f64> = data
                .iter()
                .filter_map(|d| d.valor.replace(',', ".").parse::<f64>().ok())
                .collect();

            let mut result = HashMap::new();
            result.insert("date".to_string(), dates);
            result.insert("value".to_string(), values);
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), vec![-1.0]);
            result
        }
    }
}

/// 2. bcb_pib(series_code, start_date, end_date)
/// Get GDP (PIB) data from BCB
/// series_code: BCB time series code (e.g., 21911 for PIB acumulado)
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn bcb_pib(
    series_code: i64,
    start_date: String,
    end_date: String,
) -> HashMap<String, Vec<f64>> {
    let url = format!(
        "https://api.bcb.gov.br/dados/serie/bcdata.sgs.{}/dados?formato=json&dataInicial={}&dataFinal={}",
        series_code, start_date, end_date
    );

    match fetch_bcb_data(&url) {
        Ok(data) => {
            let dates: Vec<f64> = data
                .iter()
                .filter_map(|d| parse_date_to_float(&d.data))
                .collect();
            let values: Vec<f64> = data
                .iter()
                .filter_map(|d| d.valor.replace(',', ".").parse::<f64>().ok())
                .collect();

            let mut result = HashMap::new();
            result.insert("date".to_string(), dates);
            result.insert("pib_brl".to_string(), values);
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), vec![-1.0]);
            result
        }
    }
}

/// 3. bcb_reservas_internacionais(start_date, end_date)
/// Get international reserves from BCB
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn bcb_reservas_internacionais(
    start_date: String,
    end_date: String,
) -> HashMap<String, Vec<f64>> {
    let url = format!(
        "https://api.bcb.gov.br/dados/serie/bcdata.sgs.223/dados?formato=json&dataInicial={}&dataFinal={}",
        start_date, end_date
    );

    match fetch_bcb_data(&url) {
        Ok(data) => {
            let dates: Vec<f64> = data
                .iter()
                .filter_map(|d| parse_date_to_float(&d.data))
                .collect();
            let values: Vec<f64> = data
                .iter()
                .filter_map(|d| d.valor.replace(',', ".").parse::<f64>().ok())
                .collect();

            let mut result = HashMap::new();
            result.insert("date".to_string(), dates);
            result.insert("reserves_usd".to_string(), values);
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), vec![-1.0]);
            result
        }
    }
}

/// 4. ibge_pib_municipal(uf, year)
/// Get municipal GDP data from IBGE
/// uf: state code (e.g., 43 for RS)
/// year: year of data
#[hayashi_fn]
pub fn ibge_pib_municipal(uf: i64, year: i64) -> HashMap<String, Vec<f64>> {
    let url = format!(
        "https://servicodados.ibge.gov.br/api/v3/agregados?periodo={}&codigoregiao={}&classificacao=58/all/variables/5930",
        year, uf
    );

    match fetch_ibge_data(&url) {
        Ok(data) => {
            let municipalities: Vec<f64> = data
                .iter()
                .map(|d| d.date.parse::<f64>().unwrap_or(0.0))
                .collect();
            let values: Vec<f64> = data.iter().map(|d| d.value).collect();

            let mut result = HashMap::new();
            result.insert("municipality".to_string(), municipalities);
            result.insert("pib_milhoes".to_string(), values);
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), vec![-1.0]);
            result
        }
    }
}

/// 5. ibge_inflacao_ipc_a12(start_date, end_date)
/// Get IPCA inflation (12-month accumulated) from IBGE
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn ibge_inflacao_ipc_a12(start_date: String, end_date: String) -> HashMap<String, Vec<f64>> {
    let url = format!(
        "https://servicodados.ibge.gov.br/api/v3/agregados/1737/periodos/{}/-/variaveis/2266?localidades=BR",
        extract_year_range(&start_date, &end_date)
    );

    match fetch_ibge_data(&url) {
        Ok(data) => {
            let months: Vec<f64> = data
                .iter()
                .map(|d| d.date.parse::<f64>().unwrap_or(0.0))
                .collect();
            let values: Vec<f64> = data.iter().map(|d| d.value).collect();

            let mut result = HashMap::new();
            result.insert("month".to_string(), months);
            result.insert("ipca_a12".to_string(), values);
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), vec![-1.0]);
            result
        }
    }
}

/// 6. ibge_taxa_desemprego(start_date, end_date)
/// Get unemployment rate from IBGE
/// start_date: start date in YYYY-MM-DD format
/// end_date: end date in YYYY-MM-DD format
#[hayashi_fn]
pub fn ibge_taxa_desemprego(start_date: String, end_date: String) -> HashMap<String, Vec<f64>> {
    let url = format!(
        "https://servicodados.ibge.gov.br/api/v3/agregados/6372/periodos/{}/-/variaveles/4099?localidades=BR",
        extract_year_range(&start_date, &end_date)
    );

    match fetch_ibge_data(&url) {
        Ok(data) => {
            let months: Vec<f64> = data
                .iter()
                .map(|d| d.date.parse::<f64>().unwrap_or(0.0))
                .collect();
            let values: Vec<f64> = data.iter().map(|d| d.value).collect();

            let mut result = HashMap::new();
            result.insert("month".to_string(), months);
            result.insert("unemployment_rate".to_string(), values);
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), vec![-1.0]);
            result
        }
    }
}

/// 7. cvm_empresas_cia_aberta(cnpj)
/// Get company data from CVM (Comissão de Valores Mobiliários)
/// cnpj: company CNPJ
#[hayashi_fn]
pub fn cvm_empresas_cia_aberta(cnpj: String) -> HashMap<String, String> {
    let url = "https://dados.cvm.gov.br/dados/CIA_ABERTA/CAD/DADOS/cad_cia_aberta.csv";

    match fetch_cvm_company_data(url, &cnpj) {
        Ok(company) => {
            let mut result = HashMap::new();
            result.insert("cnpj".to_string(), company.cnpj);
            result.insert("razao_social".to_string(), company.razao_social);
            result.insert("setor".to_string(), company.setor);
            result.insert("situacao".to_string(), company.situacao);
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), "Company not found".to_string());
            result
        }
    }
}

/// 8. cvm_demonstracoes_financeiras(cnpj, year)
/// Get financial statements from CVM
/// cnpj: company CNPJ
/// year: year of statements
#[hayashi_fn]
pub fn cvm_demonstracoes_financeiras(cnpj: String, year: i64) -> HashMap<String, Vec<f64>> {
    let url = format!(
        "https://dados.cvm.gov.br/dados/CIA_ABERTA/DOC/FRE/DADOS/fre_cia_aberta_{}.csv",
        year
    );

    match fetch_cvm_financial_data(&url, &cnpj) {
        Ok(data) => {
            let mut result = HashMap::new();
            result.insert("receita_liquida".to_string(), vec![data.receita_liquida]);
            result.insert("lucro_liquido".to_string(), vec![data.lucro_liquido]);
            result.insert("ativo_total".to_string(), vec![data.ativo_total]);
            result.insert(
                "patrimonio_liquido".to_string(),
                vec![data.patrimonio_liquido],
            );
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), vec![-1.0]);
            result
        }
    }
}

/// 9. cvm_fii_codigo(codigo)
/// Get FII (Fundo de Investimento Imobiliário) data from CVM
/// codigo: FII code (e.g., HGLG11)
#[hayashi_fn]
pub fn cvm_fii_codigo(codigo: String) -> HashMap<String, String> {
    let url = "https://dados.cvm.gov.br/dados/FII/DOC/DADOS/DADOS/fii.csv";

    match fetch_cvm_fii_data(url, &codigo) {
        Ok(fii) => {
            let mut result = HashMap::new();
            result.insert("codigo".to_string(), fii.codigo);
            result.insert("nome".to_string(), fii.nome);
            result.insert("tipo".to_string(), fii.tipo);
            result.insert("cnpj".to_string(), fii.cnpj);
            result
        }
        Err(_) => {
            let mut result = HashMap::new();
            result.insert("error".to_string(), "FII not found".to_string());
            result
        }
    }
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

// Helper functions

fn fetch_bcb_data(url: &str) -> Result<Vec<BCBDataPoint>, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let data: Vec<BCBDataPoint> = response.json()?;
    Ok(data)
}

fn fetch_ibge_data(url: &str) -> Result<Vec<IBGEDataPoint>, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let data: Vec<IBGEDataPoint> = response.json()?;
    Ok(data)
}

#[derive(Debug)]
struct CVMCompanyData {
    cnpj: String,
    razao_social: String,
    setor: String,
    situacao: String,
}

fn fetch_cvm_company_data(
    url: &str,
    cnpj: &str,
) -> Result<CVMCompanyData, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let text = response.text()?;

    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split(';').collect::<Vec<_>>();
        if parts.len() >= 4 && parts[0].trim() == cnpj {
            return Ok(CVMCompanyData {
                cnpj: parts[0].trim().to_string(),
                razao_social: parts[1].trim().to_string(),
                setor: parts[2].trim().to_string(),
                situacao: parts[3].trim().to_string(),
            });
        }
    }

    Err("Company not found".into())
}

#[derive(Debug)]
struct CVMFinancialData {
    receita_liquida: f64,
    lucro_liquido: f64,
    ativo_total: f64,
    patrimonio_liquido: f64,
}

fn fetch_cvm_financial_data(
    url: &str,
    cnpj: &str,
) -> Result<CVMFinancialData, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let text = response.text()?;

    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split(';').collect::<Vec<_>>();
        if parts.len() >= 5 && parts[0].trim() == cnpj {
            return Ok(CVMFinancialData {
                receita_liquida: parts[1].trim().parse().unwrap_or(0.0),
                lucro_liquido: parts[2].trim().parse().unwrap_or(0.0),
                ativo_total: parts[3].trim().parse().unwrap_or(0.0),
                patrimonio_liquido: parts[4].trim().parse().unwrap_or(0.0),
            });
        }
    }

    Err("Financial data not found".into())
}

#[derive(Debug)]
struct CVMFIIData {
    codigo: String,
    nome: String,
    tipo: String,
    cnpj: String,
}

fn fetch_cvm_fii_data(url: &str, codigo: &str) -> Result<CVMFIIData, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let text = response.text()?;

    for line in text.lines().skip(1) {
        let parts: Vec<&str> = line.split(';').collect::<Vec<_>>();
        if parts.len() >= 4 && parts[0].trim() == codigo {
            return Ok(CVMFIIData {
                codigo: parts[0].trim().to_string(),
                nome: parts[1].trim().to_string(),
                tipo: parts[2].trim().to_string(),
                cnpj: parts[3].trim().to_string(),
            });
        }
    }

    Err("FII not found".into())
}

fn parse_date_to_float(date_str: &str) -> Option<f64> {
    let date = NaiveDate::parse_from_str(date_str, "%d/%m/%Y").ok()?;
    Some(date.year() as f64 * 10000.0 + date.month() as f64 * 100.0 + date.day() as f64)
}

fn extract_year_range(start_date: &str, end_date: &str) -> String {
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").ok();
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").ok();

    match (start, end) {
        (Some(s), Some(e)) => {
            if s.year() == e.year() {
                s.year().to_string()
            } else {
                format!("{}-{}", s.year(), e.year())
            }
        }
        _ => "2024".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_data_brasil() {
        // converter_data_brasil tem #[hayashi_fn]; chamar __hayashi_impl_* diretamente.
        let result = __hayashi_impl_converter_data_brasil("01/01/2024".to_string());
        assert_eq!(result, "2024-01-01");
    }

    #[test]
    fn test_parse_date_to_float() {
        let result = parse_date_to_float("01/01/2024");
        assert_eq!(result, Some(20240101.0));
    }

    #[test]
    fn test_extract_year_range() {
        let result = extract_year_range("2024-01-01", "2024-12-31");
        assert_eq!(result, "2024");

        let result = extract_year_range("2023-01-01", "2024-12-31");
        assert_eq!(result, "2023-2024");
    }
}
