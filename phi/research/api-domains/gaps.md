Hier ist die komplette Liste all der anderen API-Findings, die Claude inventoryiert hat, formatiert als fertige Blöcke für deine `sources.φ`-Datei. 

Ich habe die URL-Kodierung für die Yahoo-Finance-Ticker (wie `^VIX` zu `%5EVIX`) bereits korrigiert, damit dein Rust-Server keine Probleme beim Parsen hat.

```text
# ==========================================
# WIRTSCHAFT & MAKRO-ÖKONOMIE
# ==========================================

source exchangerate_global
ttl 3600
url https://open.er-api.com/v6/latest/USD
field rates.EUR economic_usd_eur_rate
field rates.CNY economic_usd_cny_rate
field rates.INR economic_usd_inr_rate
field rates.BRL economic_usd_brl_rate
field rates.ZAR economic_usd_zar_rate
field rates.NGN economic_usd_ngn_rate
field rates.KES economic_usd_kes_rate
field rates.EGP economic_usd_egp_rate
field rates.GHS economic_usd_ghs_rate
field rates.IDR economic_usd_idr_rate
field rates.MXN economic_usd_mxn_rate
field rates.TRY economic_usd_try_rate
field rates.VND economic_usd_vnd_rate
field rates.PKR economic_usd_pkr_rate
field rates.BDT economic_usd_bdt_rate
field rates.PHP economic_usd_php_rate

source fred_inflation_cpi
ttl 86400
url https://api.stlouisfed.org/fred/series/observations?series_id=CPIAUCSL&api_key={FRED_API_KEY}&file_type=json&sort_order=desc&limit=1
path observations.0.value economic_us_inflation_cpi

source fred_unemployment
ttl 86400
url https://api.stlouisfed.org/fred/series/observations?series_id=UNRATE&api_key={FRED_API_KEY}&file_type=json&sort_order=desc&limit=1
path observations.0.value economic_us_unemployment_rate

source fred_fed_funds_rate
ttl 86400
url https://api.stlouisfed.org/fred/series/observations?series_id=FEDFUNDS&api_key={FRED_API_KEY}&file_type=json&sort_order=desc&limit=1
path observations.0.value economic_us_fed_funds_rate

source fred_gdp
ttl 2592000
url https://api.stlouisfed.org/fred/series/observations?series_id=GDP&api_key={FRED_API_KEY}&file_type=json&sort_order=desc&limit=1
path observations.0.value economic_us_gdp_nominal

# --- Welt- & Sparten-ETFs als Wirtschafts-Proxys (via Yahoo Finance) ---
# Global
source etf_world_developed
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/URTH?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_world_developed_price
field chart.result.0.meta.chartPreviousClose economic_world_developed_prev_close

source etf_world_all_country
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/ACWI?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_world_acwi_price
field chart.result.0.meta.chartPreviousClose economic_world_acwi_prev_close

source etf_emerging_markets
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/EEM?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_emerging_markets_price
field chart.result.0.meta.chartPreviousClose economic_emerging_markets_prev_close

source etf_frontier_markets
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/FM?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_frontier_markets_price
field chart.result.0.meta.chartPreviousClose economic_frontier_markets_prev_close

# Regionen (inkl. Globaler Süden)
source etf_usa
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/SPY?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_usa_price
field chart.result.0.meta.chartPreviousClose economic_region_usa_prev_close

source etf_europe
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/VGK?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_europe_price
field chart.result.0.meta.chartPreviousClose economic_region_europe_prev_close

source etf_germany
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/EWG?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_germany_price
field chart.result.0.meta.chartPreviousClose economic_region_germany_prev_close

source etf_japan
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/EWJ?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_japan_price
field chart.result.0.meta.chartPreviousClose economic_region_japan_prev_close

source etf_china
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/FXI?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_china_price
field chart.result.0.meta.chartPreviousClose economic_region_china_prev_close

source etf_india
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/INDA?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_india_price
field chart.result.0.meta.chartPreviousClose economic_region_india_prev_close

source etf_brazil
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/EWZ?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_brazil_price
field chart.result.0.meta.chartPreviousClose economic_region_brazil_prev_close

source etf_africa_broad
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/AFK?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_africa_price
field chart.result.0.meta.chartPreviousClose economic_region_africa_prev_close

source etf_south_africa
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/EZA?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_south_africa_price
field chart.result.0.meta.chartPreviousClose economic_region_south_africa_prev_close

source etf_southeast_asia
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/ASEA?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_southeast_asia_price
field chart.result.0.meta.chartPreviousClose economic_region_southeast_asia_prev_close

source etf_middle_east
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/KSA?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_gulf_price
field chart.result.0.meta.chartPreviousClose economic_region_gulf_prev_close

source etf_mexico
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/EWW?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_mexico_price
field chart.result.0.meta.chartPreviousClose economic_region_mexico_prev_close

source etf_latin_america_broad
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/ILF?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_region_latin_america_price
field chart.result.0.meta.chartPreviousClose economic_region_latin_america_prev_close

# Sektoren
source etf_sector_energy
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLE?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_energy_price
field chart.result.0.meta.chartPreviousClose economic_sector_energy_prev_close

source etf_sector_financials
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLF?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_financials_price
field chart.result.0.meta.chartPreviousClose economic_sector_financials_prev_close

source etf_sector_technology
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLK?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_technology_price
field chart.result.0.meta.chartPreviousClose economic_sector_technology_prev_close

source etf_sector_healthcare
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLV?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_healthcare_price
field chart.result.0.meta.chartPreviousClose economic_sector_healthcare_prev_close

source etf_sector_industrials
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLI?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_industrials_price
field chart.result.0.meta.chartPreviousClose economic_sector_industrials_prev_close

source etf_sector_materials
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLB?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_materials_price
field chart.result.0.meta.chartPreviousClose economic_sector_materials_prev_close

source etf_sector_utilities
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLU?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_utilities_price
field chart.result.0.meta.chartPreviousClose economic_sector_utilities_prev_close

source etf_sector_consumer_staples
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLP?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_consumer_staples_price
field chart.result.0.meta.chartPreviousClose economic_sector_consumer_staples_prev_close

source etf_sector_consumer_discretionary
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLY?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_consumer_discretionary_price
field chart.result.0.meta.chartPreviousClose economic_sector_consumer_discretionary_prev_close

source etf_sector_real_estate
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLRE?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_real_estate_price
field chart.result.0.meta.chartPreviousClose economic_sector_real_estate_prev_close

source etf_sector_communication
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/XLC?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_sector_communication_price
field chart.result.0.meta.chartPreviousClose economic_sector_communication_prev_close

# Leading Indicators & Commodities (URL-kodiert für ^ und =)
source etf_semiconductors
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/SOXX?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_leading_semiconductors_price
field chart.result.0.meta.chartPreviousClose economic_leading_semiconductors_prev_close

source etf_transportation
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/IYT?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_leading_transportation_price
field chart.result.0.meta.chartPreviousClose economic_leading_transportation_prev_close

source etf_dry_bulk_shipping
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/BDRY?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_leading_global_trade_proxy_price
field chart.result.0.meta.chartPreviousClose economic_leading_global_trade_proxy_prev_close

source etf_gold
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/GC%3DF?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_commodity_gold_price
field chart.result.0.meta.chartPreviousClose economic_commodity_gold_prev_close

source etf_oil
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/CL%3DF?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_commodity_oil_price
field chart.result.0.meta.chartPreviousClose economic_commodity_oil_prev_close

source etf_treasury_yield_10y
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/%5ETNX?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_interest_rate_10y_treasury
field chart.result.0.meta.chartPreviousClose economic_interest_rate_10y_treasury_prev

source etf_dollar_index
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/DX-Y.NYB?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_dollar_index_price
field chart.result.0.meta.chartPreviousClose economic_dollar_index_prev_close

source etf_volatility
ttl 60
header User-Agent "omegaflow"
url https://query1.finance.yahoo.com/v8/finance/chart/%5EVIX?interval=1d&range=1d
field chart.result.0.meta.regularMarketPrice economic_fear_gauge_vix
field chart.result.0.meta.chartPreviousClose economic_fear_gauge_vix_prev


# ==========================================
# NOOSPHÄRE & DIGITALES WISSEN
# ==========================================

source wikipedia_pageviews_total
ttl 86400
url https://wikimedia.org/api/rest_v1/metrics/pageviews/aggregate/en.wikipedia/all-access/all-agents/daily/{yesterday}/{yesterday}
path items.0.views social_wikipedia_pageviews_daily

source arxiv_new_papers
ttl 3600
url http://export.arxiv.org/api/query?search_query=all&sortBy=submittedDate&sortOrder=descending&max_results=1
format xml
last published social_arxiv_latest_paper_date

source github_public_events
ttl 60
header User-Agent "omegaflow"
url https://api.github.com/events?per_page=100
count . technosphere_github_events_per_minute

source gdelt_news_volume
ttl 900
url https://api.gdeltproject.org/api/v2/doc/doc?query=sourcelang:eng&mode=timelinevol&format=json
last value social_global_news_volume


# ==========================================
# SOZIALES & ANTHROPOSPHÄRE
# ==========================================

source unhcr_displacement
ttl 2592000
url https://api.unhcr.org/population/v1/population/?limit=1&yearFrom={last_year}&yearTo={last_year}&coo_all=true&coa_all=true
sum individuals social_global_displaced_persons


# ==========================================
# ORBITAL & KOSMISCH (Lücken)
# ==========================================

source astronauts_in_space
ttl 3600
url http://api.open-notify.org/astros.json
count people anthroposphere_humans_in_space

source moon_phase
ttl 3600
url https://api.farmsense.net/v1/moonphases/?d={unix_now}
path 0.Phase cosmic_moon_phase
path 0.Illumination cosmic_moon_illumination_pct
```

*(Hinweis: Für die FRED-API musst du dir kostenlos einen API-Key auf der St. Louis Fed Website holen und ihn in der `.env`-Datei deines Servers als `FRED_API_KEY=dein_key` eintragen. Genauso habe ich für UNHCR `{last_year}` als Platzhalter genutzt, den du ggf. in der `render_url`-Funktion im Rust-Code noch implementieren musst, falls noch nicht vorhanden, andernfalls ersetze es testweise durch das aktuelle Jahr).*
