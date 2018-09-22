use std::process::Command;
use std::fs::ReadDir;
use std::env;

fn find_wav_files(dir: ReadDir) -> Option<Vec<String>> {
    let wav_files = dir
        .map(|entry| entry.expect("IOエラーが発生しました"))
        .map(|file| file.path())
        .filter(|path| path.extension().is_some())
        .filter(|path| path.extension().unwrap().to_str().unwrap() == "wav")
        .map(|path| path.to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    if wav_files.is_empty() {
        return None
    }
    return Some(wav_files)
}

fn convert_wav_to_flac(wav_files: &Vec<String>) {
    println!("wavファイルの変換開始");
    let output_filenames = wav_files.iter()
        .map(|wav_file| wav_file.as_str().replace(".wav", ".flac"))
        .collect::<Vec<String>>();

    for (input, output) in wav_files.iter().zip(output_filenames) {
        print!("{} -> {}: ", input, output);
        let _output = Command::new("ffmpeg")
            .arg("-i")
            .arg(input)
            .arg(output)
            .output()
            .expect("ファイル変換中に問題が発生しました");
        println!(" Done！");
    }
    println!("ファイルの変換が完了しました");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let dir_path = args.get(0);

    let access_err_msg = "指定されたディレクトリにアクセスできませんでした";
    let read_dir: ReadDir;
    match dir_path {
        Some(p) => read_dir = std::fs::read_dir(p).expect(access_err_msg),
        None => read_dir = std::env::current_dir().expect(access_err_msg).read_dir().expect(access_err_msg)
    }

    let read_dir = find_wav_files(read_dir);
    match read_dir {
        Some(wav_files) => {
            convert_wav_to_flac(&wav_files)
        }
        None => println!("wavファイルが見つかりませんでした")
    }
    println!("処理を終了します");
}