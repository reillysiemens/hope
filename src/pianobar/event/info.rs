//! Pianobar event info.

use std::{collections::HashMap, str::FromStr};

/// The status of pianobar's calls to the [Pandora JSON API](https://6xq.net/pandora-apidoc/json/).
#[derive(Debug)]
pub struct PianobarStatus {
    pub code: i32,
    pub message: String,
}

/// The status of pianobar's HTTP requests using cURL.
#[derive(Debug)]
pub struct CurlStatus {
    pub code: i32,
    pub message: String,
}

/// Metadata about the currently playing song (if any).
#[derive(Debug)]
pub struct Song {
    pub duration: i32,
    pub played: i32,
}

/// Pianobar event info.
///
/// Read from `stdin` and parsed from a sequence of `=`-separated key-value
/// pairs that looks like this
/// ```text
/// artist=Count Basie
/// title=Splanky
/// album=The Atomic Mr Basie
/// ...
/// ```
///
/// Station information is special and represented in an "array" format that
/// looks like this
/// ```text
/// stationCount=124
/// station0=2Pac (Tupac) Radio
/// station1=A Tribe Called Quest Radio
/// ...
/// ```
#[derive(Debug)]
pub struct Info {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub cover_art: Option<String>,
    pub station_name: Option<String>,
    // TODO: Should this be part of the Song struct?
    pub song_station_name: Option<String>,
    pub pianobar_status: PianobarStatus,
    pub curl_status: CurlStatus,
    pub song: Song,
    pub rating: i32,
    pub detail_url: Option<String>,
    pub stations: Vec<String>,
}

/// [true] if a [String] is not empty, otherwise [false].
#[allow(clippy::ptr_arg)] // Option<String>::filter will return a &String.
fn not_empty(string: &String) -> bool {
    !string.is_empty()
}

/// An error which can be returned when parsing event info.
#[derive(Debug, thiserror::Error)]
#[error("Invalid info")]
pub struct ParseInfoError;

impl FromStr for Info {
    type Err = ParseInfoError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Splitting at "stationCount" allows us to treat the station array
        // differently from the rest of the key-value pairs.
        let Some((info, stations)) = input.split_once("stationCount=") else {
            return Err(ParseInfoError);
        };

        // Parse info into a HashMap so we don't have to trust the order of
        // key-value pairs returned by pianobar.
        let mut info: HashMap<&str, String> = info
            .lines()
            .map(|line| match line.split_once('=') {
                Some((key, value)) => Ok((key, value.to_string())),
                None => Err(ParseInfoError),
            })
            .collect::<Result<_, _>>()?;

        // We use remove here because we want ownership, not a borrow.
        let artist = info.remove("artist").filter(not_empty);
        let title = info.remove("title").filter(not_empty);
        let album = info.remove("album").filter(not_empty);
        let cover_art = info.remove("coverArt").filter(not_empty);
        let station_name = info.remove("stationName").filter(not_empty);
        let song_station_name = info.remove("songStationName").filter(not_empty);

        let pianobar_status = PianobarStatus {
            code: info
                .remove("pRet")
                .ok_or(ParseInfoError)?
                .parse()
                .or(Err(ParseInfoError))?,
            message: info.remove("pRetStr").ok_or(ParseInfoError)?,
        };

        let curl_status = CurlStatus {
            code: info
                .remove("wRet")
                .ok_or(ParseInfoError)?
                .parse()
                .or(Err(ParseInfoError))?,
            message: info.remove("wRetStr").ok_or(ParseInfoError)?,
        };

        let song = Song {
            duration: info
                .remove("songDuration")
                .ok_or(ParseInfoError)?
                .parse()
                .or(Err(ParseInfoError))?,
            played: info
                .remove("songPlayed")
                .ok_or(ParseInfoError)?
                .parse()
                .or(Err(ParseInfoError))?,
        };

        let rating = info
            .remove("rating")
            .ok_or(ParseInfoError)?
            .parse()
            .or(Err(ParseInfoError))?;
        let detail_url = info.remove("detailUrl").filter(not_empty);

        let mut stations = stations.lines();
        // Skip the line which contains the station count, we don't need it.
        stations.next();
        // Parse the rest of the station array.
        let stations: Vec<String> = stations
            .map(|line| match line.split_once('=') {
                Some((_, station)) => Ok(station.into()),
                None => Err(ParseInfoError),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            artist,
            title,
            album,
            cover_art,
            station_name,
            song_station_name,
            pianobar_status,
            curl_status,
            song,
            rating,
            detail_url,
            stations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Info;
    use test_case::test_case;

    const USER_LOGIN: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/userlogin"
    ));
    const USER_GET_STATIONS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/usergetstations"
    ));
    const SONG_START: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/songstart"
    ));
    const STATION_FETCH_PLAYLIST: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/stationfetchplaylist"
    ));

    #[test_case(USER_LOGIN ; "userlogin")]
    #[test_case(USER_GET_STATIONS ; "usergetstations")]
    #[test_case(SONG_START ; "songstart")]
    #[test_case(STATION_FETCH_PLAYLIST ; "stationfetchplaylist")]
    fn parse_info(input: &str) {
        let _: Info = input.parse().expect("Failed to parse info");
    }
}
