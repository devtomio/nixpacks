use serde::{Deserialize, Serialize};

use super::{
    images::{DEBIAN_SLIM_IMAGE, DEFAULT_BASE_IMAGE},
    nix::pkg::Pkg,
    plan::new_build_plan::{NewPhase, NewStartPhase},
};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetupPhase {
    pub pkgs: Vec<Pkg>,
    pub archive: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub apt_pkgs: Option<Vec<String>>,
    pub cmds: Option<Vec<String>>,

    #[serde(rename = "onlyIncludeFiles")]
    pub only_include_files: Option<Vec<String>>,

    #[serde(rename = "baseImage")]
    pub base_image: String,
}

impl From<SetupPhase> for NewPhase {
    fn from(setup_phase: SetupPhase) -> Self {
        NewPhase {
            name: "setup".to_string(),
            nix_pkgs: Some(setup_phase.pkgs),
            nix_libraries: setup_phase.libraries,
            nixpacks_archive: setup_phase.archive,
            apt_pkgs: setup_phase.apt_pkgs,
            cmds: setup_phase.cmds,
            only_include_files: setup_phase.only_include_files,
            ..Default::default()
        }
    }
}

impl SetupPhase {
    pub fn new(pkgs: Vec<Pkg>) -> Self {
        Self {
            pkgs,
            libraries: None,
            apt_pkgs: None,
            archive: None,
            only_include_files: None,
            base_image: DEFAULT_BASE_IMAGE.to_string(),
            cmds: None,
        }
    }

    pub fn add_file_dependency(&mut self, file: String) {
        if let Some(mut files) = self.only_include_files.clone() {
            files.push(file);
            self.only_include_files = Some(files);
        } else {
            self.only_include_files = Some(vec![file]);
        }
    }

    pub fn add_pkgs(&mut self, new_pkgs: &mut Vec<Pkg>) {
        self.pkgs.append(new_pkgs);
    }

    pub fn set_archive(&mut self, archive: String) {
        self.archive = Some(archive);
    }

    pub fn add_libraries(&mut self, lib: Vec<String>) {
        if let Some(libraries) = self.libraries.clone() {
            self.libraries = Some([libraries, lib].concat());
        } else {
            self.libraries = Some(lib);
        }
    }

    pub fn add_apt_pkgs(&mut self, apt_pkgs: Vec<String>) {
        if let Some(apt_packages) = self.apt_pkgs.clone() {
            self.apt_pkgs = Some([apt_packages, apt_pkgs].concat());
        } else {
            self.apt_pkgs = Some(apt_pkgs);
        }
    }

    pub fn add_cmd(&mut self, cmd: String) {
        if let Some(mut cmds) = self.cmds.clone() {
            cmds.push(cmd);
            self.cmds = Some(cmds);
        } else {
            self.cmds = Some(vec![cmd]);
        }
    }
}

