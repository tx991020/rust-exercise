use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shellfn::shell;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "FCODE")]
    pub fcode: String,
    #[serde(rename = "SHORTNAME")]
    pub shortname: String,
    #[serde(rename = "FUNDTYPE")]
    pub fundtype: String,
    #[serde(rename = "BFUNDTYPE")]
    pub bfundtype: String,
    #[serde(rename = "FEATURE")]
    pub feature: String,
    #[serde(rename = "FSRQ")]
    pub fsrq: String,
    #[serde(rename = "RZDF")]
    pub rzdf: String,
    #[serde(rename = "DWJZ")]
    pub dwjz: String,
    #[serde(rename = "HLDWJZ")]
    pub hldwjz: String,
    #[serde(rename = "LJJZ")]
    pub ljjz: String,
    #[serde(rename = "FTYI")]
    pub ftyi: String,
    #[serde(rename = "TEYI")]
    pub teyi: String,
    #[serde(rename = "TFYI")]
    pub tfyi: String,
    #[serde(rename = "SYL_Z")]
    pub syl_z: String,
    #[serde(rename = "SYL_Y")]
    pub syl_y: String,
    #[serde(rename = "SYL_3Y")]
    pub syl3_y: String,
    #[serde(rename = "SYL_6Y")]
    pub syl6_y: String,
    #[serde(rename = "SYL_1N")]
    pub syl1_n: String,
    #[serde(rename = "SYL_2N")]
    pub syl2_n: String,
    #[serde(rename = "SYL_3N")]
    pub syl3_n: String,
    #[serde(rename = "SYL_5N")]
    pub syl5_n: String,
    #[serde(rename = "SYL_JN")]
    pub syl_jn: String,
    #[serde(rename = "SYL_LN")]
    pub syl_ln: String,
    #[serde(rename = "ZJL")]
    pub zjl: String,
    #[serde(rename = "TARGETYIELD")]
    pub targetyield: String,
    #[serde(rename = "CYCLE")]
    pub cycle: String,
    #[serde(rename = "KFR")]
    pub kfr: String,
    #[serde(rename = "LEVERAGE")]
    pub leverage: String,
    #[serde(rename = "BAGTYPE")]
    pub bagtype: String,
    #[serde(rename = "BUY")]
    pub buy: bool,
    #[serde(rename = "LISTTEXCH")]
    pub listtexch: String,
    #[serde(rename = "NEWTEXCH")]
    pub newtexch: String,
    #[serde(rename = "ISLISTTRADE")]
    pub islisttrade: String,
    #[serde(rename = "PTDT_Y")]
    pub ptdt_y: String,
    #[serde(rename = "PTDT_TWY")]
    pub ptdt_twy: String,
    #[serde(rename = "PTDT_TRY")]
    pub ptdt_try: String,
    #[serde(rename = "PTDT_FY")]
    pub ptdt_fy: String,
    #[serde(rename = "MBDT_Y")]
    pub mbdt_y: String,
    #[serde(rename = "MBDT_TWY")]
    pub mbdt_twy: String,
    #[serde(rename = "MBDT_TRY")]
    pub mbdt_try: String,
    #[serde(rename = "MBDT_FY")]
    pub mbdt_fy: String,
    #[serde(rename = "YDDT_Y")]
    pub yddt_y: String,
    #[serde(rename = "YDDT_TWY")]
    pub yddt_twy: String,
    #[serde(rename = "YDDT_TRY")]
    pub yddt_try: String,
    #[serde(rename = "YDDT_FY")]
    pub yddt_fy: String,
    #[serde(rename = "DWDT_Y")]
    pub dwdt_y: String,
    #[serde(rename = "DWDT_TWY")]
    pub dwdt_twy: String,
    #[serde(rename = "DWDT_TRY")]
    pub dwdt_try: String,
    #[serde(rename = "DWDT_FY")]
    pub dwdt_fy: String,
    #[serde(rename = "ENDNAV")]
    pub endnav: String,
    #[serde(rename = "SALEVOLUME")]
    pub salevolume: String,
    #[serde(rename = "PV_Y")]
    pub pv_y: String,
    #[serde(rename = "DTCOUNT_Y")]
    pub dtcount_y: String,
    #[serde(rename = "ORGSALESRANK")]
    pub orgsalesrank: String,
    #[serde(rename = "ISABNORMAL")]
    pub isabnormal: String,
}
// #[derive(Serialize, Deserialize, Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    Datas: Vec<Person>,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "type", rename_all = "camelCase")]
// enum Data {
//     Datas { person: Vec<Person> },
// }

#[shell]
fn list_modified(dir: &str) -> Result<impl Iterator<Item = String>, Error> {
    r#"
netstat -n | awk '/^tcp/ {++S[$NF]} END {for(a in S) print a, S[a]}'
"#
}

#[shell(cmd = "python -c")]
fn pretty_json(json: &str, indent: u8, sort_keys: bool) -> Result<String, Error> {
    r#"
import os, json

input = os.environ['JSON']
indent = int(os.environ['INDENT'])
sort_keys = os.environ['SORT_KEYS'] == 'true'
obj = json.loads(input)

print(json.dumps(obj, indent=indent, sort_keys=sort_keys))
"#
}

#[shell]
fn curl(dir: &str) -> Result<String, Error> {
    r#"
curl -H 'gtoken: ceaf-99b78df4fab899ced2cc7e10ce64bccb' -H 'clientInfo: ttjj-JSN-AL00a-Android-10' -H 'Host: fundmobapi.eastmoney.com' -H 'User-Agent: okhttp/3.12.0' --data "appType=ttjj&Sort=desc&product=EFund&gToken=ceaf-99b78df4fab899ced2cc7e10ce64bccb&version=6.4.0&DataConstraintType=0&onFundCache=3&ctoken=ca-ueu1cjdeue6-dredkjf18uuqaed18.9&ESTABDATE=&deviceid=884a04b668a18d4fb77dec2b17efe2e4%7C%7C346719440025268&ENDNAV=&FundType=0&BUY=true&pageIndex=1&RLEVEL_SZ=&RISKLEVEL=&DISCOUNT=&utoken=rhd8rqkr6cq1r--6h8eqe6eed1edha8eccfac8b8.9&CompanyId=&SortColumn=SYL_Z&pageSize=30&MobileKey=884a04b668a18d4fb77dec2b17efe2e4%7C%7C346719440025268&TOPICAL=&plat=Android&ISABNORMAL=true" --compressed 'https://fundmobapi.eastmoney.com/FundMNewApi/FundMNRank'
"#
}

fn main() -> Result<()> {
    // let result = curl("/Users/andy/CLionProjects/todo/sh")?;
    // //
    // let data: Data = serde_json::from_str(&result)?;
    // for i in data.Datas {
    //     dbg!(i);
    // }

    use std::mem;

    let mut v: Vec<i32> = vec![1, 2];

    let old_v = mem::take(&mut v);
    assert_eq!(vec![1, 2], old_v);
    assert!(v.is_empty());
    dbg!(v);

    Ok(())
}
