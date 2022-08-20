/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetMarketsRegionIdHistory200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMarketsRegionIdHistory200Ok {
  /// average number
  #[serde(rename = "average")]
  average: f64,
  /// The date of this historical statistic entry
  #[serde(rename = "date")]
  date: String,
  /// highest number
  #[serde(rename = "highest")]
  highest: f64,
  /// lowest number
  #[serde(rename = "lowest")]
  lowest: f64,
  /// Total number of orders happened that day
  #[serde(rename = "order_count")]
  order_count: i64,
  /// Total
  #[serde(rename = "volume")]
  volume: i64
}

impl GetMarketsRegionIdHistory200Ok {
  /// 200 ok object
  pub fn new(average: f64, date: String, highest: f64, lowest: f64, order_count: i64, volume: i64) -> GetMarketsRegionIdHistory200Ok {
    GetMarketsRegionIdHistory200Ok {
      average: average,
      date: date,
      highest: highest,
      lowest: lowest,
      order_count: order_count,
      volume: volume
    }
  }

  pub fn set_average(&mut self, average: f64) {
    self.average = average;
  }

  pub fn with_average(mut self, average: f64) -> GetMarketsRegionIdHistory200Ok {
    self.average = average;
    self
  }

  pub fn average(&self) -> &f64 {
    &self.average
  }


  pub fn set_date(&mut self, date: String) {
    self.date = date;
  }

  pub fn with_date(mut self, date: String) -> GetMarketsRegionIdHistory200Ok {
    self.date = date;
    self
  }

  pub fn date(&self) -> &String {
    &self.date
  }


  pub fn set_highest(&mut self, highest: f64) {
    self.highest = highest;
  }

  pub fn with_highest(mut self, highest: f64) -> GetMarketsRegionIdHistory200Ok {
    self.highest = highest;
    self
  }

  pub fn highest(&self) -> &f64 {
    &self.highest
  }


  pub fn set_lowest(&mut self, lowest: f64) {
    self.lowest = lowest;
  }

  pub fn with_lowest(mut self, lowest: f64) -> GetMarketsRegionIdHistory200Ok {
    self.lowest = lowest;
    self
  }

  pub fn lowest(&self) -> &f64 {
    &self.lowest
  }


  pub fn set_order_count(&mut self, order_count: i64) {
    self.order_count = order_count;
  }

  pub fn with_order_count(mut self, order_count: i64) -> GetMarketsRegionIdHistory200Ok {
    self.order_count = order_count;
    self
  }

  pub fn order_count(&self) -> &i64 {
    &self.order_count
  }


  pub fn set_volume(&mut self, volume: i64) {
    self.volume = volume;
  }

  pub fn with_volume(mut self, volume: i64) -> GetMarketsRegionIdHistory200Ok {
    self.volume = volume;
    self
  }

  pub fn volume(&self) -> &i64 {
    &self.volume
  }


}



