

use std::fmt;
use serde::de;
use serde::de::{ Deserialize, Deserializer };
use crate::serde_parsers::{ deserialize_as_f64, deserialize_as_maybe_f64 };


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CurrencyPrice {
    pub symbol: CurrencyPair,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub price: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CurrencyBase {
    BNB,  // Binance coin
    BTC,  // Bitcoin
    ETH,  // Ethereum
    TUST, // TrueUSD
    USDT, // Tether
    PAX,  // Paxos
    USDC, // Circle USD
}
impl fmt::Display for CurrencyBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(format!("{:?}", self)))
    }
}


impl fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_lowercase())
    }
}

impl CurrencyPair {
    pub fn to_lowercase(&self) -> String {
        let str_pair: String = self.as_string().to_lowercase();
        str_pair
    }

    pub fn as_string(&self) -> String {
        String::from(format!("{:?}", &self))
    }

    pub fn filter_base_pair(&self, base_pair: CurrencyBase) -> bool {
        // regex matches pairs with 'CurrencyBase' as base pair.
        let base_re = regex::Regex::new(&format!(r#"{}$"#, base_pair)).unwrap();
        match base_re.find(&self.as_string()) {
            Some(s) => true,
            None => false,
        }
    }
}


/// Traits for PostgreSQL serialization for CurrencyPair enum
use diesel::sql_types::Text;
use diesel::pg::Pg;
use diesel::serialize::{ ToSql, Output };
use diesel::deserialize::{ FromSql };

impl ToSql<Text, Pg> for CurrencyPair {
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, Pg>) -> ::diesel::serialize::Result {
        let symbol = self.as_string();
        ToSql::<Text, Pg>::to_sql(&symbol, out)
    }
}

impl FromSql<Text, Pg> for CurrencyPair {
    fn from_sql(maybe_bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        match maybe_bytes {
            Some(bytes) => {
                let s = std::str::from_utf8(bytes).expect("Error converting bytes to str in FromSql for CurrencyPair");
                Ok(serde_json::from_str::<CurrencyPair>(s).unwrap())
            },
            None => panic!("No CurrencyPair read FromSql"),
        }
    }
}



#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[derive(EnumString)]   // strum converts Strings to Enum directly
#[derive(AsExpression)] // For Diesel PostgreSQL
#[sql_type = "Text"]    // Declare type as Text for PostgreSQL
pub enum CurrencyPair {
    ADABNB, ADABTC, ADAETH, ADATUSD, ADAUSDT,
    ADXBNB, ADXBTC, ADXETH,
    AEBNB, AEBTC, AEETH,
    AGIBNB, AGIBTC, AGIETH,
    AIONBNB, AIONBTC, AIONETH,
    AMBBNB, AMBBTC, AMBETH,
    APPCBNB, APPCBTC, APPCETH,
    ARDRBNB, ARDRBTC, ARDRETH,
    ARKBTC, ARKETH,
    ARNBTC, ARNETH,
    ASTBTC, ASTETH,
    BATBNB, BATBTC, BATETH,
    BCCBNB, BCCBTC, BCCETH, BCCUSDT,
    BCDBTC, BCDETH,
    BCHABCBTC, BCHABCPAX, BCHABCTUSD, BCHABCUSDC, BCHABCUSDT,
    BCHSVBTC, BCHSVPAX, BCHSVTUSD, BCHSVUSDC, BCHSVUSDT,
    BCNBNB, BCNBTC, BCNETH,
    BCPTBNB, BCPTBTC, BCPTETH,
    BLZBNB, BLZBTC, BLZETH,
    BNBBTC, BNBETH, BNBPAX, BNBUSDC, BNBUSDT,
    BNTBTC, BNTETH,
    BQXBTC, BQXETH,
    BRDBNB, BRDBTC, BRDETH,
    BTCPAX, BTCUSDC, BTCUSDT,
    BTGBTC, BTGETH, BTSBNB,
    BTSBTC, BTSETH,
    BTTBNB, BTTBTC, BTTUSDT,
    CDTBTC, CDTETH,
    CHATBTC, CHATETH,
    CLOAKBTC, CLOAKETH,
    CMTBNB, CMTBTC, CMTETH,
    CNDBNB, CNDBTC, CNDETH,
    CVCBNB, CVCBTC, CVCETH,
    DASHBTC, DASHETH,
    DATABTC, DATAETH,
    DCRBNB, DCRBTC,
    DENTBTC, DENTETH,
    DGDBTC, DGDETH,
    DLTBNB, DLTBTC, DLTETH,
    DNTBTC, DNTETH,
    DOCKBTC, DOCKETH,
    EDOBTC, EDOETH,
    ELFBTC, ELFETH,
    ENGBTC, ENGETH,
    ENJBNB, ENJBTC, ENJETH,
    EOSBNB, EOSBTC, EOSETH, EOSPAX, EOSTUSD, EOSUSDC, EOSUSDT,
    ETCBNB, ETCBTC, ETCETH, ETCUSDT,
    ETHBTC, ETHPAX, ETHUSDC, ETHUSDT,
    EVXBTC, EVXETH,
    FUELBTC, FUELETH,
    FUNBTC, FUNETH,
    GASBTC,
    GNTBNB, GNTBTC, GNTETH,
    GOBNB, GOBTC,
    GRSBTC, GRSETH,
    GTOBNB, GTOBTC, GTOETH,
    GVTBTC, GVTETH,
    GXSBTC, GXSETH,
    HCBTC, HCETH,
    HOTBTC, HOTETH,
    HSRBTC, HSRETH,
    ICNBTC, ICNETH,
    ICXBNB, ICXBTC, ICXETH, ICXUSDT,
    INSBTC, INSETH,
    IOSTBTC, IOSTETH,
    IOTABNB, IOTABTC, IOTAETH, IOTAUSDT,
    IOTXBTC, IOTXETH,
    KEYBTC, KEYETH,
    KMDBTC, KMDETH,
    KNCBTC, KNCETH,
    LENDBTC, LENDETH,
    LINKBTC, LINKETH, LINKPAX, LINKTUSD, LINKUSDC, LINKUSDT,
    LOOMBNB, LOOMBTC, LOOMETH,
    LRCBTC, LRCETH,
    LSKBNB, LSKBTC, LSKETH,
    LTCBNB, LTCBTC, LTCETH, LTCPAX, LTCTUSD, LTCUSDC, LTCUSDT,
    LUNBTC, LUNETH,
    MANABTC, MANAETH,
    MCOBNB, MCOBTC, MCOETH,
    MDABTC, MDAETH,
    MFTBNB, MFTBTC, MFTETH,
    MITHBNB, MITHBTC,
    MODBTC, MODETH,
    MTHBTC, MTHETH,
    MTLBTC, MTLETH,
    NANOBNB, NANOBTC, NANOETH,
    NASBNB, NASBTC, NASETH,
    NAVBNB, NAVBTC, NAVETH,
    NCASHBNB, NCASHBTC, NCASHETH,
    NEBLBNB, NEBLBTC, NEBLETH,
    NEOBNB, NEOBTC, NEOETH, NEOTUSD, NEOUSDT,
    NPXSBTC, NPXSETH,
    NULSBNB, NULSBTC, NULSETH, NULSUSDT,
    NXSBNB, NXSBTC, NXSETH,
    OAXBTC, OAXETH,
    OMGBTC, OMGETH,
    ONTBNB, ONTBTC, ONTETH, ONTUSDT,
    OSTBNB, OSTBTC, OSTETH,
    PAXBNB, PAXBTC, PAXETH, PAXTUSD, PAXUSDT,
    PHXBNB, PHXBTC, PHXETH,
    PIVXBNB, PIVXBTC, PIVXETH,
    POABNB, POABTC, POAETH,
    POEBTC, POEETH,
    POLYBNB, POLYBTC,
    POWRBNB, POWRBTC, POWRETH,
    PPTBTC, PPTETH,
    QKCBTC, QKCETH,
    QLCBNB, QLCBTC, QLCETH,
    QSPBNB, QSPBTC, QSPETH,
    QTUMBNB, QTUMBTC, QTUMETH, QTUMUSDT,
    RCNBNB, RCNBTC, RCNETH,
    RDNBNB, RDNBTC, RDNETH,
    RENBNB, RENBTC,
    REPBNB, REPBTC, REPETH,
    REQBTC, REQETH,
    RLCBNB, RLCBTC, RLCETH,
    RPXBNB, RPXBTC, RPXETH,
    RVNBNB, RVNBTC,
    SALTBTC, SALTETH,
    SCBNB, SCBTC, SCETH,
    SKYBNB, SKYBTC, SKYETH,
    SNGLSBTC, SNGLSETH,
    SNMBTC, SNMETH,
    SNTBTC, SNTETH,
    STEEMBNB, STEEMBTC, STEEMETH,
    STORJBTC, STORJETH,
    STORMBNB, STORMBTC, STORMETH,
    STRATBTC, STRATETH,
    SUBBTC, SUBETH,
    SYSBNB, SYSBTC, SYSETH,
    THETABNB, THETABTC, THETAETH,
    TNBBTC, TNBETH,
    TNTBTC, TNTETH,
    TRIGBNB, TRIGBTC, TRIGETH,
    TRXBNB, TRXBTC, TRXETH, TRXPAX, TRXTUSD, TRXUSDC, TRXUSDT, TRXXRP,
    TUSDBNB, TUSDBTC, TUSDETH, TUSDUSDT,
    USDCBNB, USDCBTC, USDCPAX, USDCTUSD, USDCUSDT,
    VENBNB, VENBTC, VENETH, VENUSDT,
    VETBNB, VETBTC, VETETH, VETUSDT,
    VIABNB, VIABTC, VIAETH,
    VIBBTC, VIBEBTC, VIBEETH, VIBETH,
    WABIBNB, WABIBTC, WABIETH,
    WANBNB, WANBTC, WANETH,
    WAVESBNB, WAVESBTC, WAVESETH, WAVESPAX, WAVESTUSD, WAVESUSDC, WAVESUSDT,
    WINGSBTC, WINGSETH,
    WPRBTC, WPRETH,
    WTCBNB, WTCBTC, WTCETH,
    XEMBNB, XEMBTC, XEMETH,
    XLMBNB, XLMBTC, XLMETH, XLMPAX, XLMTUSD, XLMUSDC, XLMUSDT,
    XMRBTC, XMRETH,
    XRPBNB, XRPBTC, XRPETH, XRPPAX, XRPTUSD, XRPUSDC, XRPUSDT,
    XVGBTC, XVGETH,
    XZCBNB, XZCBTC, XZCETH, XZCXRP,
    YOYOBNB, YOYOBTC, YOYOETH,
    ZECBTC, ZECETH,
    ZENBNB, ZENBTC, ZENETH,
    ZILBNB, ZILBTC, ZILETH,
    ZRXBTC, ZRXETH,
}

