use pixrs::IllustInfo;

#[test]
fn illust_info_deserialize() -> anyhow::Result<()> {
    let data: IllustInfo = serde_json::from_str(include_str!("json/illust_info_100412238.json"))?;
    println!("{data:#?}");
    Ok(())
}
