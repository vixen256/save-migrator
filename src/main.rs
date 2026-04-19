use crossterm::event::*;
use ratatui::layout::*;
use ratatui::style::*;
use ratatui::widgets::*;
use std::io::Read;

fn main() {
	let mut folders = get_save_folders().unwrap();
	if folders.base.is_none() {
		if let Some(folder) = &folders.slp {
			folders.base = Some(folder.replace(
				"/DIVA/Project DIVA MEGA39's/Steam/",
				"/SEGA/Project DIVA MEGA39's/Steam/",
			));
		} else if let Some(folder) = &folders.eden {
			folders.base = Some(folder.replace(
				"/EDEN/Project DIVA MEGA39's/Steam/",
				"/SEGA/Project DIVA MEGA39's/Steam/",
			));
		} else {
			panic!("Could not find any save data!");
		}
		std::fs::create_dir_all(folders.base.as_ref().unwrap()).unwrap();
	}

	let mut base_song_count = {
		let folder = folders.base.as_ref().unwrap();
		let mut sys_count = 0;
		let mut dml_count = 0;
		if let Some(sys) = read_savefile(&folder, "sys.dat") {
			for i in 0..300 {
				let offset = 4 + i * 0x1330;
				let id = i32::from_le_bytes([
					sys[offset + 0],
					sys[offset + 1],
					sys[offset + 2],
					sys[offset + 3],
				]);
				if id > 0 {
					sys_count += 1;
				}
			}
		}
		if let Some(dml) = read_savefile(&folder, "DivaModLoader.dat") {
			dml_count = u32::from_le_bytes([dml[8], dml[9], dml[10], dml[11]]) as usize;
		}

		Some((sys_count, dml_count))
	};

	let slp_song_count = if let Some(folder) = &folders.slp {
		let mut sys_count = 0;
		let mut dml_count = 0;
		if let Some(sys) = read_savefile(&folder, "sys.dat") {
			for i in 0..2000 {
				let offset = 4 + i * 0x1330;
				let id = i32::from_le_bytes([
					sys[offset + 0],
					sys[offset + 1],
					sys[offset + 2],
					sys[offset + 3],
				]);
				if id > 0 {
					sys_count += 1;
				}
			}
		}
		if let Some(dml) = read_savefile(&folder, "DivaModLoader.dat") {
			dml_count = u32::from_le_bytes([dml[8], dml[9], dml[10], dml[11]]) as usize;
		}

		Some((sys_count, dml_count))
	} else {
		None
	};

	let eden_song_count = if let Some(folder) = &folders.eden {
		let mut sys_count = 0;
		let mut dml_count = 0;
		if let Some(sys) = read_savefile(&folder, "sys.dat") {
			for i in 0..2000 {
				let offset = 4 + i * 0x1330;
				let id = i32::from_le_bytes([
					sys[offset + 0],
					sys[offset + 1],
					sys[offset + 2],
					sys[offset + 3],
				]);
				if id > 0 {
					sys_count += 1;
				}
			}
		}
		if let Some(dml) = read_savefile(&folder, "DivaModLoader.dat") {
			dml_count = u32::from_le_bytes([dml[8], dml[9], dml[10], dml[11]]) as usize;
		}

		Some((sys_count, dml_count))
	} else {
		None
	};

	let mut state = TableState::new().with_selected(0);
	let mut show_failed_popup = false;
	let mut show_warning_popup = false;

	ratatui::run(|terminal| {
		loop {
			terminal.draw(|frame| {
				let mut rows = Vec::new();

				if let Some((sys_count, dml_count)) = &base_song_count {
					rows.push(
						Row::new([
							Cell::from("Base"),
							Cell::from(sys_count.to_string()),
							Cell::from(dml_count.to_string()),
						])
						.style(
							Style::new()
								.fg(palette::tailwind::SLATE.c200)
								.bg(palette::tailwind::SLATE.c950),
						),
					);
				}

				if let Some((sys_count, dml_count)) = &slp_song_count {
					rows.push(
						Row::new([
							Cell::from("Song Limit Patch"),
							Cell::from(sys_count.to_string()),
							Cell::from(dml_count.to_string()),
						])
						.style(
							Style::new()
								.fg(palette::tailwind::SLATE.c200)
								.bg(palette::tailwind::SLATE.c950),
						),
					);
				}

				if let Some((sys_count, dml_count)) = &eden_song_count {
					rows.push(
						Row::new([
							Cell::from("Eden Core"),
							Cell::from(sys_count.to_string()),
							Cell::from(dml_count.to_string()),
						])
						.style(
							Style::new()
								.fg(palette::tailwind::SLATE.c200)
								.bg(palette::tailwind::SLATE.c950),
						),
					);
				}

				let t = Table::new(
					rows,
					[
						Constraint::Percentage(50),
						Constraint::Percentage(25),
						Constraint::Percentage(25),
					],
				)
				.header(
					Row::new(["Name", "Songs in sys.dat", "Songs in DML.dat"]).style(
						Style::default()
							.fg(palette::tailwind::EMERALD.c900)
							.bg(palette::tailwind::EMERALD.c200),
					),
				)
				.row_highlight_style(
					Style::default()
						.add_modifier(Modifier::REVERSED)
						.fg(palette::tailwind::EMERALD.c400),
				)
				.bg(palette::tailwind::SLATE.c950)
				.highlight_spacing(HighlightSpacing::Always);

				let layout = Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
				let areas = frame.area().layout_vec(&layout);
				frame.render_stateful_widget(t, areas[0], &mut state);
				frame.render_widget(
					Paragraph::new(
						"ESC - Quit | ↑ - move up | ↓ - move down | ENTER - Overwrite base data with selected",
					)
					.style(
						Style::new()
							.fg(palette::tailwind::SLATE.c200)
							.bg(palette::tailwind::SLATE.c950),
					)
					.centered()
					.block(
						Block::bordered()
							.border_type(BorderType::Rounded)
							.border_style(Style::new().fg(palette::tailwind::EMERALD.c400)),
					),
					areas[1],
				);

				if show_failed_popup {
					let popup_block = Block::bordered()
						.border_type(BorderType::Rounded)
						.border_style(Style::new().fg(palette::tailwind::RED.c600))
						.title("Failed");
					let centered_area = frame
						.area()
						.centered(Constraint::Percentage(50), Constraint::Percentage(25));
					frame.render_widget(Clear, centered_area);

					frame.render_widget(
						Paragraph::new("Cannot overwrite base save with itself").block(popup_block),
						centered_area,
					);
				} else if show_warning_popup {
					let popup_block = Block::bordered()
						.border_type(BorderType::Rounded)
						.border_style(Style::new().fg(palette::tailwind::RED.c600))
						.title("Warning");
					let centered_area = frame
						.area()
						.centered(Constraint::Percentage(50), Constraint::Percentage(25));
					frame.render_widget(Clear, centered_area);

					frame.render_widget(
						Paragraph::new("This will overwrite ALL data in the base save.\nPress ESC to cancel\nPress Enter to Continue")
							.block(popup_block),
						centered_area,
					);
				}
			})?;
			if let Some(key) = crossterm::event::read()?.as_key_press_event() {
				match key.code {
					KeyCode::Char('q') | KeyCode::Esc => {
						if show_failed_popup || show_warning_popup {
							show_failed_popup = false;
							show_warning_popup = false;
						} else {
							break Ok::<(), Box<dyn std::error::Error>>(());
						}
					}
					KeyCode::Up => {
						if !show_failed_popup && !show_warning_popup {
							state.select_previous()
						}
					}
					KeyCode::Down => {
						if !show_failed_popup && !show_warning_popup {
							state.select_next()
						}
					}
					KeyCode::Enter => {
						if show_failed_popup {
							show_failed_popup = false;
						} else if show_warning_popup {
							show_warning_popup = false;
							if state.selected().unwrap_or_default() == 2 || folders.slp.is_none() {
							    convert_save(folders.eden.as_ref().unwrap(), folders.base.as_ref().unwrap());
							} else {
							    convert_save(folders.slp.as_ref().unwrap(), folders.base.as_ref().unwrap());
							}

							base_song_count = {
								let folder = folders.base.as_ref().unwrap();
								let mut sys_count = 0;
								let mut dml_count = 0;
								if let Some(sys) = read_savefile(&folder, "sys.dat") {
									for i in 0..300 {
										let offset = 4 + i * 0x1330;
										let id = i32::from_le_bytes([
											sys[offset + 0],
											sys[offset + 1],
											sys[offset + 2],
											sys[offset + 3],
										]);
										if id > 0 {
											sys_count += 1;
										}
									}
								}
								if let Some(dml) = read_savefile(&folder, "DivaModLoader.dat") {
									dml_count = u32::from_le_bytes([dml[8], dml[9], dml[10], dml[11]]) as usize;
								}

								Some((sys_count, dml_count))
							};
						} else if state.selected().unwrap_or_default() == 0 {
							show_failed_popup = true;
						} else {
							show_warning_popup = true;
						}
					}
					_ => {}
				}
			}
		}
	})
	.unwrap();
}

