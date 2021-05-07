use serde::{Deserialize, Serialize};

fn main() {
    let data = r#"{"responseHeader":{"status":0,"QTime":15,"params":{"q":"accountNumber:0824467609","fl":"*","fq":"sapContactId:0707801039","rows":"10","wt":"json"}},"response":{"numFound":2,"start":0,"maxScore":14.108806,"docs":[{"id":"0824467609!1351502654|000010","sapOrderNumber":"1351502654","gcomOrderNumber":"WEB1425626151","orderedBy":"BUZZ GIDLEY","sapContactId":"0707801039","accountNumber":"0824467609","salesOffice":"E01","purchaseOrderNumber":"WEB1425626151","requisitionerName":"BUZZ GIDLEY","sapSystemTimestamp":"20190514073003.0000000","orderChannel":"ONLINE","deliveryMethod":"SH","attention":"BUZZ GIDLEY","shipCompanyName1":"UNITED NATIONS CAPITAL DEVELOPMENT FUND","shipAddress1":"13540 WAINWRIGHT DR","shipCity":"PORT CHARLOTTE","shipRegion":"FL","shipPostalCode":"33953-3301","shipCountry":"US","contactPhoneNumber":"9414354972","contactFaxNumber":"0","contactEmailAddress":"BUZZ.GIDLEY@COMPUTERREPAIRS.NET","currency":"USD","paymentMethod":"OA","customerUsername":"PICCLNT000","purchaseOrderType":"12","deletedFlag":"","sapOrderLineNumber":"000010","itemNumber":"1TDT3","quantity":1,"purchaseOrderLineNumber":"000001","itemShortDesc":"BLOWER 559 CFM 230V 0.98A 1600 RPM","brandName":"DAYTON","mfgPartNum":"1TDT3","unspsc":"40101601","orderHeaderFlag":"Y","higherLevelLineItem":"000000","origSlsOrdlineItm":"000000","_version_":1641752826636926976,"docTyp":"ZOR","orderCreatedDateTime":"2019-05-14T12:30:03Z","extendedItemPrice":173.0,"unitPrice":173.0,"orderSubtotal":173.0,"orderTax":12.11,"orderFreight":15.33,"orderTotal":200.44,"fuelSurchargeFreight":0.0,"quoteExpDt":"0002-11-30T00:00:00Z","itemTax":12.11,"itemFreight":15.33,"lastModified":"2019-08-13T11:58:24.651Z"},{"id":"0824467609!2041147002|000010","sapOrderNumber":"2041147002","gcomOrderNumber":"QTE1461477017","orderedBy":"BUZZ GIDLEY","sapContactId":"0707801039","accountNumber":"0824467609","salesOffice":"E01","purchaseOrderNumber":"QTE1461477017","sapSystemTimestamp":"20200821160044.0000000","orderChannel":"ONLINE","deliveryMethod":"SH","shipCompanyName1":"KJLKJLKJLKJLKJ","shipAddress1":"500 N MICHIGAN AVE","shipCity":"CHICAGO","shipRegion":"IL","shipPostalCode":"60611-3777","shipCountry":"US","contactPhoneNumber":"8477028802","contactFaxNumber":"8475556001","contactEmailAddress":"JAMES.KING@GRAINGER.COM","currency":"USD","paymentMethod":"OA","customerUsername":"QICCLNT000","purchaseOrderType":"12","deletedFlag":"","freightTerms":"PPA","sapOrderLineNumber":"000010","itemNumber":"1FCC2","quantity":1,"purchaseOrderLineNumber":"000001","itemShortDesc":"TAN WOOD GLUE 16.00 OZ.","brandName":"TITEBOND","unspsc":"31201610","orderHeaderFlag":"Y","lineFreightTerms":"PPA","higherLevelLineItem":"000000","zDPPItemCond":"0.00","origSlsOrdlineItm":"000000","_version_":1675670213845057536,"docTyp":"ZQT","sOrg":"0300","gsaSchedule":"51V","gsaSchedDesc":"47QSHA18D000G","itemTax":0.33,"itemFreight":0.0,"extendedItemPrice":5.3,"orderCreatedDateTime":"2020-08-21T21:00:44Z","orderFreight":0.0,"orderSubtotal":5.3,"orderTax":0.33,"unitPrice":5.3,"fuelSurchargeFreight":0.0,"quoteExpDt":"2020-09-20T00:00:00Z","orderTotal":5.63,"lastModified":"2020-08-21T21:00:45.698Z"}]}}"#;

    // let p: Root = serde_json::from_str(dddd).unwrap();
    let p = serde_json::from_str::<Root>(data);


    println!("Please call {:?}", p);
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub response_header: ResponseHeader,
    pub response: Response,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseHeader {
    pub status: i64,
    #[serde(rename = "QTime")]
    pub qtime: i64,
    pub params: Params,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub q: String,
    pub fl: String,
    pub fq: String,
    pub rows: String,
    pub wt: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub num_found: i64,
    pub start: i64,
    pub max_score: f64,
    pub docs: Vec<Doc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
