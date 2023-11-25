// extra stuff

use std::{str::FromStr, collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}, env, io, error::Error, io::Read, path, fs, f64::consts::PI, time::{Duration, SystemTime, UNIX_EPOCH}};
#[cfg(feature = "extras_rand")]
use rand::Rng;
use mime_guess;
//use serde::{Serialize, Deserialize};
//use nalgebra;

// User input, defines read!(&str) macro
#[macro_use] extern crate text_io;

const WEBLIB_SHORTCUT: &str = "/_weblib";

pub mod extra_math {
	pub fn clamp<T: PartialOrd>(n: T, min: T, max: T) -> T {
		assert!(max >= min, "Min must be <= Max");
		if n > min {
			if n < max {
				n
			}
			else {
				max
			}
		}
		else {
			min
		}
	}
	pub fn sigmoid(n: f64) -> f64 {
		clamp(n, -1.0, 1.0)// TODO: fix
	}
}

pub fn get_cwd() -> String {// https://stackoverflow.com/questions/37388107/how-to-convert-the-pathbuf-to-string
	let cwd = env::current_dir().unwrap();
	return cwd.into_os_string().into_string().unwrap();// gross
}

pub fn deg(r: f64) -> f64 {
	(r * 180.0) / PI
}

pub fn rad(d: f64) -> f64 {
	(d * PI) / 180.0
}

#[cfg(feature = "extras_rand")]
pub fn rand_unit() -> f64 {
	(rand::thread_rng().gen_range(0..1000000000) as f64) / 1000000000.0
}

#[cfg(feature = "extras_rand")]
pub fn rand_bool() -> bool {
	rand::thread_rng().gen_range(0..2) != 0
}

/*pub fn get_unix_timestamp(i: Instant) -> f64 {
	get_secs(i.duration_since(UNIX_EPOCH))
}*/

pub fn get_secs(i: Duration) -> f64 {
	i.as_micros() as f64 / 1_000_000.0
}

pub fn from_secs(secs: f64) -> Duration {
	Duration::from_secs(secs.floor() as u64) + Duration::from_nanos(((secs % 1.0) * 1_000_000_000.0) as u64)
}

pub fn prompt(s: &str) -> String {
	print!("{}: ", s);
	let res: String = read!("{}\n");
	res.trim().to_owned()
}

pub fn get_unix_ts_secs() -> f64 {
	let start = SystemTime::now();
    get_secs(start.duration_since(UNIX_EPOCH).expect("Time went backwards"))
}

pub fn get_unix_ts_secs_u64() -> u64 {
	let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}

pub fn sign(n: f64) -> i32 {
	match n >= 0.0 {
		true => 1,
		false => -1
	}
}

pub fn decode_url_query(query: &str) -> Result<HashMap<String, String>, String> {
	// Copied from evolution_model main.rs
	// TODO: return error
	let mut out = HashMap::new();
	for pair in query.split("&") {
		let key = pair.split("=").collect::<Vec<&str>>()[0];
		let value = pair.split("=").collect::<Vec<&str>>()[1];
		out.insert(key.to_owned(), value.to_owned());
	}
	Ok(out)
}

pub fn http_query_file(root: &str, http_file_path: String) -> (Vec<u8>, u16, String) {
	// This was copied from evolution_model::resources
	// data, status code, MIME type
	// Check if it is /_weblib
	let mut file_path: String = if http_file_path.contains(WEBLIB_SHORTCUT) {
		http_file_path.replace(
			"/_weblib",
			&(match env::home_dir() {
				Some(path) => path.display().to_string(),
				None => {return (b"Could not get home directory on server".to_vec(), 500, "text/html".to_owned())},
			} + "/python_lib/http/weblib")
		)
	}
	else {
		root.to_owned() + &http_file_path
	}.to_owned();
    if path::Path::new(&file_path).exists() {
        // https://www.dotnetperls.com/read-bytes-rust
        if path::Path::new(&file_path).is_dir() {
            file_path.push_str("/index.html");
        }
        let f = match fs::File::open(&file_path) {
            Ok(f) => f,
            Err(_) => panic!("Could not open file '{}'", file_path)
        };
        let mut reader = io::BufReader::new(f);
        let mut buffer = Vec::new();
        // Read file into vector
        reader.read_to_end(&mut buffer).expect(&format!("Could not load file {file_path} into Vec<u8>"));
        (buffer, 200, mime_guess::from_path(&file_path).first_or(mime_guess::Mime::from_str("text/html").unwrap()).to_string())
    }
    else {
        (format!("\"{file_path}\" Not Found").into_bytes(), 404, "text/html".to_owned())
    }
}

pub fn to_string_err<T, E: Error>(result: Result<T, E>) -> Result<T, String> {
	match result {
		Ok(t) => Ok(t),
		Err(e) => Err(e.to_string())
	}
}

pub fn to_string_err_with_message<T, E: Error>(result: Result<T, E>, message: &str) -> Result<T, String> {
	match result {
		Ok(t) => Ok(t),
		Err(e) => Err(format!("Message: {}, Error: {}", message, e.to_string()))
	}
}

pub fn option_to_result<T>(option: Option<T>, error_message: &str) -> Result<T, String> {// ChatGPT
    match option {
        Some(value) => Ok(value),
        None => Err(error_message.to_string()),
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {// https://doc.rust-lang.org/std/hash/index.html
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// I found out it is possible to directly serialize nalgebra structs
/*#[derive(Serialize, Deserialize)]
pub struct QuaternionSave<T> (
	T, T, T, T
);

impl<T: Copy> QuaternionSave<T> {
	pub fn load(&self) -> nalgebra::geometry::Quaternion<T> {
		nalgebra::geometry::Quaternion {
			coords: nalgebra::base::Vector4::<T>::new(self.0, self.1, self.2, self.3)
		}
	}
	pub fn save(q: nalgebra::geometry::Quaternion<T>) -> Self {
		let c = &q.coords.data;
		Self::<T> (
			c[(0, 0)],
			c.1,
			c.2,
			c.3
		)
	}
}*/

pub fn remove_dups<T>(v: &mut Vec<T>)// From ChatGPT
where
    T: PartialEq + Clone, // T needs to implement Clone for this approach
{
    let mut unique_items = Vec::new();
    let mut index = 0;

    while index < v.len() {
        let item = v[index].clone(); // Clone the item for comparison

        if !unique_items.contains(&item) {
            unique_items.push(item.clone());
            index += 1;
        } else {
            v.remove(index);
        }
    }
}

pub const FLOAT_TOLERANCE: f64 = 1.0e-6;

// Tests
#[cfg(test)]
mod tests {
	#[test]
	fn clamp() {
		assert_eq!(crate::extra_math::clamp(0.5, 0.0, 1.0), 0.5);
	}
	#[test]
	fn unix_ts() {
		println!("Unis epoch timestamp: {}", crate::get_unix_ts_secs());
	}
}
