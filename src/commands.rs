use std::error::Error;
use std::fs;

pub fn handle_command(input: &str) -> Result<Option<String>, Box<dyn Error>> {
    let input = input.trim();
    if input.eq_ignore_ascii_case("list files in current directory") {
        return list_files();
    } else if let Some(expr) = input.strip_prefix("calculate ") {
        return calculate(expr);
    } else if input.eq_ignore_ascii_case("plot a sine wave") {
        return plot_sine_wave();
    } else if input.eq_ignore_ascii_case("show current directory") {
        return show_current_directory();
    } else if let Some(path) = input.strip_prefix("change directory to ") {
        return change_directory(path.trim());
    } else if let Some(filename) = input.strip_prefix("show file ") {
        return show_file(filename.trim());
    } else if let Some(rest) = input.strip_prefix("write to file ") {
        if let Some((filename, content)) = rest.split_once(":") {
            return write_file(filename.trim(), content.trim());
        }
    } else if let Some(filename) = input.strip_prefix("delete file ") {
        return delete_file(filename.trim());
    } else if input.eq_ignore_ascii_case("show system info") {
        return show_system_info();
    } else if input.eq_ignore_ascii_case("show date and time") {
        return show_date_time();
    } else if input.eq_ignore_ascii_case("help") {
        return help();
    } else if let Some(rest) = input.strip_prefix("search ") {
        if let Some((pattern, target)) = rest.trim().strip_prefix('"').and_then(|s| s.split_once('"')) {
            if let Some(target) = target.trim().strip_prefix("in ") {
                return search_pattern(pattern, target.trim());
            }
        }
    } else if let Some(filename) = input.strip_prefix("summarize file ") {
        return summarize_file(filename.trim());
    } else if let Some(rest) = input.strip_prefix("count ") {
        if let Some(filename) = rest.strip_prefix("lines in ") {
            return count_file_stats(filename.trim(), "lines");
        } else if let Some(filename) = rest.strip_prefix("words in ") {
            return count_file_stats(filename.trim(), "words");
        } else if let Some(filename) = rest.strip_prefix("chars in ") {
            return count_file_stats(filename.trim(), "chars");
        }
    } else if input.eq_ignore_ascii_case("show disk usage") {
        return show_disk_usage();
    } else if input.eq_ignore_ascii_case("show top processes") {
        return show_top_processes();
    } else if let Some(rest) = input.strip_prefix("download ") {
        if let Some((url, filename)) = rest.split_once(" to ") {
            return download_file(url.trim(), filename.trim());
        }
    } else if let Some(rest) = input.strip_prefix("extract ") {
        if let Some((archive, dir)) = rest.split_once(" to ") {
            return extract_archive(archive.trim(), dir.trim());
        }
    } else if let Some(length) = input.strip_prefix("generate password ") {
        if let Ok(len) = length.trim().parse::<usize>() {
            return generate_password(len);
        }
    } else if input.eq_ignore_ascii_case("show calendar") {
        return show_calendar();
    } else if let Some(city) = input.strip_prefix("show weather in ") {
        return show_weather(city.trim());
    } else if let Some(cmd) = input.strip_prefix("explain ") {
        if let Some(shell_cmd) = cmd.trim().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            return explain_shell_command(shell_cmd.trim());
        }
    } else if let Some(instr) = input.strip_prefix("run ") {
        if let Some(nl_cmd) = instr.trim().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            return run_shell_command(nl_cmd.trim());
        }
    } else if let Some(rest) = input.strip_prefix("run code ") {
        if let Some((lang, code)) = rest.split_once(":") {
            return run_code_snippet(lang.trim(), code.trim());
        }
    } else if let Some(rest) = input.strip_prefix("diff ") {
        let mut parts = rest.split_whitespace();
        if let (Some(file1), Some(file2)) = (parts.next(), parts.next()) {
            return file_diff(file1, file2);
        }
    } else if let Some(rest) = input.strip_prefix("rename files in ") {
        if let Some((dir_and_pattern, replacement)) = rest.split_once(" to ") {
            if let Some((dir, pattern)) = dir_and_pattern.trim().split_once(" matching ") {
                if let Some(pattern) = pattern.trim().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    if let Some(replacement) = replacement.trim().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                        return batch_rename(dir.trim(), pattern, replacement);
                    }
                }
            }
        }
    } else if let Some(rest) = input.strip_prefix("resize image ") {
        if let Some((file, size)) = rest.split_once(" to ") {
            if let Some((w, h)) = size.split_once('x') {
                if let (Ok(width), Ok(height)) = (w.trim().parse::<u32>(), h.trim().parse::<u32>()) {
                    return resize_image(file.trim(), width, height);
                }
            }
        }
    } else if let Some(rest) = input.strip_prefix("convert image ") {
        if let Some((file, format)) = rest.split_once(" to ") {
            return convert_image(file.trim(), format.trim());
        }
    } else if let Some(file) = input.strip_prefix("play audio ") {
        return play_audio(file.trim());
    } else if let Some(rest) = input.strip_prefix("convert audio ") {
        if let Some((file, format)) = rest.split_once(" to ") {
            return convert_audio(file.trim(), format.trim());
        }
    } else if let Some(text) = input.strip_prefix("speak ") {
        if let Some(text) = text.trim().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            return speak_text(text.trim());
        }
    } else if let Some(rest) = input.strip_prefix("copy ") {
        if let Some((text, to_clipboard)) = rest.trim().split_once(" to clipboard") {
            if let Some(text) = text.trim().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                return copy_to_clipboard(text.trim());
            }
        }
    } else if input.trim() == "paste from clipboard" {
        return paste_from_clipboard();
    } else if let Some(target) = input.strip_prefix("watch ") {
        return watch_file_or_dir(target.trim());
    } else if let Some(rest) = input.strip_prefix("schedule ") {
        if let Some((cmd, at_time)) = rest.trim().strip_prefix("\"").and_then(|s| s.split_once("\" at ")) {
            return schedule_command(cmd.trim(), at_time.trim());
        }
    }
    Ok(None)
}

