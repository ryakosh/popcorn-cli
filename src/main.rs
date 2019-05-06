pub mod db;
pub mod types;
pub mod output;

#[macro_use] extern crate clap;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
extern crate serde_json;
use types::candidates;

fn main() {
    use types::CastJob;
    use candidates::{CastCandidate, MovieCandidate};

    let yaml = load_yaml!("args.yml");
    let m = clap::App::from_yaml(yaml).get_matches();

    if let Some(writersm) = m.subcommand_matches("writers") {
        if let Some(querym) = writersm.subcommand_matches("query") {
            let json =
                output::to_json_pretty(&db::writers_query(querym.value_of("SEARCH").unwrap()));
            println!("{}", json);
        } else if let Some(addm) = writersm.subcommand_matches("add") {
            let candidate = CastCandidate::new(CastJob::Writer,
                addm.value_of("FIRST").unwrap(),
                addm.value_of("LAST").unwrap(),
                addm.value_of("GENDER").unwrap());

            db::cast_add(&candidate);
        }

    } else if let Some(directorsm) = m.subcommand_matches("directors") {
        if let Some(querym) = directorsm.subcommand_matches("query") {
            let json =
                output::to_json_pretty(&db::directors_query(querym.value_of("SEARCH").unwrap()));
            println!("{}", json);
        } else if let Some(addm) = directorsm.subcommand_matches("add") {
            let candidate = CastCandidate::new(CastJob::Director,
                addm.value_of("FIRST").unwrap(),
                addm.value_of("LAST").unwrap(),
                addm.value_of("GENDER").unwrap());

            db::cast_add(&candidate);
        }
    } else if let Some(artistsm) = m.subcommand_matches("artists") {
        if let Some(querym) = artistsm.subcommand_matches("query") {
            let json =
                output::to_json_pretty(&db::artists_query(querym.value_of("SEARCH").unwrap()));
            println!("{}", json);
        } else if let Some(addm) = artistsm.subcommand_matches("add") {
            let candidate = CastCandidate::new(CastJob::Artist,
                addm.value_of("FIRST").unwrap(),
                addm.value_of("LAST").unwrap(),
                addm.value_of("GENDER").unwrap());

            db::cast_add(&candidate);
        }
    } else if let Some(moviesm) = m.subcommand_matches("movies") {
        let candidate = MovieCandidate::new(moviesm.value_of("TITLE").unwrap(),
                                            moviesm.value_of("DESCRIPTION").unwrap(),
                                            moviesm.value_of("POSTER").unwrap(),
                                            moviesm.value_of("GENRES").unwrap(),
                                            moviesm.value_of("LANGUAGES").unwrap(),
                                            moviesm.value_of("RELEASE_COUNTRY").unwrap(),
                                            moviesm.value_of("RELEASE_DATE").unwrap(),
                                            moviesm.value_of("DURATION").unwrap(),
                                            moviesm.value_of("WRITERS").unwrap(),
                                            moviesm.value_of("DIRECTORS").unwrap(),
                                            moviesm.value_of("ARTISTS").unwrap());
        
        db::movies_add(&candidate);
    }
}