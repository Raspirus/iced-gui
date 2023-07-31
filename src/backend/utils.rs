use chrono::{Utc, DateTime, Local};
use directories_next::ProjectDirs;
use flume::Sender;
use job_scheduler_ng::{Job, JobScheduler};
use log::{error, info, warn};
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::Write,
    iter::once,
    path::Path,
    process::exit,
    time::{self, Duration}
};
use tokio::runtime::Runtime;

#[cfg(windows)]
use std::os::windows::prelude::OsStrExt;
#[cfg(windows)]
use winapi::um::fileapi::GetDriveTypeW;
#[cfg(windows)]
use winapi::um::winbase::DRIVE_REMOVABLE;

use super::{config_file::Config, db_ops::DBOps, file_scanner};

pub struct Utils {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UsbDevice {
    pub name: String,
    pub path: String,
}

impl Utils {
    pub async fn start_scanner(
        path: String,
        sender: Option<Sender<f32>>,
    ) -> Result<Vec<String>, String> {
        info!("Started Virus scanner on the backend");
        let mut fs = match file_scanner::FileScanner::new(&path, sender) {
            Ok(fs) => fs,
            Err(err) => {
                error!("{}", err);
                return Err(err.to_string());
            }
        };
        let config = Config::new();
        let obfuscated = config.obfuscated_is_active;
        let dirty_files = match fs.search_files(obfuscated) {
            Ok(files) => files,
            Err(e) => {
                error!("{}", e);
                return Err(e);
            }
        };

        Ok(dirty_files)
    }

    pub async fn update_database(sender: Option<Sender<f32>>) -> Result<String, String> {
        info!("Started Database update on the backend");
        let mut db_connection = match DBOps::new(sender) {
            Ok(db_conn) => db_conn,
            Err(err) => {
                error!("{err}");
                return Err(err.to_string());
            }
        };

        let big_tic = time::Instant::now();
        match tokio::task::spawn_blocking(move || {
            let hash_count;
            match db_connection.update_db() {
                Ok(res) => {
                    hash_count = res;
                }
                Err(err) => {
                    error!("{err}");
                    exit(-1)
                }
            }
            hash_count
        })
        .await
        {
            Ok(res) => {
                let big_toc = time::Instant::now();
                info!(
                    "Updated DB in {} seconds",
                    big_toc.duration_since(big_tic).as_secs_f64()
                );
                Ok(serde_json::to_string(&res).unwrap_or_default())
            }
            Err(err) => {
                error!("{err}");
                return Err(err.to_string());
            }
        }
    }

