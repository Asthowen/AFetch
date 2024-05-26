use crate::error::FetchInfosError;
use crate::utils::{get_file_content_without_lines, return_str_from_command};
use std::path::Path;
use std::process::Command;

pub async fn get_host() -> Result<Option<String>, FetchInfosError> {
    let mut host = String::default();
    #[cfg(target_os = "linux")]
    {
        if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
            host = format!(
                "{}{}",
                return_str_from_command(Command::new("getprop").arg("ro.product.brand"))?,
                return_str_from_command(Command::new("getprop").arg("ro.product.model"))?
            );
        } else if Path::new("/sys/devices/virtual/dmi/id/product_name").exists()
            && Path::new("/sys/devices/virtual/dmi/id/product_version").exists()
        {
            host = format!(
                "{} {}",
                get_file_content_without_lines("/sys/devices/virtual/dmi/id/product_name").await?,
                get_file_content_without_lines("/sys/devices/virtual/dmi/id/product_version")
                    .await?
            );
        } else if Path::new("/sys/firmware/devicetree/base/model").exists() {
            host = get_file_content_without_lines("/sys/firmware/devicetree/base/model").await?;
        } else if Path::new("/tmp/sysinfo/model").exists() {
            host = get_file_content_without_lines("/tmp/sysinfo/model").await?;
        }

        if (host.contains("System Product Name") || host.is_empty())
            && Path::new("/sys/devices/virtual/dmi/id/board_vendor").exists()
            && Path::new("/sys/devices/virtual/dmi/id/board_name").exists()
        {
            host = format!(
                "{} {}",
                get_file_content_without_lines("/sys/devices/virtual/dmi/id/board_vendor").await?,
                get_file_content_without_lines("/sys/devices/virtual/dmi/id/board_name")
                    .await?
                    .as_str(),
            )
        }

        Ok(Some(host))
    }

    #[cfg(target_os = "windows")]
    {
        host = return_str_from_command(
            Command::new("wmic")
                .arg("computersystem")
                .arg("get")
                .arg("manufacturer,model"),
        )?
        .replace("Manufacturer  Model", "")
        .replace("     ", " ")
        .trim()
        .to_owned();
        host
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        // TODO - add other OS
        String::default()
    }
}