pub const UNCHANGED_SAVES: [&'static str; 13] = [
	"localsys.ldat",
	"NewClassics.dat",
	"pl.dat",
	"tshirt_module_00F.dat",
	"tshirt_module_00B.dat",
	"tshirt_module_01F.dat",
	"tshirt_module_01B.dat",
	"tshirt_module_02F.dat",
	"tshirt_module_02B.dat",
	"tshirt_module_03F.dat",
	"tshirt_module_03B.dat",
	"tshirt_module_04F.dat",
	"tshirt_module_04B.dat",
];

fn convert_save(in_folder: &str, out_folder: &str) {
	for filename in &UNCHANGED_SAVES {
		let (filename, extension) = filename.split_at(filename.rfind('.').unwrap());
		let filename =
			openssl::hash::hash(openssl::hash::MessageDigest::md5(), filename.as_bytes()).unwrap();
		let filename = filename
			.iter()
			.map(|c| {
				[
					char::from_digit(*c as u32 / 16, 16).unwrap(),
					char::from_digit(*c as u32 % 16, 16).unwrap(),
				]
			})
			.flatten()
			.collect::<String>();

		_ = std::fs::copy(
			format!("{in_folder}/{filename}{extension}"),
			format!("{out_folder}/{filename}{extension}"),
		);
	}

	let mut sys = read_savefile(in_folder, "sys.dat").unwrap();

	let mut sys_count = 0;
	for i in 0..300 {
		let offset = 4 + i * 0x1330;
		let id = i32::from_le_bytes([
			sys[offset + 0],
			sys[offset + 1],
			sys[offset + 2],
			sys[offset + 3],
		]);
		if id > 0 {
			sys_count += 1;
		}
	}

	if sys_count > 300 {
		let mut dml = match read_savefile(in_folder, "DivaModLoader.dat") {
			Some(data) => data,
			None => vec![
				0x01, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			],
		};
		let dml_count = u32::from_le_bytes([dml[8], dml[9], dml[10], dml[11]]) as usize;

		let extra_songs = &sys[(0x1330 * 300 + 4)..(0x1330 * sys_count + 4)];
		let mut extra_songs = extra_songs.to_vec();
		let header_size = u32::from_le_bytes([dml[4], dml[5], dml[6], dml[7]]) as usize;
		let mut data = dml.split_off(header_size);

		let song_count_bytes = (sys_count as u32 - 300 + dml_count as u32).to_le_bytes();
		dml[8] = song_count_bytes[0];
		dml[9] = song_count_bytes[1];
		dml[10] = song_count_bytes[2];
		dml[11] = song_count_bytes[3];

		dml.append(&mut extra_songs);
		dml.append(&mut data);

		save_savefile(out_folder, "DivaModLoader.dat", &dml).unwrap();
	} else {
		if let Some(data) = read_savefile(in_folder, "DivaModLoader.dat") {
			save_savefile(out_folder, "DivaModLoader.dat", &data).unwrap();
		}
	}

	let mut extra_songs = sys.split_off(0x1330 * 300 + 4);
	sys.append(&mut extra_songs.split_off(0x1330 * 1700));
	save_savefile(out_folder, "sys.dat", &sys).unwrap();
}

