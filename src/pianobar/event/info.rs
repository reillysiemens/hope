//! Pianobar event info.

use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct PianobarStatus {
    // Defined by the PianoReturn_t enum, but it's... complicated.
    code: i32,
    message: String,
}

#[derive(Debug)]
pub struct CurlStatus {
    code: i32,
    message: String,
}

#[derive(Debug)]
pub struct Song {
    duration: i32,
    played: i32,
}

#[derive(Debug)]
pub struct Info {
    artist: Option<String>,
    title: Option<String>,
    album: Option<String>,
    cover_art: Option<String>, // TODO: Make this a URL?
    station_name: Option<String>,
    song_station_name: Option<String>, // TODO: Is this ever not null?
    pianobar_status: PianobarStatus,
    curl_status: CurlStatus,
    song: Song,
    rating: i32,
    detail_url: Option<String>,
    stations: Vec<String>,
}

// Option<String>::filter will return a &String.
#[allow(clippy::ptr_arg)]
fn empty(string: &String) -> bool {
    !string.is_empty()
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid event info")]
pub struct ParseEventInfoError;

impl FromStr for Info {
    type Err = ParseEventInfoError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Splitting at "stationCount" allows us to treat the station array
        // differently from the rest of the key-value pairs.
        let Some((info, stations)) = input.split_once("stationCount=") else {
            return Err(ParseEventInfoError);
        };

        // Parse info into a HashMap so we don't have to trust the order of
        // key-value pairs returned by pianobar.
        let mut info: HashMap<&str, String> = info
            .lines()
            .map(|line| match line.split_once('=') {
                Some((key, value)) => Ok((key, value.to_string())),
                None => Err(ParseEventInfoError),
            })
            .collect::<Result<_, _>>()?;

        let artist = info.remove("artist").filter(empty);
        let title = info.remove("title").filter(empty);
        let album = info.remove("album").filter(empty);
        let cover_art = info.remove("coverArt").filter(empty);
        let station_name = info.remove("stationName").filter(empty);
        let song_station_name = info.remove("songStationName").filter(empty);

        let pianobar_status = PianobarStatus {
            code: info
                .remove("pRet")
                .ok_or(ParseEventInfoError)?
                .parse()
                .map_err(|_| ParseEventInfoError)?,
            message: info.remove("pRetStr").ok_or(ParseEventInfoError)?,
        };

        let curl_status = CurlStatus {
            code: info
                .remove("wRet")
                .ok_or(ParseEventInfoError)?
                .parse()
                .map_err(|_| ParseEventInfoError)?,
            message: info.remove("wRetStr").ok_or(ParseEventInfoError)?,
        };

        let song = Song {
            duration: info
                .remove("songDuration")
                .ok_or(ParseEventInfoError)?
                .parse()
                .map_err(|_| ParseEventInfoError)?,
            played: info
                .remove("songPlayed")
                .ok_or(ParseEventInfoError)?
                .parse()
                .map_err(|_| ParseEventInfoError)?,
        };

        let rating = info
            .remove("rating")
            .ok_or(ParseEventInfoError)?
            .parse()
            .map_err(|_| ParseEventInfoError)?;
        let detail_url = info.remove("detailUrl").filter(empty);

        let mut stations = stations.lines();
        // Skip the line which contains the station count, we don't need it.
        stations.next();
        // Parse the rest of the station array.
        let stations: Vec<String> = stations
            .map(|line| match line.split_once('=') {
                Some((_, station)) => Ok(station.into()),
                None => Err(ParseEventInfoError),
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
