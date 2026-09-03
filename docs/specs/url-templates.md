<!--
  title: URL Template Variables
  class: concept
  sha256: d530c78cf06eb916f52f4e13da7c30a7e3bbc5c618d2cff7deea0b5cdebd58ca
  status: live
-->
# URL Template Variables

`src/main.rs`

All variables substituted via `.replace()` in `render_url()`.

## Spatial

```
{x} {y} {z}                    ICRS coordinates
{lat} {lon}                     geodetic (6 decimal places)
{lat_int} {lon_int}             as integer
{lat_min} {lat_max}             bounding box (Lemma-derived via extent / m_per_deg)
{lon_min} {lon_max}             
{grid}                          4×4 grid of lat,lon pairs (pipe-separated)
{grid_lat} {grid_lon}           grid axes (comma-separated)
```

## Temporal

```
{today}                         YYYY-MM-DD
{yesterday} {tomorrow}          
{today_yyyymmdd} {today_ymd}    YYYYMMDD (aliases)
{today_nodashes} {yesterday_nodashes} {tomorrow_nodashes}  YYYYMMDD
{t_start}                       = {yesterday}
{t_end}                         = {today}
{hour_ago}                      YYYY-MM-DDTHH:MM:00Z
{now} {now_minus_1} {now_minus_2}  YYYY-MM-DDTHH:MM:SSZ
{week_ago}                      YYYY-MM-DDTHH:MM:SSZ
{week_ago_nodashes}             YYYYMMDD
{today_plus_365}                YYYY-MM-DD (one year ahead)
{unix_now} {unix_now_plus_3600} UNIX seconds
```

## Date components

```
{year}                          YYYY
{year2}                         YY
{month}                         MM
{day}                           DD
{yday}                          DDD (001-366)
{hour}                          HH (00-23)
{minute}                        MM (00-59)
```

## Secrets

`{KEY_NAME}` resolved via `resolve_secret()`. Lookup from environment (loaded from `.secrets.local` at startup). Case-insensitive.

## Format directive

`src/main.rs`

```
format json        default
format text        
format csv         
format ephemeris_binary   direct .eph binary response
format universal           auto-detect from Content-Type
```
