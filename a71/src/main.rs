extern crate reqwest;

use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "gtoken",
        "ceaf-99b78df4fab899ced2cc7e10ce64bccb".parse().unwrap(),
    );
    headers.insert("clientInfo", "ttjj-JSN-AL00a-Android-10".parse().unwrap());
    headers.insert("Host", "fundmobapi.eastmoney.com".parse().unwrap());
    headers.insert("User-Agent", "okhttp/3.12.0".parse().unwrap());

    let res = reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap()
        .post("https://fundmobapi.eastmoney.com/FundMNewApi/FundMNRank")
        .headers(headers)
        .body("appType=ttjj&Sort=desc&product=EFund&gToken=ceaf-99b78df4fab899ced2cc7e10ce64bccb&version=6.4.0&DataConstraintType=0&onFundCache=3&ctoken=ca-ueu1cjdeue6-dredkjf18uuqaed18.9&ESTABDATE=&deviceid=884a04b668a18d4fb77dec2b17efe2e4%7C%7C346719440025268&ENDNAV=&FundType=0&BUY=true&pageIndex=1&RLEVEL_SZ=&RISKLEVEL=&DISCOUNT=&utoken=rhd8rqkr6cq1r--6h8eqe6eed1edha8eccfac8b8.9&CompanyId=&SortColumn=SYL_Z&pageSize=30&MobileKey=884a04b668a18d4fb77dec2b17efe2e4%7C%7C346719440025268&TOPICAL=&plat=Android&ISABNORMAL=true")
        .send().await?
        .text().await?;
    println!("{}", res);

    Ok(())
}