fn read_savefile(dir: &str, filename: &str) -> Option<Vec<u8>> {
	let key = generate_key(filename, get_steam_userid()?);
	let (filename, extension) = filename.split_at(filename.rfind('.').unwrap());
	let filename =
		openssl::hash::hash(openssl::hash::MessageDigest::md5(), filename.as_bytes()).ok()?;
	let filename = filename
		.iter()
		.map(|c| {
			[
				char::from_digit(*c as u32 / 16, 16).unwrap(),
				char::from_digit(*c as u32 % 16, 16).unwrap(),
			]
		})
		.flatten()
		.collect::<String>();

	let file = format!("{dir}/{filename}{extension}");
	let data = std::fs::read(&file).ok()?;

	let compressed = openssl::symm::decrypt(
		openssl::symm::Cipher::aes_128_cbc(),
		&key,
		Some(&[
			0xC2, 0x55, 0xFD, 0x73, 0xD8, 0x30, 0xFA, 0xEF, 0xD5, 0x32, 0x08, 0x54, 0xA2, 0x26,
			0x44, 0x14,
		]),
		&data,
	)
	.ok()?;

	let mut decoder = flate2::read::GzDecoder::new(&compressed[..]);
	let mut buf = Vec::new();
	decoder.read_to_end(&mut buf).ok()?;
	Some(buf)
}

