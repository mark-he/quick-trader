use std::str::FromStr;
use strum::{Display, VariantNames,};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Copy, Clone, Display)]
#[derive(Serialize, VariantNames)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Spot,
    Linear,
    Inverse,
    Option,
}
impl FromStr for Category {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "spot" => Ok(Category::Spot),
            "linear" => Ok(Category::Linear),
            "inverse" => Ok(Category::Inverse),
            "option" => Ok(Category::Option),
            _ => Err("Invalid Category".to_string()),
        }
    }
}
impl<'de> Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Debug, Copy, Clone, Display)]
#[derive(Serialize)]
pub enum Side {
    Buy,
    Sell,
}
impl FromStr for Side {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Buy" => Ok(Side::Buy),
            "Sell" => Ok(Side::Sell),
            _ => Err("Invalid Side".to_string()),
        }
    }
}
impl<'de> Deserialize<'de> for Side {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Debug, Copy, Clone, Display)]
#[derive(Serialize)]
pub enum OrderType {
    Market,
    Limit,
}
impl FromStr for OrderType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Market" => Ok(OrderType::Market),
            "Limit" => Ok(OrderType::Limit),
            _ => Err("Invalid OrderType".to_string()),
        }
    }
}
impl<'de> Deserialize<'de> for OrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Debug, Copy, Clone, Display)]
#[derive(Serialize)]
pub enum TimeInForceType {
    IOC,
    GTC,
}
impl FromStr for TimeInForceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IOC" => Ok(TimeInForceType::IOC),
            "GTC" => Ok(TimeInForceType::GTC),
            _ => Err("Invalid TimeInForceType".to_string()),
        }
    }
}
impl<'de> Deserialize<'de> for TimeInForceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Debug, Copy, Clone, Display)]
#[derive(Serialize)]
pub enum TriggerBy {
    LastPrice,
    IndexPrice,
    MarkPrice,
}
impl FromStr for TriggerBy {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LastPrice" => Ok(TriggerBy::LastPrice),
            "IndexPrice" => Ok(TriggerBy::IndexPrice),
            "MarkPrice" => Ok(TriggerBy::MarkPrice),
            _ => Err("Invalid TriggerBy".to_string()),
        }
    }
}
impl<'de> Deserialize<'de> for TriggerBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}


#[derive(Copy, Clone, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum KlineInterval {
    #[strum(serialize = "1")]
    Minutes1,
    #[strum(serialize = "3")]
    Minutes3,
    #[strum(serialize = "5")]
    Minutes5,
    #[strum(serialize = "15")]
    Minutes15,
    #[strum(serialize = "30")]
    Minutes30,
    #[strum(serialize = "60")]
    Hours1,
    #[strum(serialize = "120")]
    Hours2,
    #[strum(serialize = "240")]
    Hours4,
    #[strum(serialize = "360")]
    Hours6,
    #[strum(serialize = "720")]
    Hours12,
    #[strum(serialize = "D")]
    Days1,
    #[strum(serialize = "W")]
    Weeks1,
    #[strum(serialize = "M")]
    Months1,
}

impl FromStr for KlineInterval {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(KlineInterval::Minutes1),
            "3" => Ok(KlineInterval::Minutes3),
            "5" => Ok(KlineInterval::Minutes5),
            "15" => Ok(KlineInterval::Minutes15),
            "30" => Ok(KlineInterval::Minutes30),
            "60" => Ok(KlineInterval::Hours1),
            "120" => Ok(KlineInterval::Hours2),
            "240" => Ok(KlineInterval::Hours4),
            "360" => Ok(KlineInterval::Hours6),
            "720" => Ok(KlineInterval::Hours12),
            "D" => Ok(KlineInterval::Days1),
            "W" => Ok(KlineInterval::Weeks1),
            "M" => Ok(KlineInterval::Months1),
            _ => Err("Invalid KlineInterval".to_string()),
        }
    }
}