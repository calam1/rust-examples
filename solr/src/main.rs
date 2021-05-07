use stellr::{DirectSolrClient};
use stellr::prelude::*;
use serde::{Deserialize};

#[tokio::main]
async fn main() {
    // above macro does the same as below commented out code and adding async to main
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // let future = stuff();
    // let _a = rt.block_on(future);

    let result = stuff().await;
    println!("result from stuff in main {:?}", result);
}

async fn stuff() -> Result<Root, SolrError> {
    let solr_client = DirectSolrClient::new("https://q2lsolap501.gcom.grainger.com:8983/solr")?;

    let pwd_option: Option<String> = Some(String::from("73xAQPnTRu7"));

    let solr_request = solr_client
        .select("orders")?
        .rows(10)
        .q("accountNumber:0824467609")
        .fq("sapContactId:0707801039")
        .fl("*")
        .wt("json")
        .basic_auth("hybris-us", pwd_option);

    // let result_struct = solr_request.unstructured_call().await?;
    let result_struct = solr_request.call::<Root>().await?;
    println!("{:?}", result_struct);

    Ok(result_struct)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub response_header: ResponseHeader,
    pub response: Response,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseHeader {
    pub status: i64,
    #[serde(rename = "QTime")]
    pub qtime: i64,
    pub params: Params,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub q: String,
    pub fl: String,
    pub fq: String,
    pub rows: String,
    pub wt: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub num_found: i64,
    pub start: i64,
    pub max_score: f64,
    pub docs: Vec<Doc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Doc {
    pub id: String,
    pub sap_order_number: String,
    pub gcom_order_number: String,
    pub ordered_by: String,
    pub sap_contact_id: String,
    pub account_number: String,
    pub sales_office: String,
    pub purchase_order_number: String,
    pub requisitioner_name: Option<String>,
    pub sap_system_timestamp: String,
    pub order_channel: String,
    pub delivery_method: String,
    pub attention: Option<String>,
    pub ship_company_name1: String,
    pub ship_address1: String,
    pub ship_city: String,
    pub ship_region: String,
    pub ship_postal_code: String,
    pub ship_country: String,
    pub contact_phone_number: String,
    pub contact_fax_number: String,
    pub contact_email_address: String,
    pub currency: String,
    pub payment_method: String,
    pub customer_username: String,
    pub purchase_order_type: String,
    pub deleted_flag: String,
    pub sap_order_line_number: String,
    pub item_number: String,
    pub quantity: i64,
    pub purchase_order_line_number: String,
    pub item_short_desc: String,
    pub brand_name: String,
    pub mfg_part_num: Option<String>,
    pub unspsc: String,
    pub order_header_flag: String,
    pub higher_level_line_item: String,
    pub orig_sls_ordline_itm: String,
    #[serde(rename = "_version_")]
    pub version: i64,
    pub doc_typ: String,
    pub order_created_date_time: String,
    pub extended_item_price: f64,
    pub unit_price: f64,
    pub order_subtotal: f64,
    pub order_tax: f64,
    pub order_freight: f64,
    pub order_total: f64,
    pub fuel_surcharge_freight: f64,
    pub quote_exp_dt: String,
    pub item_tax: f64,
    pub item_freight: f64,
    pub last_modified: String,
    pub freight_terms: Option<String>,
    pub line_freight_terms: Option<String>,
    #[serde(rename = "zDPPItemCond")]
    pub z_dppitem_cond: Option<String>,
    pub s_org: Option<String>,
    pub gsa_schedule: Option<String>,
    pub gsa_sched_desc: Option<String>,
}