fn list_files() -> Result<Option<String>, Box<dyn Error>> {
    let entries = fs::read_dir(".")?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        files.push(file_name.to_string_lossy().to_string());
    }
    Ok(Some(files.join("\n")))
}

fn calculate(expr: &str) -> Result<Option<String>, Box<dyn Error>> {
    match meval::eval_str(expr) {
        Ok(result) => Ok(Some(result.to_string())),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

fn plot_sine_wave() -> Result<Option<String>, Box<dyn Error>> {
    use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries, WHITE, RED};
    let path = "sine_wave.png";
    let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Sine Wave", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..(2.0 * std::f32::consts::PI), -1.2f32..1.2f32)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        (0..1000).map(|x| {
            let x = x as f32 * 2.0 * std::f32::consts::PI / 1000.0;
            (x, x.sin())
        }),
        &RED,
    ))?;
    root.present()?;
    Ok(Some(format!("Plot saved to {}", path)))
}

fn show_current_directory() -> Result<Option<String>, Box<dyn Error>> {
    let cwd = std::env::current_dir()?;
    Ok(Some(cwd.display().to_string()))
}

fn change_directory(path: &str) -> Result<Option<String>, Box<dyn Error>> {
    std::env::set_current_dir(path)?;
    Ok(Some(format!("Changed directory to {}", path)))
}

fn show_file(filename: &str) -> Result<Option<String>, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filename)?;
    Ok(Some(contents))
}

fn write_file(filename: &str, content: &str) -> Result<Option<String>, Box<dyn Error>> {
    std::fs::write(filename, content)?;
    Ok(Some(format!("Wrote to file {}", filename)))
}

fn delete_file(filename: &str) -> Result<Option<String>, Box<dyn Error>> {
    std::fs::remove_file(filename)?;
    Ok(Some(format!("Deleted file {}", filename)))
}

fn show_system_info() -> Result<Option<String>, Box<dyn Error>> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let info = format!("OS: {}\nArch: {}", os, arch);
    Ok(Some(info))
}

fn show_date_time() -> Result<Option<String>, Box<dyn Error>> {
    let now = chrono::Local::now();
    Ok(Some(now.format("%Y-%m-%d %H:%M:%S").to_string()))
}

