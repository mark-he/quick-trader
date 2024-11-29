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