fn save_savefile(dir: &str, filename: &str, data: &[u8]) -> Option<()> {
	let mut encoder = flate2::read::GzEncoder::new(&data[..], flate2::Compression::best());
	let mut compressed = Vec::new();
	encoder.read_to_end(&mut compressed).ok()?;

	let key = generate_key(filename, get_steam_userid()?);

	let encrypted = openssl::symm::encrypt(
		openssl::symm::Cipher::aes_128_cbc(),
		&key,
		Some(&[
			0xC2, 0x55, 0xFD, 0x73, 0xD8, 0x30, 0xFA, 0xEF, 0xD5, 0x32, 0x08, 0x54, 0xA2, 0x26,
			0x44, 0x14,
		]),
		&compressed,
	)
	.ok()?;

	let (filename, extension) = filename.split_at(filename.rfind('.').unwrap());
	let filename =
		openssl::hash::hash(openssl::hash::MessageDigest::md5(), filename.as_bytes()).ok()?;
	let filename = filename
		.iter()
		.map(|c| {
			[
				char::from_digit(*c as u32 / 16, 16).unwrap(),
				char::from_digit(*c as u32 % 16, 16).unwrap(),
			]
		})
		.flatten()
		.collect::<String>();

	let file = format!("{dir}/{filename}{extension}");
	std::fs::write(file, encrypted).ok()
}

fn generate_key(input: &str, steam_id: u64) -> [u8; 16] {
	let mut seed = 0i32;
	for char in input.chars() {
		seed = (seed >> 3) + (char as i32 * 10997 >> 5) * 23684;
	}

	for _ in 0..8 {
		seed = (seed >> 1) + 21404519;
	}

	for char in steam_id.to_string().chars() {
		seed = (seed >> 2) + (char as i32 * 47109 >> 5) * 13409;
	}

	let mut rand = RandState::seed(seed as u32);
	let mut out = [0u8; 16];
	for i in 0..16 {
		let next = rand.rand() as u8;
		let upper = (next & 15) / 10;
		let lower = (next & 15) % 10;
		out[i] = upper * 16 + lower;
	}
	out
}

fn get_steam_userid() -> Option<u64> {
	let steam_folder = get_steam_folder()?;
	let data = std::fs::read_to_string(format!("{steam_folder}/config/loginusers.vdf")).ok()?;
	let users = keyvalues_parser::Vdf::parse(&data).ok()?;

	users.value.get_obj()?.0.iter().find_map(|(id, user)| {
		if user
			.first()?
			.get_obj()?
			.get("MostRecent")?
			.first()?
			.get_str()?
			== "1"
		{
			id.parse::<u64>().ok()
		} else {
			None
		}
	})
}

#[derive(Debug)]
struct SaveFolders {
	base: Option<String>,
	slp: Option<String>,
	eden: Option<String>,
}

