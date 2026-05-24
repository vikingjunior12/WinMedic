use crate::services::powershell;

fn validate_username(username: &str) -> Result<(), String> {
    if username.is_empty() || username.len() > 20 {
        return Err("Username must be 1–20 characters long".to_string());
    }
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        return Err(
            "Username may only contain letters, digits, _, -, and .".to_string(),
        );
    }
    Ok(())
}

#[tauri::command]
pub async fn check_user_exists(username: String) -> Result<bool, String> {
    validate_username(&username)?;
    let esc = powershell::escape(&username);
    let ps = format!(
        "$u = Get-LocalUser -Name '{esc}' -ErrorAction SilentlyContinue; if ($null -ne $u) {{ Write-Output 'EXISTS' }} else {{ Write-Output 'MISSING' }}"
    );
    let result = powershell::run(&ps).await?;
    Ok(result.contains("EXISTS"))
}

#[tauri::command]
pub async fn create_local_user(
    username: String,
    password: String,
    add_to_admins: bool,
) -> Result<String, String> {
    validate_username(&username)?;
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }

    let esc_user = powershell::escape(&username);
    let esc_pw = powershell::escape(&password);

    let mut ps = format!(
        "$ss = New-Object System.Security.SecureString; ('{esc_pw}').ToCharArray() | ForEach-Object {{ $ss.AppendChar($_) }}; New-LocalUser -Name '{esc_user}' -Password $ss -PasswordNeverExpires:$true -AccountNeverExpires:$true"
    );

    if add_to_admins {
        ps.push_str(&format!(
            "; Add-LocalGroupMember -Group 'Administrators' -Member '{esc_user}'"
        ));
    }

    powershell::run(&ps).await?;
    Ok(format!("User '{}' created successfully", username))
}
