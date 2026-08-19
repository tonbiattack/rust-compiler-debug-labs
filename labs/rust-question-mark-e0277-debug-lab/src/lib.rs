/// 文字列で与えられたポート番号を解析します。
pub fn parse_port(input: &str) -> Result<u16, String> {
    let port: u16 = input
        .parse()
        .map_err(|error| format!("ポート番号を解析できません: {error}"))?;

    Ok(port)
}