fn get_save_folders() -> Option<SaveFolders> {
	let user_id = get_steam_userid()?;

	#[cfg(target_os = "windows")]
	let dir = {
		let dir = dirs::config_dir()?;
		dir.to_string_lossy().to_string()
	};
	#[cfg(not(target_os = "windows"))]
	let dir = {
		let steam_folder = get_steam_folder()?;
		let data =
			std::fs::read_to_string(format!("{steam_folder}/config/libraryfolders.vdf")).ok()?;
		let libraries = keyvalues_parser::Vdf::parse(&data).ok()?;

		let dir = libraries
			.value
			.get_obj()?
			.0
			.iter()
			.find_map(|(_, library)| {
				if library
					.first()?
					.get_obj()?
					.get("apps")?
					.first()?
					.get_obj()?
					.contains_key("1761390")
				{
					Some(
						library
							.first()?
							.get_obj()?
							.get("path")?
							.first()?
							.get_str()?,
					)
				} else {
					None
				}
			})?;
		format!("{dir}/steamapps/compatdata/1761390/pfx/drive_c/users/steamuser/AppData/Roaming")
	};

	let mut folders = SaveFolders {
		base: None,
		slp: None,
		eden: None,
	};
	if std::path::Path::new(&format!("{dir}/SEGA")).exists() {
		folders.base = Some(format!("{dir}/SEGA/Project DIVA MEGA39's/Steam/{user_id}/"));
	}
	if std::path::Path::new(&format!("{dir}/DIVA")).exists() {
		folders.slp = Some(format!("{dir}/DIVA/Project DIVA MEGA39's/Steam/{user_id}/"));
	}
	if std::path::Path::new(&format!("{dir}/EDEN")).exists() {
		folders.eden = Some(format!("{dir}/EDEN/Project DIVA MEGA39's/Steam/{user_id}/"));
	}
	Some(folders)
}

// From R4D
fn get_steam_folder() -> Option<String> {
	#[cfg(target_os = "linux")]
	{
		let mut binding = dirs::home_dir()?;
		binding.push(".local/share/Steam");
		if !binding.exists() {
			binding = dirs::home_dir()?;
			binding.push(".var/app/com.valvesoftware.Steam/data/Steam");
			if !binding.exists() {
				return None;
			}
		}
		Some(binding.display().to_string())
	}
	#[cfg(target_os = "macos")]
	{
		let mut binding = dirs::home_dir()?;
		binding.push("Library/Application Support/Steam");
		if !binding.exists() {
			return None;
		}
		Some(binding.display().to_string())
	}
	#[cfg(target_os = "windows")]
	{
		let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
		let steam_key = hklm
			.open_subkey(r#"SOFTWARE\WOW6432Node\Valve\Steam"#)
			.ok()?;
		let res: std::io::Result<String> = steam_key.get_value("InstallPath");
		if let Ok(path) = res {
			if std::path::PathBuf::from(path.clone()).exists() {
				return Some(path.clone());
			} else {
				return Some(r#"C:\Program Files (x86)\Steam"#.to_string());
			}
		} else {
			return Some(r#"C:\Program Files (x86)\Steam"#.to_string());
		}
	}
}

// From ReDIVA
struct RandState {
	state_array: [u32; 624],
	state_index: u32,
}

const fn shift_val(a: u32, b: u32, c: u32) -> u32 {
	(0xffffffff & !((1 << (c - b)) - 1)) | (a >> b)
}

impl RandState {
	fn seed(val: u32) -> Self {
		let mut rand = Self {
			state_array: [0; 624],
			state_index: 0,
		};
		rand.state_array[0] = val;
		for i in 1..624 {
			rand.state_array[i] = 1812433253
				* (rand.state_array[i - 1] ^ (rand.state_array[i - 1] >> (32 - 2)))
				+ i as u32;
		}
		rand
	}

	fn rand(&mut self) -> u32 {
		let index = self.state_index as usize;
		let mut shift_index = index + 397;
		let mut next_index = index + 1;
		if shift_index >= 624 {
			shift_index -= 624;
			if next_index >= 624 {
				next_index = 0;
			}
		}

		let state = self.state_array[index];
		let mt_value = state ^ (state ^ self.state_array[next_index]) & (0xffffffff >> 1);
		let mt_value = self.state_array[shift_index]
			^ (-(mt_value as i32 & 1) as u32 & 0x9908b0df)
			^ (mt_value >> 1);
		self.state_array[index] = mt_value;
		self.state_index = next_index as u32;

		let mt_value = mt_value ^ (mt_value >> 11);
		let mt_value = mt_value ^ ((mt_value & shift_val(0x9d2c5680, 7, 32)) << 7);
		let mt_value = mt_value ^ ((mt_value & shift_val(0xefc60000, 15, 32)) << 15);
		let mt_value = mt_value ^ (mt_value >> 18);
		mt_value
	}
}