fn help() -> Result<Option<String>, Box<dyn Error>> {
    let help_text = r#"Available commands:
- list files in current directory
- show current directory
- change directory to <path>
- show file <filename>
- write to file <filename>: <content>
- delete file <filename>
- calculate <expression>
- plot a sine wave
- show system info
- show date and time
- help
"#;
    Ok(Some(help_text.to_string()))
}

fn search_pattern(pattern: &str, target: &str) -> Result<Option<String>, Box<dyn Error>> {
    use regex::Regex;
    use std::fs;
    use std::path::Path;
    let re = Regex::new(pattern)?;
    let mut results = Vec::new();
    fn search_file(path: &Path, re: &Regex, results: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        for (i, line) in content.lines().enumerate() {
            if re.is_match(line) {
                results.push(format!("{}:{}: {}", path.display(), i + 1, line));
            }
        }
        Ok(())
    }
    let path = Path::new(target);
    if path.is_file() {
        search_file(path, &re, &mut results)?;
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                search_file(&path, &re, &mut results)?;
            }
        }
    } else {
        return Ok(Some(format!("Target '{}' not found", target)));
    }
    if results.is_empty() {
        Ok(Some("No matches found.".to_string()))
    } else {
        Ok(Some(results.join("\n")))
    }
}

fn summarize_file(filename: &str) -> Result<Option<String>, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filename)?;
    let summary: Vec<_> = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .take(5)
        .collect();
    if summary.is_empty() {
        Ok(Some("File is empty or contains no summary lines.".to_string()))
    } else {
        Ok(Some(format!("Summary:\n{}", summary.join("\n"))))
    }
}

fn count_file_stats(filename: &str, stat: &str) -> Result<Option<String>, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filename)?;
    let result = match stat {
        "lines" => contents.lines().count().to_string(),
        "words" => contents.split_whitespace().count().to_string(),
        "chars" => contents.chars().count().to_string(),
        _ => "Invalid stat".to_string(),
    };
    Ok(Some(format!("{}: {}", stat, result)))
}

fn show_disk_usage() -> Result<Option<String>, Box<dyn Error>> {
    use std::fs;
    use std::path::Path;
    fn dir_size(path: &Path) -> u64 {
        if path.is_file() {
            fs::metadata(path).map(|m| m.len()).unwrap_or(0)
        } else if path.is_dir() {
            fs::read_dir(path)
                .map(|entries| {
                    entries.filter_map(|e| e.ok())
                        .map(|e| dir_size(&e.path()))
                        .sum()
                })
                .unwrap_or(0)
        } else {
            0
        }
    }
    let cwd = std::env::current_dir()?;
    let size = dir_size(&cwd);
    Ok(Some(format!("Disk usage for {}: {:.2} MB", cwd.display(), size as f64 / 1_048_576.0)))
}

fn show_top_processes() -> Result<Option<String>, Box<dyn Error>> {
    use sysinfo::{System, Process};
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
    let top: Vec<String> = processes.iter().take(5).map(|p| {
        format!("{} (pid {}): {:.2}% CPU, {} KB RAM", p.name(), p.pid(), p.cpu_usage(), p.memory())
    }).collect();
    Ok(Some(format!("Top processes:\n{}", top.join("\n"))))
}

fn download_file(url: &str, filename: &str) -> Result<Option<String>, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?;
    let bytes = resp.bytes()?;
    std::fs::write(filename, &bytes)?;
    Ok(Some(format!("Downloaded {} to {}", url, filename)))
}

fn extract_archive(archive: &str, dir: &str) -> Result<Option<String>, Box<dyn Error>> {
    use std::fs::File;
    use std::path::Path;
    let path = Path::new(archive);
    if archive.ends_with(".zip") {
        let file = File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = Path::new(dir).join(file.name());
            if file.is_dir() {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        Ok(Some(format!("Extracted {} to {}", archive, dir)))
    } else if archive.ends_with(".tar.gz") || archive.ends_with(".tgz") {
        let file = File::open(path)?;
        let decompressor = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decompressor);
        archive.unpack(dir)?;
        Ok(Some(format!("Extracted {} to {}", archive, dir)))
    } else {
        Ok(Some("Unsupported archive format. Only .zip and .tar.gz/.tgz supported.".to_string()))
    }
}

fn generate_password(length: usize) -> Result<Option<String>, Box<dyn Error>> {
    use rand::{distributions::Alphanumeric, Rng};
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    Ok(Some(password))
}