    pub fn list_usb_drives() -> Result<Vec<UsbDevice>, String> {
        let mut usb_drives = Vec::new();

        if cfg!(target_os = "linux") {
            info!("Trying to retrieve USB drives from Linux OS");
            let username = match env::var("USER") {
                Ok(val) => val,
                Err(_) => panic!("Could not get current username"),
            };

            let dir_path = format!("/media/{}", username);
            let entries = match fs::read_dir(dir_path) {
                Ok(entries) => entries,
                Err(err) => {
                    return Err(err.to_string());
                }
            };

            for entry in entries {
                let entry = entry.expect("I couldn't read something inside the directory");
                let path = entry.path();

                usb_drives.push(UsbDevice {
                    name: entry
                        .file_name()
                        .into_string()
                        .expect("File name is strange"),
                    path: path
                        .as_path()
                        .to_str()
                        .expect("Path is strange")
                        .to_string(),
                });
            }
        } else if cfg!(target_os = "windows") {
            info!("Trying to retrieve USB drives from Windows OS");
            let drive_letters: Vec<OsString> = vec![
                OsString::from("A"),
                OsString::from("B"),
                OsString::from("C"),
                OsString::from("D"),
                OsString::from("E"),
                OsString::from("F"),
                OsString::from("G"),
                OsString::from("H"),
                OsString::from("I"),
                OsString::from("J"),
                OsString::from("K"),
                OsString::from("L"),
                OsString::from("M"),
                OsString::from("N"),
                OsString::from("O"),
                OsString::from("P"),
                OsString::from("Q"),
                OsString::from("R"),
                OsString::from("S"),
                OsString::from("T"),
                OsString::from("U"),
                OsString::from("V"),
                OsString::from("W"),
                OsString::from("X"),
                OsString::from("Y"),
                OsString::from("Z"),
            ];
            for letter in drive_letters {
                let drive_path = letter.clone().into_string().unwrap() + ":\\";
                let drive_path = Path::new(&drive_path);
                let drive_name = drive_path.file_name().unwrap_or_default();
                let drive_path = drive_path.to_str().unwrap();

                #[cfg(windows)]
                let wide_path = OsStr::new(&drive_path)
                    .encode_wide()
                    .chain(once(0))
                    .collect::<Vec<_>>();
                #[cfg(windows)]
                let drive_type = unsafe { GetDriveTypeW(wide_path.as_ptr()) };

                match fs::metadata(drive_path) {
                    Ok(metadata) =>
                    {
                        #[cfg(windows)]
                        if metadata.is_dir() && drive_type == DRIVE_REMOVABLE {
                            info!("Found Drive: {}", drive_path);
                            usb_drives.push(UsbDevice {
                                name: drive_path.to_string() + " " + &drive_name.to_string_lossy(),
                                path: drive_path.to_string(),
                            });
                        }
                    }
                    Err(_) => {}
                }
            }
        } else {
            warn!("Not retrieving USBs -> Wrong OS");
            return Err("Not retrieving USBs, wrong OS".to_string());
        }
        Ok(usb_drives)
    }

    pub async fn auto_update_scheduler(hour: i32, weekday: i32) {
        // ISSUE: Needs to restart app to apply new update schedule

        // In cron, the time is in 24h format, while the weekday starts at 0 = sunday and 6 = saturday
        let mut scheduler = JobScheduler::new();

        // Construct the cron-like syntax using the given hour and weekday
        let cron_schedule = format!("0 {} * * {}", hour, weekday);

        scheduler.add(Job::new(
            cron_schedule.parse().expect("Given CronSyntax is invalid"),
            move || {
                // Check current time and use it for the name of the update logs file
                let now: DateTime<Local> = Local::now();
                let now_str = now.format("%Y_%m_%d_%H_%M_%S").to_string();
                let log_str = format!("{}.log", now_str);
                // Write to logs that the update function has started
                let message = format!("{} DB update executed\n", Utc::now());
                Self::log_update_res(&message, log_str.clone()).expect("Failed to write update logs to file.");
                // Execute the async function using Tokio's Runtime
                let runtime = Runtime::new().expect("Unable to create AutoUpdate Runtime");
                match runtime.block_on(Self::update_database(None)) {
                    Ok(result) => {
                        let message = format!("{} DB update finished\n", Utc::now());
                        Self::log_update_res(&message, log_str.clone())
                            .expect("Failed to write update logs to file.");
                        info!("AutoUpdate finished with: {}", result);
                    }
                    Err(error) => {
                        let message = format!("{} DB update error {}\n", Utc::now(), error);
                        Self::log_update_res(&message, log_str.clone())
                            .expect("Failed to write update logs to file.");
                        error!("AutoUpdate failed with: {}", error)
                    }
                };
            },
        ));

        // Block the main thread to keep the program running until terminated
        loop {
            scheduler.tick();
            std::thread::sleep(Duration::from_millis(500));
        }
    }

    fn log_update_res(data: &str, fname: String) -> std::io::Result<()> {
        let project_dirs = ProjectDirs::from("com", "Raspirus", "Logs").expect("Failed to get project directories.");
        let log_dir = project_dirs.data_local_dir().join("updates"); 
        // Open the file (creates if it doesn't exist)
        let mut file = File::create(log_dir.join(fname)).unwrap();
        // Write the data to the file
        file.write_all(data.as_bytes())?;
        // Flush the buffer to ensure all data is written
        file.flush()?;
        Ok(())
    }
}
