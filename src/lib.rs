use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

pub type Record = HashMap<String, String>;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataFrameCountry {
    #[serialize_always]
    SNA_price_valuation: Option<String>,
    #[serialize_always]
    IMF_data_dissemination_standard: Option<String>,
    #[serialize_always]
    Latest_industrial_data: Option<String>,
    #[serialize_always]
    System_of_National_Accounts: Option<String>,
    #[serialize_always]
    Latest_household_survey: Option<String>,
    #[serialize_always]
    Special_Notes: Option<String>,
    #[serialize_always]
    Alternative_conversion_factor: Option<String>,
    #[serialize_always]
    Government_Accounting_concept: Option<String>,
    #[serialize_always]
    National_accounts_base_year: Option<String>,
    #[serialize_always]
    Balance_of_Payments_Manual_in_use: Option<String>,
    #[serialize_always]
    PPP_survey_year: Option<String>,
    #[serialize_always]
    Other_groups: Option<String>,
    #[serialize_always]
    Lending_category: Option<String>,
    #[serialize_always]
    Source_of_most_recent_Income_and_expenditure_data: Option<String>,
    #[serialize_always]
    Latest_trade_data: Option<String>,
    #[serialize_always]
    External_debt_Reporting_status: Option<String>,
    #[serialize_always]
    Latest_population_census: Option<String>,
    #[serialize_always]
    Country_Code: Option<String>,
    #[serialize_always]
    WB_2_code: Option<String>,
    #[serialize_always]
    Income_Group: Option<String>,
    #[serialize_always]
    National_accounts_reference_year: Option<String>,
    #[serialize_always]
    System_of_trade: Option<String>,
    #[serialize_always]
    Region: Option<String>,
    #[serialize_always]
    Table_Name: Option<String>,
    #[serialize_always]
    Long_Name: Option<String>,
    #[serialize_always]
    Latest_agricultural_census: Option<String>,
    #[serialize_always]
    Vital_registration_complete: Option<String>,
    #[serialize_always]
    pub Short_Name: Option<String>,
    #[serialize_always]
    Currency_Unit: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataFrame {
    #[serialize_always]
    pub individual: Option<f64>,
    #[serialize_always]
    natlty3_txt: Option<String>,
    #[serialize_always]
    ransom: Option<f64>,
    #[serialize_always]
    related: Option<String>,
    #[serialize_always]
    gsubname: Option<String>,
    #[serialize_always]
    claim2: Option<String>,
    #[serialize_always]
    nreleased: Option<String>,
    #[serialize_always]
    weaptype3: Option<String>,
    #[serialize_always]
    weapsubtype3: Option<String>,
    #[serialize_always]
    natlty2: Option<String>,
    #[serialize_always]
    propextent: Option<String>,
    #[serialize_always]
    nhostkid: Option<String>,
    #[serialize_always]
    INT_ANY: Option<f64>,
    #[serialize_always]
    eventid: Option<f64>,
    #[serialize_always]
    nwoundte: Option<String>,
    #[serialize_always]
    pub country_txt: Option<String>,
    #[serialize_always]
    alternative_txt: Option<String>,
    #[serialize_always]
    gsubname2: Option<String>,
    #[serialize_always]
    ndays: Option<String>,
    #[serialize_always]
    multiple: Option<f64>,
    #[serialize_always]
    gname3: Option<String>,
    #[serialize_always]
    attacktype1_txt: Option<String>,
    #[serialize_always]
    guncertain3: Option<String>,
    #[serialize_always]
    claimed: Option<String>,
    #[serialize_always]
    propextent_txt: Option<String>,
    #[serialize_always]
    targsubtype1_txt: Option<String>,
    #[serialize_always]
    extended: Option<f64>,
    #[serialize_always]
    suicide: Option<f64>,
    #[serialize_always]
    attacktype2_txt: Option<String>,
    #[serialize_always]
    gsubname3: Option<String>,
    #[serialize_always]
    guncertain1: Option<f64>,
    #[serialize_always]
    weaptype2: Option<String>,
    #[serialize_always]
    nwound: Option<f64>,
    #[serialize_always]
    targsubtype1: Option<f64>,
    #[serialize_always]
    attacktype3: Option<String>,
    #[serialize_always]
    ransompaid: Option<String>,
    #[serialize_always]
    success: Option<f64>,
    #[serialize_always]
    specificity: Option<f64>,
    #[serialize_always]
    doubtterr: Option<f64>,
    #[serialize_always]
    weaptype2_txt: Option<String>,
    #[serialize_always]
    nkillter: Option<String>,
    #[serialize_always]
    scite1: Option<String>,
    #[serialize_always]
    INT_LOG: Option<f64>,
    #[serialize_always]
    INT_IDEO: Option<f64>,
    #[serialize_always]
    latitude: Option<f64>,
    #[serialize_always]
    iyear: Option<f64>,
    #[serialize_always]
    imonth: Option<f64>,
    #[serialize_always]
    weapdetail: Option<String>,
    #[serialize_always]
    weaptype4: Option<String>,
    #[serialize_always]
    targsubtype2: Option<String>,
    #[serialize_always]
    targtype3_txt: Option<String>,
    #[serialize_always]
    claimmode: Option<String>,
    #[serialize_always]
    weaptype1_txt: Option<String>,
    #[serialize_always]
    alternative: Option<String>,
    #[serialize_always]
    claimmode2: Option<String>,
    #[serialize_always]
    target1: Option<String>,
    #[serialize_always]
    weaptype1: Option<f64>,
    #[serialize_always]
    nkillus: Option<String>,
    #[serialize_always]
    scite3: Option<String>,
    #[serialize_always]
    INT_MISC: Option<f64>,
    #[serialize_always]
    kidhijcountry: Option<String>,
    #[serialize_always]
    natlty2_txt: Option<String>,
    #[serialize_always]
    ransomnote: Option<String>,
    #[serialize_always]
    weaptype4_txt: Option<String>,
    #[serialize_always]
    hostkidoutcome_txt: Option<String>,
    #[serialize_always]
    guncertain2: Option<String>,
    #[serialize_always]
    longitude: Option<f64>,
    #[serialize_always]
    target2: Option<String>,
    #[serialize_always]
    pub nkill: Option<f64>,
    #[serialize_always]
    nwoundus: Option<String>,
    #[serialize_always]
    nhostkidus: Option<String>,
    #[serialize_always]
    attacktype2: Option<String>,
    #[serialize_always]
    ransomamtus: Option<String>,
    #[serialize_always]
    attacktype1: Option<f64>,
    #[serialize_always]
    weapsubtype3_txt: Option<String>,
    #[serialize_always]
    ransompaidus: Option<String>,
    #[serialize_always]
    crit3: Option<f64>,
    #[serialize_always]
    nperpcap: Option<String>,
    #[serialize_always]
    claimmode_txt: Option<String>,
    #[serialize_always]
    weaptype3_txt: Option<String>,
    #[serialize_always]
    region: Option<f64>,
    #[serialize_always]
    nhours: Option<String>,
    #[serialize_always]
    provstate: Option<String>,
    #[serialize_always]
    propcomment: Option<String>,
    #[serialize_always]
    weapsubtype1_txt: Option<String>,
    #[serialize_always]
    targsubtype2_txt: Option<String>,
    #[serialize_always]
    natlty1: Option<f64>,
    #[serialize_always]
    country: Option<f64>,
    #[serialize_always]
    corp3: Option<String>,
    #[serialize_always]
    target3: Option<String>,
    #[serialize_always]
    targtype2: Option<String>,
    #[serialize_always]
    approxdate: Option<String>,
    #[serialize_always]
    motive: Option<String>,
    #[serialize_always]
    nperps: Option<String>,
    #[serialize_always]
    region_txt: Option<String>,
    #[serialize_always]
    claimmode3: Option<String>,
    #[serialize_always]
    targtype1_txt: Option<String>,
    #[serialize_always]
    gname: Option<String>,
    #[serialize_always]
    resolution: Option<String>,
    #[serialize_always]
    corp2: Option<String>,
    #[serialize_always]
    scite2: Option<String>,
    #[serialize_always]
    divert: Option<String>,
    #[serialize_always]
    iday: Option<f64>,
    #[serialize_always]
    ishostkid: Option<f64>,
    #[serialize_always]
    city: Option<String>,
    #[serialize_always]
    vicinity: Option<f64>,
    #[serialize_always]
    corp1: Option<String>,
    #[serialize_always]
    natlty3: Option<String>,
    #[serialize_always]
    weapsubtype4: Option<String>,
    #[serialize_always]
    dbsource: Option<String>,
    #[serialize_always]
    property: Option<f64>,
    #[serialize_always]
    ransomamt: Option<String>,
    #[serialize_always]
    claimmode3_txt: Option<String>,
    #[serialize_always]
    propvalue: Option<String>,
    #[serialize_always]
    targtype3: Option<String>,
    #[serialize_always]
    claim3: Option<String>,
    #[serialize_always]
    targtype2_txt: Option<String>,
    #[serialize_always]
    crit1: Option<f64>,
    #[serialize_always]
    targsubtype3: Option<String>,
    #[serialize_always]
    weapsubtype2_txt: Option<String>,
    #[serialize_always]
    claimmode2_txt: Option<String>,
    #[serialize_always]
    weapsubtype4_txt: Option<String>,
    #[serialize_always]
    targtype1: Option<f64>,
    #[serialize_always]
    natlty1_txt: Option<String>,
    #[serialize_always]
    hostkidoutcome: Option<String>,
    #[serialize_always]
    addnotes: Option<String>,
    #[serialize_always]
    compclaim: Option<String>,
    #[serialize_always]
    targsubtype3_txt: Option<String>,
    #[serialize_always]
    crit2: Option<f64>,
    #[serialize_always]
    location: Option<String>,
    #[serialize_always]
    summary: Option<String>,
    #[serialize_always]
    weapsubtype1: Option<String>,
    #[serialize_always]
    weapsubtype2: Option<String>,
    #[serialize_always]
    gname2: Option<String>,
    #[serialize_always]
    attacktype3_txt: Option<String>,
    #[serialize_always]
    pub computed: Option<f64>,
    #[serialize_always]
    country_ext: Option<String>,
}

impl DataFrame {
    pub fn add_country_ext(&mut self, country: Option<DataFrameCountry>) {
        self.country_ext = Some(format!("{:?}", country))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupBy {
    pub country: String,
    pub total_nkill: f64,
    pub average_individual: f64,
    pub count: f64,
}