fn show_calendar() -> Result<Option<String>, Box<dyn Error>> {
    use chrono::{Datelike, Local, Weekday};
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let first = chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last = if month == 12 {
        chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap().pred_opt().unwrap()
    } else {
        chrono::NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().pred_opt().unwrap()
    };
    let mut cal = format!("{:^20}\nSu Mo Tu We Th Fr Sa\n", now.format("%B %Y"));
    let mut day = 1;
    let mut week = vec!["  "; 7];
    let first_weekday = first.weekday().num_days_from_sunday() as usize;
    for i in first_weekday..7 {
        week[i] = format!("{:2}", day);
        day += 1;
    }
    cal.push_str(&week.join(" "));
    cal.push('\n');
    while day <= last.day() {
        let mut week = vec![];
        for _ in 0..7 {
            if day > last.day() {
                week.push("  ".to_string());
            } else {
                week.push(format!("{:2}", day));
                day += 1;
            }
        }
        cal.push_str(&week.join(" "));
        cal.push('\n');
    }
    Ok(Some(cal))
}

fn show_weather(city: &str) -> Result<Option<String>, Box<dyn Error>> {
    let url = format!("https://wttr.in/{}?format=3", city.replace(' ', "+"));
    let resp = reqwest::blocking::get(&url)?.text()?;
    Ok(Some(resp))
}

fn explain_shell_command(cmd: &str) -> Result<Option<String>, Box<dyn Error>> {
    // Stub: In a real implementation, use an LLM or a shell explain API
    Ok(Some(format!("[Explanation for shell command: '{}']", cmd)))
}

fn run_shell_command(cmd: &str) -> Result<Option<String>, Box<dyn Error>> {
    use std::process::Command;
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd").args(["/C", cmd]).output()?;
    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh").args(["-c", cmd]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.trim().is_empty() {
        Ok(Some(format!("{}\n[stderr]: {}", stdout, stderr)))
    } else {
        Ok(Some(stdout.to_string()))
    }
}

fn run_code_snippet(lang: &str, code: &str) -> Result<Option<String>, Box<dyn Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;
    let (cmd, args) = match lang.to_lowercase().as_str() {
        "python" => ("python3", vec!["-c"]),
        "javascript" | "js" | "node" => ("node", vec!["-e"]),
        "bash" | "sh" => ("bash", vec![]),
        _ => return Ok(Some("Unsupported language. Supported: python, javascript, bash.".to_string())),
    };
    let mut child = if lang.to_lowercase() == "bash" || lang.to_lowercase() == "sh" {
        let mut child = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(code.as_bytes())?;
        }
        child
    } else {
        Command::new(cmd)
            .args(&args)
            .arg(code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    };
    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.trim().is_empty() {
        Ok(Some(format!("{}\n[stderr]: {}", stdout, stderr)))
    } else {
        Ok(Some(stdout.to_string()))
    }
}

fn file_diff(file1: &str, file2: &str) -> Result<Option<String>, Box<dyn Error>> {
    let text1 = std::fs::read_to_string(file1)?;
    let text2 = std::fs::read_to_string(file2)?;
    let diff = similar::TextDiff::from_lines(&text1, &text2)
        .unified_diff()
        .header(file1, file2)
        .to_string();
    Ok(Some(diff))
}

fn batch_rename(dir: &str, pattern: &str, replacement: &str) -> Result<Option<String>, Box<dyn Error>> {
    use regex::Regex;
    use std::fs;
    let re = Regex::new(pattern)?;
    let mut renamed = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
            if re.is_match(fname) {
                let new_name = re.replace_all(fname, replacement);
                let new_path = path.parent().unwrap().join(new_name.as_ref());
                fs::rename(&path, &new_path)?;
                renamed.push(format!("{} -> {}", fname, new_path.display()));
            }
        }
    }
    if renamed.is_empty() {
        Ok(Some("No files matched the pattern.".to_string()))
    } else {
        Ok(Some(format!("Renamed files:\n{}", renamed.join("\n"))))
    }
}

fn resize_image(file: &str, width: u32, height: u32) -> Result<Option<String>, Box<dyn Error>> {
    use image::imageops::FilterType;
    let img = image::open(file)?;
    let resized = img.resize(width, height, FilterType::Lanczos3);
    let out = format!("{}_resized.png", file);
    resized.save(&out)?;
    Ok(Some(format!("Resized image saved to {}", out)))
}

