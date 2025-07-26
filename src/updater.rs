use log::info;
use self_update;
use std::error::Error;

pub struct Updater {
    repo_owner: String,
    repo_name: String,
    bin_name: String,
}

impl Updater {
    pub fn new(repo_owner: &str, repo_name: &str, bin_name: &str) -> Self {
        Self {
            repo_owner: repo_owner.to_string(),
            repo_name: repo_name.to_string(),
            bin_name: bin_name.to_string(),
        }
    }

    /// 获取当前版本号
    fn get_current_version(&self) -> String {
        const GIT_TAG: &str = env!("GIT_TAG");
        GIT_TAG.trim_start_matches('v').to_string()
    }

    /// 检查是否有新版本可用
    pub fn check_update(&self) -> Result<bool, Box<dyn Error>> {
        let current_version = self.get_current_version();
        info!("当前版本: {}", current_version);

        let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner(&self.repo_owner)
            .repo_name(&self.repo_name)
            .build()?
            .fetch()?;

        if let Some(latest_release) = releases.first() {
            let latest_version = latest_release.version.trim_start_matches('v');
            info!("最新版本: {}", latest_version);

            if latest_version != current_version {
                println!("发现新版本: {} -> {}", current_version, latest_version);
                return Ok(true);
            } else {
                println!("当前已是最新版本: {}", current_version);
                return Ok(false);
            }
        }

        println!("无法获取版本信息");
        Ok(false)
    }

    /// 执行自动更新
    pub fn update(&self) -> Result<(), Box<dyn Error>> {
        let current_version = self.get_current_version();
        info!("开始更新，当前版本: {}", current_version);

        let status = self_update::backends::github::Update::configure()
            .repo_owner(&self.repo_owner)
            .repo_name(&self.repo_name)
            .bin_name(&self.bin_name)
            .show_download_progress(true)
            .current_version(&current_version)
            .build()?
            .update()?;

        match status {
            self_update::Status::UpToDate(_) => {
                println!("当前已是最新版本！");
            }
            self_update::Status::Updated(version) => {
                println!("更新成功！新版本: {}", version);
                println!("请重新启动程序以使用新版本");
            }
        }

        Ok(())
    }

    /// 检查并提示更新（非强制）
    pub fn check_and_prompt_update(&self) -> Result<(), Box<dyn Error>> {
        if self.check_update()? {
            println!("运行 'gitig update' 进行更新");
        }
        Ok(())
    }
}

/// 创建默认的更新器实例
pub fn create_updater() -> Updater {
    // 请根据你的 GitHub 仓库信息修改这些值
    Updater::new("cfanbo", "gitig", "gitig")
}
