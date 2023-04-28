use crate::cli::day::Day;
use crate::cli::infomation::Infomation;
use crate::cli::reserve::Reserve;
use structopt::StructOpt;

///Command line arguments
#[derive(Debug, StructOpt)]
pub enum Action {
    ///Query library site or student
    Query {
        /// the day to query
        #[structopt(
        short,
        long,
        possible_values = &["today","tomorrow"],
        case_insensitive = true,
        default_value = "today"
        )]
        day: Day,

        /// the name to query
        #[structopt(short, long)]
        name: Option<String>,

        /// the site to query
        #[structopt(short, long)]
        site: Option<String>,
    },

    ///Login library
    Login {
        /// username to login
        #[structopt(short, long, env = "NJFUUSERNAME")]
        username: String,

        /// password to login
        #[structopt(short, long, env = "NJFUPASSWORD", hide_env_values = true)]
        password: String,
    },

    ///list personal status
    State {},

    ///cancel the reservation
    Cancel {
        /// the id of the reservation to cancel
        id: String,
    },

    ///reserve a site
    Reserve(Reserve),

    /// check in (not support yet)
    In { _id: String },

    /// check out
    Out { id: String },

    /// show info
    Info {
        #[structopt(
            possible_values = &["floor","author","user"],
            case_insensitive = true,
            default_value = "floor"
        )]
        infomation: Infomation,
    },
}
