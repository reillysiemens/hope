//! Pianobar eventcmd parsing.
//!
//! Pianobar's eventcmd types are described in [`pianobar(1)`](https://github.com/PromyLOPh/pianobar/blob/master/contrib/pianobar.1).

use std::str::FromStr;

/// An error which can be returned when parsing an eventcmd.
#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Invalid eventcmd: {0}")]
pub struct ParseEventCmdError(String);

/// A pianobar eventcmd.
#[derive(Debug, Clone, PartialEq)]
pub enum EventCmd {
    ArtistBookmark,
    SettingsChange,
    SettingsGet,
    SongBan,
    SongBookmark,
    SongExplain,
    SongFinish,
    SongLove,
    SongShelf,
    SongStart,
    StationAddGenre,
    StationAddMusic,
    StationAddShared,
    StationCreate,
    StationDelete,
    StationDeleteArtistSeed,
    StationDeleteFeedback,
    StationDeleteSongSeed,
    StationDeleteStationSeed,
    StationFetchGenre,
    StationFetchInfo,
    StationFetchPlaylist,
    StationGetModes,
    StationQuickMixToggle,
    StationRename,
    StationSetMode,
    UserGetStations,
    UserLogin,
}

impl FromStr for EventCmd {
    type Err = ParseEventCmdError;

    fn from_str(eventcmd: &str) -> Result<Self, Self::Err> {
        Ok(match eventcmd {
            "artistbookmark" => Self::ArtistBookmark,
            "settingschange" => Self::SettingsChange,
            "settingsget" => Self::SettingsGet,
            "songban" => Self::SongBan,
            "songbookmark" => Self::SongBookmark,
            "songexplain" => Self::SongExplain,
            "songfinish" => Self::SongFinish,
            "songlove" => Self::SongLove,
            "songshelf" => Self::SongShelf,
            "songstart" => Self::SongStart,
            "stationaddgenre" => Self::StationAddGenre,
            "stationaddmusic" => Self::StationAddMusic,
            "stationaddshared" => Self::StationAddShared,
            "stationcreate" => Self::StationCreate,
            "stationdelete" => Self::StationDelete,
            "stationdeleteartistseed" => Self::StationDeleteArtistSeed,
            "stationdeletefeedback" => Self::StationDeleteFeedback,
            "stationdeletesongseed" => Self::StationDeleteSongSeed,
            "stationdeletestationseed" => Self::StationDeleteStationSeed,
            "stationfetchgenre" => Self::StationFetchGenre,
            "stationfetchinfo" => Self::StationFetchInfo,
            "stationfetchplaylist" => Self::StationFetchPlaylist,
            "stationgetmodes" => Self::StationGetModes,
            "stationquickmixtoggle" => Self::StationQuickMixToggle,
            "stationrename" => Self::StationRename,
            "stationsetmode" => Self::StationSetMode,
            "usergetstations" => Self::UserGetStations,
            "userlogin" => Self::UserLogin,
            _ => return Err(ParseEventCmdError(eventcmd.to_string())),
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    use super::{EventCmd, ParseEventCmdError};

    #[test]
    fn invalid_eventcmd() {
        let invalid = "lolwut".to_string();
        let result: Result<EventCmd, ParseEventCmdError> = invalid.parse();
        assert_eq!(result, Err(ParseEventCmdError(invalid)));
    }

    #[test_case("artistbookmark", EventCmd::ArtistBookmark ; "artistbookmark")]
    #[test_case("settingschange", EventCmd::SettingsChange ; "settingschange")]
    #[test_case("settingsget", EventCmd::SettingsGet ; "settingsget")]
    #[test_case("songban", EventCmd::SongBan ; "songban")]
    #[test_case("songbookmark", EventCmd::SongBookmark ; "songbookmark")]
    #[test_case("songexplain", EventCmd::SongExplain ; "songexplain")]
    #[test_case("songfinish", EventCmd::SongFinish ; "songfinish")]
    #[test_case("songlove", EventCmd::SongLove ; "songlove")]
    #[test_case("songshelf", EventCmd::SongShelf ; "songshelf")]
    #[test_case("songstart", EventCmd::SongStart ; "songstart")]
    #[test_case("stationaddgenre", EventCmd::StationAddGenre ; "stationaddgenre")]
    #[test_case("stationaddmusic", EventCmd::StationAddMusic ; "stationaddmusic")]
    #[test_case("stationaddshared", EventCmd::StationAddShared ; "stationaddshared")]
    #[test_case("stationcreate", EventCmd::StationCreate ; "stationcreate")]
    #[test_case("stationdelete", EventCmd::StationDelete ; "stationdelete")]
    #[test_case("stationdeleteartistseed", EventCmd::StationDeleteArtistSeed ; "stationdeleteartistseed")]
    #[test_case("stationdeletefeedback", EventCmd::StationDeleteFeedback ; "stationdeletefeedback")]
    #[test_case("stationdeletesongseed", EventCmd::StationDeleteSongSeed ; "stationdeletesongseed")]
    #[test_case("stationdeletestationseed", EventCmd::StationDeleteStationSeed ; "stationdeletestationseed")]
    #[test_case("stationfetchgenre", EventCmd::StationFetchGenre ; "stationfetchgenre")]
    #[test_case("stationfetchinfo", EventCmd::StationFetchInfo ; "stationfetchinfo")]
    #[test_case("stationfetchplaylist", EventCmd::StationFetchPlaylist ; "stationfetchplaylist")]
    #[test_case("stationgetmodes", EventCmd::StationGetModes ; "stationgetmodes")]
    #[test_case("stationquickmixtoggle", EventCmd::StationQuickMixToggle ; "stationquickmixtoggle")]
    #[test_case("stationrename", EventCmd::StationRename ; "stationrename")]
    #[test_case("stationsetmode", EventCmd::StationSetMode ; "stationsetmode")]
    #[test_case("usergetstations", EventCmd::UserGetStations ; "usergetstations")]
    #[test_case("userlogin", EventCmd::UserLogin ; "userlogin")]
    fn valid_eventcmd(eventcmd: &str, expected: EventCmd) {
        let eventcmd: EventCmd = eventcmd.parse().expect("Invalid event type");
        assert_eq!(eventcmd, expected);
    }
}