impl Default for SetupPhase {
    fn default() -> Self {
        Self {
            pkgs: Default::default(),
            libraries: Default::default(),
            apt_pkgs: None,
            archive: Default::default(),
            only_include_files: Default::default(),
            base_image: DEFAULT_BASE_IMAGE.to_string(),
            cmds: Default::default(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct InstallPhase {
    pub cmds: Option<Vec<String>>,

    #[serde(rename = "onlyIncludeFiles")]
    pub only_include_files: Option<Vec<String>>,

    #[serde(rename = "cacheDirectories")]
    pub cache_directories: Option<Vec<String>>,

    pub paths: Option<Vec<String>>,
}

impl From<InstallPhase> for NewPhase {
    fn from(install_phase: InstallPhase) -> Self {
        NewPhase {
            name: "install".to_string(),
            cmds: install_phase.cmds,
            only_include_files: install_phase.only_include_files,
            cache_directories: install_phase.cache_directories,
            ..Default::default()
        }
    }
}

impl InstallPhase {
    pub fn new(cmd: String) -> Self {
        Self {
            cmds: Some(vec![cmd]),
            only_include_files: None,
            cache_directories: None,
            paths: None,
        }
    }

    pub fn add_file_dependency(&mut self, file: String) {
        if let Some(mut files) = self.only_include_files.clone() {
            files.push(file);
            self.only_include_files = Some(files);
        } else {
            self.only_include_files = Some(vec![file]);
        }
    }

    pub fn add_cache_directory(&mut self, dir: String) {
        if let Some(mut dirs) = self.cache_directories.clone() {
            dirs.push(dir);
            self.cache_directories = Some(dirs);
        } else {
            self.cache_directories = Some(vec![dir]);
        }
    }

    pub fn add_path(&mut self, path: String) {
        if let Some(mut paths) = self.paths.clone() {
            paths.push(path);
            self.paths = Some(paths);
        } else {
            self.paths = Some(vec![path]);
        }
    }

    pub fn add_cmd(&mut self, cmd: String) {
        if let Some(mut cmds) = self.cmds.clone() {
            cmds.push(cmd);
            self.cmds = Some(cmds);
        } else {
            self.cmds = Some(vec![cmd]);
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct BuildPhase {
    pub cmds: Option<Vec<String>>,

    #[serde(rename = "onlyIncludeFiles")]
    pub only_include_files: Option<Vec<String>>,

    #[serde(rename = "cacheDirectories")]
    pub cache_directories: Option<Vec<String>>,
}

impl From<BuildPhase> for NewPhase {
    fn from(build_phase: BuildPhase) -> Self {
        NewPhase {
            name: "build".to_string(),
            cmds: build_phase.cmds,
            only_include_files: build_phase.only_include_files,
            cache_directories: build_phase.cache_directories,
            ..Default::default()
        }
    }
}

impl BuildPhase {
    pub fn new(cmd: String) -> Self {
        Self {
            cmds: Some(vec![cmd]),
            only_include_files: None,
            cache_directories: None,
        }
    }

    pub fn add_file_dependency(&mut self, file: String) {
        if let Some(mut files) = self.only_include_files.clone() {
            files.push(file);
            self.only_include_files = Some(files);
        } else {
            self.only_include_files = Some(vec![file]);
        }
    }

    pub fn add_cache_directory(&mut self, dir: String) {
        if let Some(mut dirs) = self.cache_directories.clone() {
            dirs.push(dir);
            self.cache_directories = Some(dirs);
        } else {
            self.cache_directories = Some(vec![dir]);
        }
    }

    pub fn add_cmd(&mut self, cmd: String) {
        if let Some(mut cmds) = self.cmds.clone() {
            cmds.push(cmd);
            self.cmds = Some(cmds);
        } else {
            self.cmds = Some(vec![cmd]);
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct StartPhase {
    pub cmd: Option<String>,

    #[serde(rename = "runImage")]
    pub run_image: Option<String>,

    #[serde(rename = "onlyIncludeFiles")]
    pub only_include_files: Option<Vec<String>>,
}

impl From<StartPhase> for NewStartPhase {
    fn from(start_phase: StartPhase) -> Self {
        NewStartPhase {
            run_image: start_phase.run_image,
            cmd: start_phase.cmd,
            ..Default::default()
        }
    }
}

impl StartPhase {
    pub fn new(cmd: String) -> Self {
        Self {
            cmd: Some(cmd),
            run_image: None,
            only_include_files: None,
        }
    }

    pub fn run_in_image(&mut self, image_name: String) {
        self.run_image = Some(image_name);
    }

    pub fn run_in_default_image(&mut self) {
        self.run_image = Some(DEFAULT_BASE_IMAGE.to_string());
    }

    pub fn run_in_slim_image(&mut self) {
        self.run_image = Some(DEBIAN_SLIM_IMAGE.to_string());
    }

    pub fn add_file_dependency(&mut self, file: String) {
        if let Some(mut files) = self.only_include_files.clone() {
            files.push(file);
            self.only_include_files = Some(files);
        } else {
            self.only_include_files = Some(vec![file]);
        }
    }
}
