#![forbid(unsafe_code)]

use {
    csv::ReaderBuilder,
    serde::Deserialize,
    std::{
        collections::HashMap,
        env,
        error::Error,
        ffi::OsStr,
        fs,
        path::Path,
        process,
        thread::{JoinHandle, spawn},
    },
    xlsxwriter::Workbook,
};

fn main() {
    if once() {
        let args: Vec<String> = std::env::args().collect();
        if args.len() == 1 {
            Settings {
                icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
                ..Default::default()
            }
            .sandbox::<Siege>();
        } else {
            let param = Param::build(args).unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {err}");
                std::process::exit(1);
            });
            if let Err(e) = route(param) {
                eprintln!("Application error: {e}");
                std::process::exit(1);
            }
        }
    }
}

pub(crate) const NAME: &str = "FlHudson";

struct Param {
    case: String,
    path: String,
}

impl Param {
    pub fn build(args: Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        Ok(Self {
            case: args[1].clone(),
            path: args[2].clone(),
        })
    }
    fn route(&self) -> Result<(), Box<dyn Error>> {
        save_xlsx(
            &(env::var("HOME").unwrap() + "/report.xlsx"),
            &match self.case.as_str() {
                "csv" => open_csv(find_csv(&param.path)),
                "virsh" => Virsh::build(&param.path).run(),
                &_ => HashMap::new(),
            },
        );
        Ok(())
    }
}

fn find_csv(path: &str) -> Vec<String> {
    if let Ok(entries) = fs::read_dir(path) {
        let files: Vec<String> = entries
            .map(|entry| entry.ok().unwrap().path())
            .filter(|path| path.extension() == Some(OsStr::new("csv")))
            .map(|path| format!("{}", path.display()))
            .collect::<Vec<String>>();
        files
    } else {
        panic!("\x1b[31mERROR\x1b[0m")
    }
}

fn open_csv(source: Vec<String>) -> HashMap<String, Vec<Vec<String>>> {
    let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let mut children: HashMap<String, JoinHandle<Vec<Vec<String>>>> = HashMap::new();
    for file in source {
        let label = Path::new(&file).file_stem().unwrap().to_str().unwrap();
        children.insert(
            label.to_string(),
            spawn(move || -> Vec<Vec<String>> {
                ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(fs::read_to_string(file).unwrap().as_bytes())
                    .records()
                    .map(|row| {
                        row.unwrap()
                            .iter()
                            .map(str::to_string)
                            .collect::<Vec<String>>()
                    })
                    .collect()
            }),
        );
    }
    for (label, thread) in children {
        data.insert(label, thread.join().unwrap());
    }
    data
}

fn once() -> bool {
    if cfg!(target_os = "linux") {
        let run = process::Command::new("lsof")
            .args(["-t", &env::current_exe().unwrap().display().to_string()])
            .output()
            .expect("failed to execute bash");
        if run.status.success() {
            String::from_utf8_lossy(&run.stdout)
                .split_whitespace()
                .count()
                == 1
        } else {
            panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr))
        }
    } else {
        true
    }
}

trait Action {
    fn build(path: &str) -> Self {
        serde_json::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap()
    }
    fn run(&self) -> HashMap<String, Vec<Vec<String>>>;
}

#[derive(Deserialize, Default)]
struct Virsh {
    pub nodes: Vec<usize>,
}

impl Action for Virsh {
    fn run(&self) -> HashMap<String, Vec<Vec<String>>> {
        let mut children: HashMap<String, JoinHandle<Vec<Vec<String>>>> = HashMap::new();
        for node in self.nodes.clone() {
            children.insert(
                node.to_string(),
                spawn(move || -> Vec<Vec<String>> { vm_list(node) }),
            );
        }
        let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        for (node, thread) in children {
            data.insert(node, thread.join().unwrap());
        }
        data
    }
}

fn vm_list(node: usize) -> Vec<Vec<String>> {
    let run = process::Command::new("bash")
        .args(["-xc", &format!("sshpass -e virsh --connect qemu+ssh://{}@192.168.25.{node}/system list --name --state-running", env::var("SSHUSER").unwrap())])
        .output()
        .expect("failed to execute bash");
    let mut list_raw: Vec<Vec<String>> = Vec::new();
    if run.status.success() {
        eprintln!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
        for pool in String::from_utf8_lossy(&run.stdout)
            .split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(5)
        {
            let mut children: Vec<JoinHandle<Vec<String>>> = Vec::new();
            for line in pool {
                let vm: String = line.to_string();
                children.push(spawn(move || -> Vec<String> { vm_dump(node, vm) }));
            }
            for thread in children {
                list_raw.push(thread.join().unwrap())
            }
        }
    } else {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr))
    };
    list_raw
}

#[derive(Deserialize)]
struct VM {
    name: String,
    memory: String,
    vcpu: String,
}

fn vm_dump(node: usize, name: String) -> Vec<String> {
    let run = process::Command::new("bash")
        .args([
            "-xc",
            &format!(
                "sshpass -e virsh --connect qemu+ssh://admin@192.168.25.{}/system dumpxml {}",
                node, name
            ),
        ])
        .output()
        .expect("failed to execute bash");
    let vm: VM = if run.status.success() {
        eprintln!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
        serde_xml_rs::from_str(String::from_utf8_lossy(&run.stdout).to_string().as_str()).unwrap()
    } else {
        panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr))
    };
    vec![vm.name, vm.memory, vm.vcpu]
}

fn shell(command: &str) {
    let run = std::process::Command::new("pwsh")
        .arg("-NoExit")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(format!("$ErrorActionPreference = 'stop'; Set-PSDebug -Strict; & {command} | Out-Host; Exit $LastExitCode"))
        .output()
        .unwrap_or_else(|e| panic!("\x1b[31m{command}: {e}\x1b[0m"));
    if !run.status.success() {
        panic!(
            "\x1b[31m{command}\nstdout:\n{}\nstderr:\n{}\x1b[0m",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr),
        );
    }
}
