# haybrasil

Brazilian macroeconomic data plugin for [Hayashi](https://github.com/sheep-farm/hayashi) — provides access to real-time data from BCB (Banco Central do Brasil), IBGE, and CVM (Comissão de Valores Mobiliários) through official APIs.

## Install

```bash
hay install sheep-farm/haybrasil
```

Or manually:

```bash
git clone https://github.com/sheep-farm/haybrasil.git
cd haybrasil
cargo build --release
cp target/release/libhaybrasil.so ~/.hay/packages/sheep-farm/haybrasil.so
```

## Usage

```hayashi
import("sheep-farm/haybrasil", as=br)

// Get Selic rate from BCB (real-time data)
let selic = br::bcb_selic(432, "2024-01-01", "2024-01-30")
print(selic)

// Get GDP data from BCB
let pib = br::bcb_pib(21911, "2024-01-01", "2024-12-31")
print(pib)

// Get international reserves
let reservas = br::bcb_reservas_internacionais("2024-01-01", "2024-01-30")
print(reservas)

// Get municipal GDP from IBGE
let pib_municipal = br::ibge_pib_municipal(43, 2022)
print(pib_municipal)

// Get IPCA inflation
let ipca = br::ibge_inflacao_ipc_a12("2024-01-01", "2024-12-31")
print(ipca)

// Get unemployment rate
let desemprego = br::ibge_taxa_desemprego("2024-01-01", "2024-12-31")
print(desemprego)

// Get company data from CVM
let empresa = br::cvm_empresas_cia_aberta("00.000.000/0001-00")
print(empresa)

// Get financial statements
let df = br::cvm_demonstracoes_financeiras("00.000.000/0001-00", 2023)
print(df)

// Get FII data
let fii = br::cvm_fii_codigo("HGLG11")
print(fii)

// List available series
let series_bcb = br::series_disponiveis_bcb()
print(series_bcb)

let series_ibge = br::series_disponiveis_ibge()
print(series_ibge)

// Convert Brazilian date format
let data_iso = br::converter_data_brasil("01/01/2024")
print(data_iso)
```

## Functions

### BCB (Banco Central do Brasil)

#### `bcb_selic(series_code, start_date, end_date)`
Get Selic rate from BCB API (real-time data).

- `series_code`: BCB time series code (e.g., 432 for Selic meta)
- `start_date`: start date in YYYY-MM-DD format
- `end_date`: end date in YYYY-MM-DD format

Returns a dict with `date` and `value` keys with real data from BCB API.

#### `bcb_pib(series_code, start_date, end_date)`
Get GDP (PIB) data from BCB API.

- `series_code`: BCB time series code (e.g., 21911 for PIB acumulado)
- `start_date`: start date in YYYY-MM-DD format
- `end_date`: end date in YYYY-MM-DD format

Returns a dict with `date` and `pib_brl` keys with real data from BCB API.

#### `bcb_reservas_internacionais(start_date, end_date)`
Get international reserves from BCB API.

- `start_date`: start date in YYYY-MM-DD format
- `end_date`: end date in YYYY-MM-DD format

Returns a dict with `date` and `reserves_usd` keys with real data from BCB API.

### IBGE (Instituto Brasileiro de Geografia e Estatística)

#### `ibge_pib_municipal(uf, year)`
Get municipal GDP data from IBGE API.

- `uf`: state code (e.g., 43 for RS)
- `year`: year of data

Returns a dict with `municipality` and `pib_milhoes` keys with real data from IBGE API.

#### `ibge_inflacao_ipc_a12(start_date, end_date)`
Get IPCA inflation (12-month accumulated) from IBGE API.

- `start_date`: start date in YYYY-MM-DD format
- `end_date`: end date in YYYY-MM-DD format

Returns a dict with `month` and `ipca_a12` keys with real data from IBGE API.

#### `ibge_taxa_desemprego(start_date, end_date)`
Get unemployment rate from IBGE API.

- `start_date`: start date in YYYY-MM-DD format
- `end_date`: end date in YYYY-MM-DD format

Returns a dict with `month` and `unemployment_rate` keys with real data from IBGE API.

### CVM (Comissão de Valores Mobiliários)

#### `cvm_empresas_cia_aberta(cnpj)`
Get company data from CVM official database.

- `cnpj`: company CNPJ

Returns a dict with company information from CVM database.

#### `cvm_demonstracoes_financeiras(cnpj, year)`
Get financial statements from CVM official database.

- `cnpj`: company CNPJ
- `year`: year of statements

Returns a dict with financial metrics from CVM database.

#### `cvm_fii_codigo(codigo)`
Get FII (Fundo de Investimento Imobiliário) data from CVM official database.

- `codigo`: FII code (e.g., HGLG11)

Returns a dict with FII information from CVM database.

### Utilities

#### `series_disponiveis_bcb()`
List available BCB time series codes.

Returns a list of series descriptions.

#### `series_disponiveis_ibge()`
List available IBGE data series.

Returns a list of series descriptions.

#### `converter_data_brasil(data_str)`
Convert Brazilian date format (DD/MM/YYYY) to ISO format (YYYY-MM-DD).

- `data_str`: date string in DD/MM/YYYY format

Returns date string in YYYY-MM-DD format.

## BCB Series Codes

Common BCB time series codes:

| Code | Description |
|------|-------------|
| 432 | Selic meta |
| 4189 | Selic over |
| 21911 | PIB acumulado |
| 223 | Reservas internacionais |
| 13621 | Dólar comercial |
| 13522 | Dólar turismo |
| 12 | IPCA |
| 433 | IGPM |
| 226 | IGP-M |
| 7482 | Taxa de câmbio real/dólar |

## Data Sources

- **BCB**: Banco Central do Brasil API (https://api.bcb.gov.br) - Real-time economic data
- **IBGE**: Instituto Brasileiro de Geografia e Estatística (https://servicodados.ibge.gov.br) - Official statistics
- **CVM**: Comissão de Valores Mobiliários (https://dados.cvm.gov.br) - Official company and financial data

## Error Handling

All functions return error indicators if API calls fail:
- Numeric functions return `{"error": [-1.0]}` on failure
- String functions return `{"error": "error message"}` on failure

## Development

```bash
cargo build --release
cp target/release/libhaybrasil.so ~/.hay/packages/sheep-farm/haybrasil.so
```

## License

MIT