fn convert_image(file: &str, format: &str) -> Result<Option<String>, Box<dyn Error>> {
    let img = image::open(file)?;
    let out = match format.to_lowercase().as_str() {
        "png" => format!("{}.png", file),
        "jpg" | "jpeg" => format!("{}.jpg", file),
        "bmp" => format!("{}.bmp", file),
        "gif" => format!("{}.gif", file),
        _ => return Ok(Some("Unsupported format. Supported: png, jpg, bmp, gif.".to_string())),
    };
    img.save(&out)?;
    Ok(Some(format!("Converted image saved to {}", out)))
}

fn play_audio(file: &str) -> Result<Option<String>, Box<dyn Error>> {
    use std::process::Command;
    #[cfg(target_os = "macos")]
    let status = Command::new("afplay").arg(file).status()?;
    #[cfg(target_os = "linux")]
    let status = Command::new("aplay").arg(file).status()?;
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let status = Command::new("echo").arg("Audio playback not supported on this OS").status()?;
    if status.success() {
        Ok(Some("Audio played successfully.".to_string()))
    } else {
        Ok(Some("Failed to play audio.".to_string()))
    }
}

fn convert_audio(file: &str, format: &str) -> Result<Option<String>, Box<dyn Error>> {
    use std::process::Command;
    let out = format!("{}.{}", file, format);
    let status = Command::new("ffmpeg")
        .args(["-y", "-i", file, &out])
        .status()?;
    if status.success() {
        Ok(Some(format!("Converted audio saved to {}", out)))
    } else {
        Ok(Some("Failed to convert audio. Ensure ffmpeg is installed.".to_string()))
    }
}

fn speak_text(text: &str) -> Result<Option<String>, Box<dyn Error>> {
    use std::process::Command;
    #[cfg(target_os = "macos")]
    let status = Command::new("say").arg(text).status()?;
    #[cfg(target_os = "linux")]
    let status = Command::new("espeak").arg(text).status()?;
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let status = Command::new("echo").arg("Text-to-speech not supported on this OS").status()?;
    if status.success() {
        Ok(Some("Spoken successfully.".to_string()))
    } else {
        Ok(Some("Failed to speak text.".to_string()))
    }
}

fn copy_to_clipboard(text: &str) -> Result<Option<String>, Box<dyn Error>> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(text.to_string())?;
    Ok(Some("Copied to clipboard.".to_string()))
}

fn paste_from_clipboard() -> Result<Option<String>, Box<dyn Error>> {
    let mut clipboard = arboard::Clipboard::new()?;
    let text = clipboard.get_text()?;
    Ok(Some(format!("Clipboard: {}", text)))
}

fn watch_file_or_dir(target: &str) -> Result<Option<String>, Box<dyn Error>> {
    use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event};
    use std::sync::mpsc::channel;
    use std::time::Duration;
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
    watcher.watch(target, RecursiveMode::Recursive)?;
    println!("Watching {} for changes. Press Ctrl+C to stop.", target);
    for res in rx {
        match res {
            Ok(Event { kind, .. }) => println!("Change detected: {:?}", kind),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
    Ok(Some("Stopped watching.".to_string()))
}

fn schedule_command(cmd: &str, at_time: &str) -> Result<Option<String>, Box<dyn Error>> {
    use chrono::{Local, NaiveTime};
    use std::{thread, time::Duration};
    let now = Local::now().time();
    let target = NaiveTime::parse_from_str(at_time, "%H:%M")?;
    let now_secs = now.second() as i64 + now.minute() as i64 * 60 + now.hour() as i64 * 3600;
    let target_secs = target.second() as i64 + target.minute() as i64 * 60 + target.hour() as i64 * 3600;
    let secs = (target_secs - now_secs).rem_euclid(24*3600);
    println!("Scheduling command '{}' to run in {} seconds (at {}).", cmd, secs, at_time);
    thread::sleep(Duration::from_secs(secs as u64));
    let result = run_shell_command(cmd)?;
    Ok(Some(format!("Scheduled command output:\n{}", result.unwrap_or_default())))
} 