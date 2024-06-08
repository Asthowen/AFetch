use crate::error::FetchInfosError;
#[cfg(target_os = "linux")]
use {crate::utils::get_file_content_without_lines, std::path::Path};
#[cfg(any(target_os = "linux", target_os = "windows", target_os = "freebsd"))]
use {crate::utils::return_str_from_command, std::process::Command};

pub async fn get_host() -> Result<Option<String>, FetchInfosError> {
    #[cfg(target_os = "linux")]
    {
        let mut host = String::default();
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
        let host: String = return_str_from_command(
            Command::new("wmic")
                .arg("computersystem")
                .arg("get")
                .arg("manufacturer,model"),
        )?
        .replace("Manufacturer  Model", "")
        .replace("     ", " ")
        .trim()
        .to_owned();

        Ok(Some(host))
    }

    #[cfg(target_os = "freebsd")]
    {
        let output: String = return_str_from_command(&mut Command::new("kenv"))?;

        let mut vendor: String = String::default();
        let mut product: String = String::default();

        for line in output.lines() {
            if line.contains("smbios.system.maker=") {
                vendor = line
                    .split('=')
                    .nth(1)
                    .ok_or(FetchInfosError::error("Failed to parse host vendor"))?
                    .trim_matches('"')
                    .clone_into(&mut vendor);
            } else if line.contains("smbios.system.product=") {
                product = line
                    .split('=')
                    .nth(1)
                    .ok_or(FetchInfosError::error("Failed to parse host product"))?
                    .trim_matches('"')
                    .clone_into(&mut product);
            }
        }

        if vendor.is_empty() || product.is_empty() {
            return Ok(None);
        }

        Ok(Some(format!("{} {}", vendor, product)))
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "freebsd")))]
    {
        // TODO - add other OS
        Ok(None)
    }
}
