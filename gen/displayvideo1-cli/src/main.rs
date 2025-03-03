// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_displayvideo1::{api, Error, oauth2, client::chrono, FieldMask};


use google_clis_common as client;

use client::{InvalidOptionsError, CLIError, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::error::Error as StdError;
use std::str::FromStr;

use serde_json as json;
use clap::ArgMatches;
use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n, S> {
    opt: ArgMatches<'n>,
    hub: api::DisplayVideo<S>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, S> Engine<'n, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn _advertisers_assets_upload(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "filename" => Some(("filename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["filename"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateAssetRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().assets_upload(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_audit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().audit(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-mask" => {
                    call = call.read_mask(        value.map(|v| arg_from_str(v, err, "read-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["read-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_bulk_edit_advertiser_assigned_targeting_options(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditAdvertiserAssignedTargetingOptionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().bulk_edit_advertiser_assigned_targeting_options(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_bulk_list_advertiser_assigned_targeting_options(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().bulk_list_advertiser_assigned_targeting_options(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_campaigns_bulk_list_campaign_assigned_targeting_options(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().campaigns_bulk_list_campaign_assigned_targeting_options(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("campaign-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_campaigns_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.end-date.day" => Some(("campaignFlight.plannedDates.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.end-date.month" => Some(("campaignFlight.plannedDates.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.end-date.year" => Some(("campaignFlight.plannedDates.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.start-date.day" => Some(("campaignFlight.plannedDates.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.start-date.month" => Some(("campaignFlight.plannedDates.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.start-date.year" => Some(("campaignFlight.plannedDates.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-spend-amount-micros" => Some(("campaignFlight.plannedSpendAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.campaign-goal-type" => Some(("campaignGoal.campaignGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.performance-goal.performance-goal-amount-micros" => Some(("campaignGoal.performanceGoal.performanceGoalAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.performance-goal.performance-goal-percentage-micros" => Some(("campaignGoal.performanceGoal.performanceGoalPercentageMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.performance-goal.performance-goal-string" => Some(("campaignGoal.performanceGoal.performanceGoalString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.performance-goal.performance-goal-type" => Some(("campaignGoal.performanceGoal.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-id" => Some(("campaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.max-impressions" => Some(("frequencyCap.maxImpressions", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit" => Some(("frequencyCap.timeUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit-count" => Some(("frequencyCap.timeUnitCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.unlimited" => Some(("frequencyCap.unlimited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "campaign-flight", "campaign-goal", "campaign-goal-type", "campaign-id", "day", "display-name", "end-date", "entity-status", "frequency-cap", "max-impressions", "month", "name", "performance-goal", "performance-goal-amount-micros", "performance-goal-percentage-micros", "performance-goal-string", "performance-goal-type", "planned-dates", "planned-spend-amount-micros", "start-date", "time-unit", "time-unit-count", "unlimited", "update-time", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Campaign = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().campaigns_create(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_campaigns_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().campaigns_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("campaign-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_campaigns_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().campaigns_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("campaign-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_campaigns_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().campaigns_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_campaigns_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.end-date.day" => Some(("campaignFlight.plannedDates.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.end-date.month" => Some(("campaignFlight.plannedDates.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.end-date.year" => Some(("campaignFlight.plannedDates.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.start-date.day" => Some(("campaignFlight.plannedDates.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.start-date.month" => Some(("campaignFlight.plannedDates.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-dates.start-date.year" => Some(("campaignFlight.plannedDates.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "campaign-flight.planned-spend-amount-micros" => Some(("campaignFlight.plannedSpendAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.campaign-goal-type" => Some(("campaignGoal.campaignGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.performance-goal.performance-goal-amount-micros" => Some(("campaignGoal.performanceGoal.performanceGoalAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.performance-goal.performance-goal-percentage-micros" => Some(("campaignGoal.performanceGoal.performanceGoalPercentageMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.performance-goal.performance-goal-string" => Some(("campaignGoal.performanceGoal.performanceGoalString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-goal.performance-goal.performance-goal-type" => Some(("campaignGoal.performanceGoal.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-id" => Some(("campaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.max-impressions" => Some(("frequencyCap.maxImpressions", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit" => Some(("frequencyCap.timeUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit-count" => Some(("frequencyCap.timeUnitCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.unlimited" => Some(("frequencyCap.unlimited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "campaign-flight", "campaign-goal", "campaign-goal-type", "campaign-id", "day", "display-name", "end-date", "entity-status", "frequency-cap", "max-impressions", "month", "name", "performance-goal", "performance-goal-amount-micros", "performance-goal-percentage-micros", "performance-goal-string", "performance-goal-type", "planned-dates", "planned-spend-amount-micros", "start-date", "time-unit", "time-unit-count", "unlimited", "update-time", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Campaign = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().campaigns_patch(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("campaign-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_campaigns_targeting_types_assigned_targeting_options_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().campaigns_targeting_types_assigned_targeting_options_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("campaign-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""), opt.value_of("assigned-targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_campaigns_targeting_types_assigned_targeting_options_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().campaigns_targeting_types_assigned_targeting_options_list(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("campaign-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-id" => Some(("channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negatively-targeted-line-item-count" => Some(("negativelyTargetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "positively-targeted-line-item-count" => Some(("positivelyTargetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "channel-id", "display-name", "name", "negatively-targeted-line-item-count", "partner-id", "positively-targeted-line-item-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().channels_create(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().channels_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().channels_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-id" => Some(("channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negatively-targeted-line-item-count" => Some(("negativelyTargetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "positively-targeted-line-item-count" => Some(("positivelyTargetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "channel-id", "display-name", "name", "negatively-targeted-line-item-count", "partner-id", "positively-targeted-line-item-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().channels_patch(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["partner-id", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_sites_bulk_edit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted-sites" => Some(("deletedSites", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "deleted-sites", "partner-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditSitesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().channels_sites_bulk_edit(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_sites_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-or-app-id" => Some(("urlOrAppId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["name", "url-or-app-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Site = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().channels_sites_create(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_sites_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().channels_sites_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""), opt.value_of("url-or-app-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_sites_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().channels_sites_list(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_channels_sites_replace(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "partner-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReplaceSitesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().channels_sites_replace(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "ad-server-config.cm-hybrid-config.cm-account-id" => Some(("adServerConfig.cmHybridConfig.cmAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ad-server-config.cm-hybrid-config.cm-floodlight-config-id" => Some(("adServerConfig.cmHybridConfig.cmFloodlightConfigId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ad-server-config.cm-hybrid-config.cm-floodlight-linking-authorized" => Some(("adServerConfig.cmHybridConfig.cmFloodlightLinkingAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ad-server-config.cm-hybrid-config.cm-syncable-site-ids" => Some(("adServerConfig.cmHybridConfig.cmSyncableSiteIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ad-server-config.cm-hybrid-config.dv360-to-cm-cost-reporting-enabled" => Some(("adServerConfig.cmHybridConfig.dv360ToCmCostReportingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ad-server-config.cm-hybrid-config.dv360-to-cm-data-sharing-enabled" => Some(("adServerConfig.cmHybridConfig.dv360ToCmDataSharingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ad-server-config.third-party-only-config.pixel-order-id-reporting-enabled" => Some(("adServerConfig.thirdPartyOnlyConfig.pixelOrderIdReportingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-config.dynamic-creative-enabled" => Some(("creativeConfig.dynamicCreativeEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creative-config.ias-client-id" => Some(("creativeConfig.iasClientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-config.oba-compliance-disabled" => Some(("creativeConfig.obaComplianceDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creative-config.video-creative-data-sharing-authorized" => Some(("creativeConfig.videoCreativeDataSharingAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "data-access-config.sdf-config.override-partner-sdf-config" => Some(("dataAccessConfig.sdfConfig.overridePartnerSdfConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "data-access-config.sdf-config.sdf-config.admin-email" => Some(("dataAccessConfig.sdfConfig.sdfConfig.adminEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-access-config.sdf-config.sdf-config.version" => Some(("dataAccessConfig.sdfConfig.sdfConfig.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "general-config.currency-code" => Some(("generalConfig.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "general-config.domain-url" => Some(("generalConfig.domainUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "general-config.time-zone" => Some(("generalConfig.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.details" => Some(("integrationDetails.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.integration-code" => Some(("integrationDetails.integrationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prisma-enabled" => Some(("prismaEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "serving-config.exempt-tv-from-viewability-targeting" => Some(("servingConfig.exemptTvFromViewabilityTargeting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-server-config", "admin-email", "advertiser-id", "cm-account-id", "cm-floodlight-config-id", "cm-floodlight-linking-authorized", "cm-hybrid-config", "cm-syncable-site-ids", "creative-config", "currency-code", "data-access-config", "details", "display-name", "domain-url", "dv360-to-cm-cost-reporting-enabled", "dv360-to-cm-data-sharing-enabled", "dynamic-creative-enabled", "entity-status", "exempt-tv-from-viewability-targeting", "general-config", "ias-client-id", "integration-code", "integration-details", "name", "oba-compliance-disabled", "override-partner-sdf-config", "partner-id", "pixel-order-id-reporting-enabled", "prisma-enabled", "sdf-config", "serving-config", "third-party-only-config", "time-zone", "update-time", "version", "video-creative-data-sharing-authorized"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Advertiser = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_creatives_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "appended-tag" => Some(("appendedTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cm-placement-id" => Some(("cmPlacementId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cm-tracking-ad.cm-ad-id" => Some(("cmTrackingAd.cmAdId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cm-tracking-ad.cm-creative-id" => Some(("cmTrackingAd.cmCreativeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cm-tracking-ad.cm-placement-id" => Some(("cmTrackingAd.cmPlacementId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "companion-creative-ids" => Some(("companionCreativeIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-attributes" => Some(("creativeAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-id" => Some(("creativeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-type" => Some(("creativeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimensions.height-pixels" => Some(("dimensions.heightPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "dimensions.width-pixels" => Some(("dimensions.widthPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic" => Some(("dynamic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expand-on-hover" => Some(("expandOnHover", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "expanding-direction" => Some(("expandingDirection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hosting-source" => Some(("hostingSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html5-video" => Some(("html5Video", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ias-campaign-monitoring" => Some(("iasCampaignMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "integration-code" => Some(("integrationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "js-tracker-url" => Some(("jsTrackerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "line-item-ids" => Some(("lineItemIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "media-duration" => Some(("mediaDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mp3-audio" => Some(("mp3Audio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.click-tracking-url" => Some(("obaIcon.clickTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.dimensions.height-pixels" => Some(("obaIcon.dimensions.heightPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "oba-icon.dimensions.width-pixels" => Some(("obaIcon.dimensions.widthPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "oba-icon.landing-page-url" => Some(("obaIcon.landingPageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.position" => Some(("obaIcon.position", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.program" => Some(("obaIcon.program", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.resource-mime-type" => Some(("obaIcon.resourceMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.resource-url" => Some(("obaIcon.resourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.view-tracking-url" => Some(("obaIcon.viewTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ogg-audio" => Some(("oggAudio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "progress-offset.percentage" => Some(("progressOffset.percentage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "progress-offset.seconds" => Some(("progressOffset.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "require-html5" => Some(("requireHtml5", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "require-mraid" => Some(("requireMraid", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "require-ping-for-attribution" => Some(("requirePingForAttribution", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "review-status.approval-status" => Some(("reviewStatus.approvalStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "review-status.content-and-policy-review-status" => Some(("reviewStatus.contentAndPolicyReviewStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "review-status.creative-and-landing-page-review-status" => Some(("reviewStatus.creativeAndLandingPageReviewStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skip-offset.percentage" => Some(("skipOffset.percentage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skip-offset.seconds" => Some(("skipOffset.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skippable" => Some(("skippable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-tag" => Some(("thirdPartyTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tracker-urls" => Some(("trackerUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "universal-ad-id.id" => Some(("universalAdId.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "universal-ad-id.registry" => Some(("universalAdId.registry", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vast-tag-url" => Some(("vastTagUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vpaid" => Some(("vpaid", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "appended-tag", "approval-status", "click-tracking-url", "cm-ad-id", "cm-creative-id", "cm-placement-id", "cm-tracking-ad", "companion-creative-ids", "content-and-policy-review-status", "create-time", "creative-and-landing-page-review-status", "creative-attributes", "creative-id", "creative-type", "dimensions", "display-name", "dynamic", "entity-status", "expand-on-hover", "expanding-direction", "height-pixels", "hosting-source", "html5-video", "ias-campaign-monitoring", "id", "integration-code", "js-tracker-url", "landing-page-url", "line-item-ids", "media-duration", "mp3-audio", "name", "notes", "oba-icon", "ogg-audio", "percentage", "position", "program", "progress-offset", "registry", "require-html5", "require-mraid", "require-ping-for-attribution", "resource-mime-type", "resource-url", "review-status", "seconds", "skip-offset", "skippable", "third-party-tag", "tracker-urls", "universal-ad-id", "update-time", "vast-tag-url", "view-tracking-url", "vpaid", "width-pixels"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Creative = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().creatives_create(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_creatives_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().creatives_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("creative-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_creatives_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().creatives_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("creative-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_creatives_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().creatives_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_creatives_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "appended-tag" => Some(("appendedTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cm-placement-id" => Some(("cmPlacementId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cm-tracking-ad.cm-ad-id" => Some(("cmTrackingAd.cmAdId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cm-tracking-ad.cm-creative-id" => Some(("cmTrackingAd.cmCreativeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cm-tracking-ad.cm-placement-id" => Some(("cmTrackingAd.cmPlacementId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "companion-creative-ids" => Some(("companionCreativeIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-attributes" => Some(("creativeAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-id" => Some(("creativeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-type" => Some(("creativeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimensions.height-pixels" => Some(("dimensions.heightPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "dimensions.width-pixels" => Some(("dimensions.widthPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dynamic" => Some(("dynamic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expand-on-hover" => Some(("expandOnHover", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "expanding-direction" => Some(("expandingDirection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hosting-source" => Some(("hostingSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html5-video" => Some(("html5Video", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ias-campaign-monitoring" => Some(("iasCampaignMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "integration-code" => Some(("integrationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "js-tracker-url" => Some(("jsTrackerUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "line-item-ids" => Some(("lineItemIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "media-duration" => Some(("mediaDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mp3-audio" => Some(("mp3Audio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.click-tracking-url" => Some(("obaIcon.clickTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.dimensions.height-pixels" => Some(("obaIcon.dimensions.heightPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "oba-icon.dimensions.width-pixels" => Some(("obaIcon.dimensions.widthPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "oba-icon.landing-page-url" => Some(("obaIcon.landingPageUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.position" => Some(("obaIcon.position", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.program" => Some(("obaIcon.program", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.resource-mime-type" => Some(("obaIcon.resourceMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.resource-url" => Some(("obaIcon.resourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "oba-icon.view-tracking-url" => Some(("obaIcon.viewTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ogg-audio" => Some(("oggAudio", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "progress-offset.percentage" => Some(("progressOffset.percentage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "progress-offset.seconds" => Some(("progressOffset.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "require-html5" => Some(("requireHtml5", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "require-mraid" => Some(("requireMraid", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "require-ping-for-attribution" => Some(("requirePingForAttribution", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "review-status.approval-status" => Some(("reviewStatus.approvalStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "review-status.content-and-policy-review-status" => Some(("reviewStatus.contentAndPolicyReviewStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "review-status.creative-and-landing-page-review-status" => Some(("reviewStatus.creativeAndLandingPageReviewStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skip-offset.percentage" => Some(("skipOffset.percentage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skip-offset.seconds" => Some(("skipOffset.seconds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skippable" => Some(("skippable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-tag" => Some(("thirdPartyTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tracker-urls" => Some(("trackerUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "universal-ad-id.id" => Some(("universalAdId.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "universal-ad-id.registry" => Some(("universalAdId.registry", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vast-tag-url" => Some(("vastTagUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vpaid" => Some(("vpaid", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "appended-tag", "approval-status", "click-tracking-url", "cm-ad-id", "cm-creative-id", "cm-placement-id", "cm-tracking-ad", "companion-creative-ids", "content-and-policy-review-status", "create-time", "creative-and-landing-page-review-status", "creative-attributes", "creative-id", "creative-type", "dimensions", "display-name", "dynamic", "entity-status", "expand-on-hover", "expanding-direction", "height-pixels", "hosting-source", "html5-video", "ias-campaign-monitoring", "id", "integration-code", "js-tracker-url", "landing-page-url", "line-item-ids", "media-duration", "mp3-audio", "name", "notes", "oba-icon", "ogg-audio", "percentage", "position", "program", "progress-offset", "registry", "require-html5", "require-mraid", "require-ping-for-attribution", "resource-mime-type", "resource-url", "review-status", "seconds", "skip-offset", "skippable", "third-party-tag", "tracker-urls", "universal-ad-id", "update-time", "vast-tag-url", "view-tracking-url", "vpaid", "width-pixels"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Creative = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().creatives_patch(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("creative-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().delete(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().get(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_insertion_orders_bulk_list_insertion_order_assigned_targeting_options(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().insertion_orders_bulk_list_insertion_order_assigned_targeting_options(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("insertion-order-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_insertion_orders_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.fixed-bid.bid-amount-micros" => Some(("bidStrategy.fixedBid.bidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.custom-bidding-algorithm-id" => Some(("bidStrategy.maximizeSpendAutoBid.customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.max-average-cpm-bid-amount-micros" => Some(("bidStrategy.maximizeSpendAutoBid.maxAverageCpmBidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.performance-goal-type" => Some(("bidStrategy.maximizeSpendAutoBid.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.raise-bid-for-deals" => Some(("bidStrategy.maximizeSpendAutoBid.raiseBidForDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.custom-bidding-algorithm-id" => Some(("bidStrategy.performanceGoalAutoBid.customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.max-average-cpm-bid-amount-micros" => Some(("bidStrategy.performanceGoalAutoBid.maxAverageCpmBidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.performance-goal-amount-micros" => Some(("bidStrategy.performanceGoalAutoBid.performanceGoalAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.performance-goal-type" => Some(("bidStrategy.performanceGoalAutoBid.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billable-outcome" => Some(("billableOutcome", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.automation-type" => Some(("budget.automationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.budget-unit" => Some(("budget.budgetUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-id" => Some(("campaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.max-impressions" => Some(("frequencyCap.maxImpressions", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit" => Some(("frequencyCap.timeUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit-count" => Some(("frequencyCap.timeUnitCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.unlimited" => Some(("frequencyCap.unlimited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "insertion-order-id" => Some(("insertionOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "insertion-order-type" => Some(("insertionOrderType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.details" => Some(("integrationDetails.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.integration-code" => Some(("integrationDetails.integrationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.daily-max-impressions" => Some(("pacing.dailyMaxImpressions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.daily-max-micros" => Some(("pacing.dailyMaxMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.pacing-period" => Some(("pacing.pacingPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.pacing-type" => Some(("pacing.pacingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "performance-goal.performance-goal-amount-micros" => Some(("performanceGoal.performanceGoalAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "performance-goal.performance-goal-percentage-micros" => Some(("performanceGoal.performanceGoalPercentageMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "performance-goal.performance-goal-string" => Some(("performanceGoal.performanceGoalString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "performance-goal.performance-goal-type" => Some(("performanceGoal.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reservation-type" => Some(("reservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "automation-type", "bid-amount-micros", "bid-strategy", "billable-outcome", "budget", "budget-unit", "campaign-id", "custom-bidding-algorithm-id", "daily-max-impressions", "daily-max-micros", "details", "display-name", "entity-status", "fixed-bid", "frequency-cap", "insertion-order-id", "insertion-order-type", "integration-code", "integration-details", "max-average-cpm-bid-amount-micros", "max-impressions", "maximize-spend-auto-bid", "name", "pacing", "pacing-period", "pacing-type", "performance-goal", "performance-goal-amount-micros", "performance-goal-auto-bid", "performance-goal-percentage-micros", "performance-goal-string", "performance-goal-type", "raise-bid-for-deals", "reservation-type", "time-unit", "time-unit-count", "unlimited", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InsertionOrder = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().insertion_orders_create(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_insertion_orders_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().insertion_orders_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("insertion-order-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_insertion_orders_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().insertion_orders_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("insertion-order-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_insertion_orders_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().insertion_orders_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_insertion_orders_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.fixed-bid.bid-amount-micros" => Some(("bidStrategy.fixedBid.bidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.custom-bidding-algorithm-id" => Some(("bidStrategy.maximizeSpendAutoBid.customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.max-average-cpm-bid-amount-micros" => Some(("bidStrategy.maximizeSpendAutoBid.maxAverageCpmBidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.performance-goal-type" => Some(("bidStrategy.maximizeSpendAutoBid.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.raise-bid-for-deals" => Some(("bidStrategy.maximizeSpendAutoBid.raiseBidForDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.custom-bidding-algorithm-id" => Some(("bidStrategy.performanceGoalAutoBid.customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.max-average-cpm-bid-amount-micros" => Some(("bidStrategy.performanceGoalAutoBid.maxAverageCpmBidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.performance-goal-amount-micros" => Some(("bidStrategy.performanceGoalAutoBid.performanceGoalAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.performance-goal-type" => Some(("bidStrategy.performanceGoalAutoBid.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billable-outcome" => Some(("billableOutcome", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.automation-type" => Some(("budget.automationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.budget-unit" => Some(("budget.budgetUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-id" => Some(("campaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.max-impressions" => Some(("frequencyCap.maxImpressions", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit" => Some(("frequencyCap.timeUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit-count" => Some(("frequencyCap.timeUnitCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.unlimited" => Some(("frequencyCap.unlimited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "insertion-order-id" => Some(("insertionOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "insertion-order-type" => Some(("insertionOrderType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.details" => Some(("integrationDetails.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.integration-code" => Some(("integrationDetails.integrationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.daily-max-impressions" => Some(("pacing.dailyMaxImpressions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.daily-max-micros" => Some(("pacing.dailyMaxMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.pacing-period" => Some(("pacing.pacingPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.pacing-type" => Some(("pacing.pacingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "performance-goal.performance-goal-amount-micros" => Some(("performanceGoal.performanceGoalAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "performance-goal.performance-goal-percentage-micros" => Some(("performanceGoal.performanceGoalPercentageMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "performance-goal.performance-goal-string" => Some(("performanceGoal.performanceGoalString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "performance-goal.performance-goal-type" => Some(("performanceGoal.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reservation-type" => Some(("reservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "automation-type", "bid-amount-micros", "bid-strategy", "billable-outcome", "budget", "budget-unit", "campaign-id", "custom-bidding-algorithm-id", "daily-max-impressions", "daily-max-micros", "details", "display-name", "entity-status", "fixed-bid", "frequency-cap", "insertion-order-id", "insertion-order-type", "integration-code", "integration-details", "max-average-cpm-bid-amount-micros", "max-impressions", "maximize-spend-auto-bid", "name", "pacing", "pacing-period", "pacing-type", "performance-goal", "performance-goal-amount-micros", "performance-goal-auto-bid", "performance-goal-percentage-micros", "performance-goal-string", "performance-goal-type", "raise-bid-for-deals", "reservation-type", "time-unit", "time-unit-count", "unlimited", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InsertionOrder = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().insertion_orders_patch(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("insertion-order-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_insertion_orders_targeting_types_assigned_targeting_options_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().insertion_orders_targeting_types_assigned_targeting_options_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("insertion-order-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""), opt.value_of("assigned-targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_insertion_orders_targeting_types_assigned_targeting_options_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().insertion_orders_targeting_types_assigned_targeting_options_list(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("insertion-order-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_invoices_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().invoices_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "loi-sapin-invoice-type" => {
                    call = call.loi_sapin_invoice_type(value.unwrap_or(""));
                },
                "issue-month" => {
                    call = call.issue_month(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["issue-month", "loi-sapin-invoice-type", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_invoices_lookup_invoice_currency(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().invoices_lookup_invoice_currency(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "invoice-month" => {
                    call = call.invoice_month(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["invoice-month"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_bulk_edit_line_item_assigned_targeting_options(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditLineItemAssignedTargetingOptionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().line_items_bulk_edit_line_item_assigned_targeting_options(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_bulk_list_line_item_assigned_targeting_options(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().line_items_bulk_list_line_item_assigned_targeting_options(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.fixed-bid.bid-amount-micros" => Some(("bidStrategy.fixedBid.bidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.custom-bidding-algorithm-id" => Some(("bidStrategy.maximizeSpendAutoBid.customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.max-average-cpm-bid-amount-micros" => Some(("bidStrategy.maximizeSpendAutoBid.maxAverageCpmBidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.performance-goal-type" => Some(("bidStrategy.maximizeSpendAutoBid.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.raise-bid-for-deals" => Some(("bidStrategy.maximizeSpendAutoBid.raiseBidForDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.custom-bidding-algorithm-id" => Some(("bidStrategy.performanceGoalAutoBid.customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.max-average-cpm-bid-amount-micros" => Some(("bidStrategy.performanceGoalAutoBid.maxAverageCpmBidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.performance-goal-amount-micros" => Some(("bidStrategy.performanceGoalAutoBid.performanceGoalAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.performance-goal-type" => Some(("bidStrategy.performanceGoalAutoBid.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.budget-allocation-type" => Some(("budget.budgetAllocationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.budget-unit" => Some(("budget.budgetUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.max-amount" => Some(("budget.maxAmount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-id" => Some(("campaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversion-counting.post-view-count-percentage-millis" => Some(("conversionCounting.postViewCountPercentageMillis", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creative-ids" => Some(("creativeIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-new-exchanges" => Some(("excludeNewExchanges", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "flight.date-range.end-date.day" => Some(("flight.dateRange.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.end-date.month" => Some(("flight.dateRange.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.end-date.year" => Some(("flight.dateRange.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.start-date.day" => Some(("flight.dateRange.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.start-date.month" => Some(("flight.dateRange.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.start-date.year" => Some(("flight.dateRange.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.flight-date-type" => Some(("flight.flightDateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "flight.trigger-id" => Some(("flight.triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.max-impressions" => Some(("frequencyCap.maxImpressions", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit" => Some(("frequencyCap.timeUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit-count" => Some(("frequencyCap.timeUnitCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.unlimited" => Some(("frequencyCap.unlimited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "insertion-order-id" => Some(("insertionOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.details" => Some(("integrationDetails.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.integration-code" => Some(("integrationDetails.integrationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-ids" => Some(("inventorySourceIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "line-item-id" => Some(("lineItemId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "line-item-type" => Some(("lineItemType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.app-id" => Some(("mobileApp.appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.display-name" => Some(("mobileApp.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.platform" => Some(("mobileApp.platform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.publisher" => Some(("mobileApp.publisher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.daily-max-impressions" => Some(("pacing.dailyMaxImpressions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.daily-max-micros" => Some(("pacing.dailyMaxMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.pacing-period" => Some(("pacing.pacingPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.pacing-type" => Some(("pacing.pacingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-revenue-model.markup-amount" => Some(("partnerRevenueModel.markupAmount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-revenue-model.markup-type" => Some(("partnerRevenueModel.markupType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reservation-type" => Some(("reservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting-expansion.exclude-first-party-audience" => Some(("targetingExpansion.excludeFirstPartyAudience", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "targeting-expansion.targeting-expansion-level" => Some(("targetingExpansion.targetingExpansionLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "warning-messages" => Some(("warningMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "app-id", "bid-amount-micros", "bid-strategy", "budget", "budget-allocation-type", "budget-unit", "campaign-id", "conversion-counting", "creative-ids", "custom-bidding-algorithm-id", "daily-max-impressions", "daily-max-micros", "date-range", "day", "details", "display-name", "end-date", "entity-status", "exclude-first-party-audience", "exclude-new-exchanges", "fixed-bid", "flight", "flight-date-type", "frequency-cap", "insertion-order-id", "integration-code", "integration-details", "inventory-source-ids", "line-item-id", "line-item-type", "markup-amount", "markup-type", "max-amount", "max-average-cpm-bid-amount-micros", "max-impressions", "maximize-spend-auto-bid", "mobile-app", "month", "name", "pacing", "pacing-period", "pacing-type", "partner-revenue-model", "performance-goal-amount-micros", "performance-goal-auto-bid", "performance-goal-type", "platform", "post-view-count-percentage-millis", "publisher", "raise-bid-for-deals", "reservation-type", "start-date", "targeting-expansion", "targeting-expansion-level", "time-unit", "time-unit-count", "trigger-id", "unlimited", "update-time", "warning-messages", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LineItem = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().line_items_create(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().line_items_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_generate_default(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "insertion-order-id" => Some(("insertionOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "line-item-type" => Some(("lineItemType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.app-id" => Some(("mobileApp.appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.display-name" => Some(("mobileApp.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.platform" => Some(("mobileApp.platform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.publisher" => Some(("mobileApp.publisher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["app-id", "display-name", "insertion-order-id", "line-item-type", "mobile-app", "platform", "publisher"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GenerateDefaultLineItemRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().line_items_generate_default(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().line_items_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().line_items_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.fixed-bid.bid-amount-micros" => Some(("bidStrategy.fixedBid.bidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.custom-bidding-algorithm-id" => Some(("bidStrategy.maximizeSpendAutoBid.customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.max-average-cpm-bid-amount-micros" => Some(("bidStrategy.maximizeSpendAutoBid.maxAverageCpmBidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.performance-goal-type" => Some(("bidStrategy.maximizeSpendAutoBid.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.maximize-spend-auto-bid.raise-bid-for-deals" => Some(("bidStrategy.maximizeSpendAutoBid.raiseBidForDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.custom-bidding-algorithm-id" => Some(("bidStrategy.performanceGoalAutoBid.customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.max-average-cpm-bid-amount-micros" => Some(("bidStrategy.performanceGoalAutoBid.maxAverageCpmBidAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.performance-goal-amount-micros" => Some(("bidStrategy.performanceGoalAutoBid.performanceGoalAmountMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bid-strategy.performance-goal-auto-bid.performance-goal-type" => Some(("bidStrategy.performanceGoalAutoBid.performanceGoalType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.budget-allocation-type" => Some(("budget.budgetAllocationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.budget-unit" => Some(("budget.budgetUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget.max-amount" => Some(("budget.maxAmount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "campaign-id" => Some(("campaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "conversion-counting.post-view-count-percentage-millis" => Some(("conversionCounting.postViewCountPercentageMillis", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creative-ids" => Some(("creativeIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-new-exchanges" => Some(("excludeNewExchanges", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "flight.date-range.end-date.day" => Some(("flight.dateRange.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.end-date.month" => Some(("flight.dateRange.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.end-date.year" => Some(("flight.dateRange.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.start-date.day" => Some(("flight.dateRange.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.start-date.month" => Some(("flight.dateRange.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.date-range.start-date.year" => Some(("flight.dateRange.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "flight.flight-date-type" => Some(("flight.flightDateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "flight.trigger-id" => Some(("flight.triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.max-impressions" => Some(("frequencyCap.maxImpressions", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit" => Some(("frequencyCap.timeUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frequency-cap.time-unit-count" => Some(("frequencyCap.timeUnitCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "frequency-cap.unlimited" => Some(("frequencyCap.unlimited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "insertion-order-id" => Some(("insertionOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.details" => Some(("integrationDetails.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.integration-code" => Some(("integrationDetails.integrationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-ids" => Some(("inventorySourceIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "line-item-id" => Some(("lineItemId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "line-item-type" => Some(("lineItemType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.app-id" => Some(("mobileApp.appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.display-name" => Some(("mobileApp.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.platform" => Some(("mobileApp.platform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-app.publisher" => Some(("mobileApp.publisher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.daily-max-impressions" => Some(("pacing.dailyMaxImpressions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.daily-max-micros" => Some(("pacing.dailyMaxMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.pacing-period" => Some(("pacing.pacingPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pacing.pacing-type" => Some(("pacing.pacingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-revenue-model.markup-amount" => Some(("partnerRevenueModel.markupAmount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-revenue-model.markup-type" => Some(("partnerRevenueModel.markupType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reservation-type" => Some(("reservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting-expansion.exclude-first-party-audience" => Some(("targetingExpansion.excludeFirstPartyAudience", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "targeting-expansion.targeting-expansion-level" => Some(("targetingExpansion.targetingExpansionLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "warning-messages" => Some(("warningMessages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "app-id", "bid-amount-micros", "bid-strategy", "budget", "budget-allocation-type", "budget-unit", "campaign-id", "conversion-counting", "creative-ids", "custom-bidding-algorithm-id", "daily-max-impressions", "daily-max-micros", "date-range", "day", "details", "display-name", "end-date", "entity-status", "exclude-first-party-audience", "exclude-new-exchanges", "fixed-bid", "flight", "flight-date-type", "frequency-cap", "insertion-order-id", "integration-code", "integration-details", "inventory-source-ids", "line-item-id", "line-item-type", "markup-amount", "markup-type", "max-amount", "max-average-cpm-bid-amount-micros", "max-impressions", "maximize-spend-auto-bid", "mobile-app", "month", "name", "pacing", "pacing-period", "pacing-type", "partner-revenue-model", "performance-goal-amount-micros", "performance-goal-auto-bid", "performance-goal-type", "platform", "post-view-count-percentage-millis", "publisher", "raise-bid-for-deals", "reservation-type", "start-date", "targeting-expansion", "targeting-expansion-level", "time-unit", "time-unit-count", "trigger-id", "unlimited", "update-time", "warning-messages", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LineItem = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().line_items_patch(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_targeting_types_assigned_targeting_options_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "age-range-details.age-range" => Some(("ageRangeDetails.ageRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "age-range-details.targeting-option-id" => Some(("ageRangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-category-details.display-name" => Some(("appCategoryDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-category-details.negative" => Some(("appCategoryDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "app-category-details.targeting-option-id" => Some(("appCategoryDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.app-id" => Some(("appDetails.appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.app-platform" => Some(("appDetails.appPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.display-name" => Some(("appDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.negative" => Some(("appDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "assigned-targeting-option-id" => Some(("assignedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-content-type-details.audio-content-type" => Some(("audioContentTypeDetails.audioContentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-content-type-details.targeting-option-id" => Some(("audioContentTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorized-seller-status-details.authorized-seller-status" => Some(("authorizedSellerStatusDetails.authorizedSellerStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorized-seller-status-details.targeting-option-id" => Some(("authorizedSellerStatusDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "browser-details.display-name" => Some(("browserDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "browser-details.negative" => Some(("browserDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "browser-details.targeting-option-id" => Some(("browserDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.display-name" => Some(("businessChainDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.proximity-radius-amount" => Some(("businessChainDetails.proximityRadiusAmount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "business-chain-details.proximity-radius-unit" => Some(("businessChainDetails.proximityRadiusUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.targeting-option-id" => Some(("businessChainDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.display-name" => Some(("carrierAndIspDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.negative" => Some(("carrierAndIspDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.targeting-option-id" => Some(("carrierAndIspDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "category-details.display-name" => Some(("categoryDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "category-details.negative" => Some(("categoryDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "category-details.targeting-option-id" => Some(("categoryDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-details.channel-id" => Some(("channelDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-details.negative" => Some(("channelDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-duration-details.content-duration" => Some(("contentDurationDetails.contentDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-duration-details.targeting-option-id" => Some(("contentDurationDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-genre-details.display-name" => Some(("contentGenreDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-genre-details.negative" => Some(("contentGenreDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-genre-details.targeting-option-id" => Some(("contentGenreDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.ad-type" => Some(("contentInstreamPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.content-instream-position" => Some(("contentInstreamPositionDetails.contentInstreamPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.targeting-option-id" => Some(("contentInstreamPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.ad-type" => Some(("contentOutstreamPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.content-outstream-position" => Some(("contentOutstreamPositionDetails.contentOutstreamPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.targeting-option-id" => Some(("contentOutstreamPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-stream-type-details.content-stream-type" => Some(("contentStreamTypeDetails.contentStreamType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-stream-type-details.targeting-option-id" => Some(("contentStreamTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "day-and-time-details.day-of-week" => Some(("dayAndTimeDetails.dayOfWeek", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "day-and-time-details.end-hour" => Some(("dayAndTimeDetails.endHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "day-and-time-details.start-hour" => Some(("dayAndTimeDetails.startHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "day-and-time-details.time-zone-resolution" => Some(("dayAndTimeDetails.timeZoneResolution", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-make-model-details.display-name" => Some(("deviceMakeModelDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-make-model-details.negative" => Some(("deviceMakeModelDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-make-model-details.targeting-option-id" => Some(("deviceMakeModelDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-type-details.device-type" => Some(("deviceTypeDetails.deviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-type-details.targeting-option-id" => Some(("deviceTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "digital-content-label-exclusion-details.content-rating-tier" => Some(("digitalContentLabelExclusionDetails.contentRatingTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "digital-content-label-exclusion-details.excluded-targeting-option-id" => Some(("digitalContentLabelExclusionDetails.excludedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-details.environment" => Some(("environmentDetails.environment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-details.targeting-option-id" => Some(("environmentDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exchange-details.targeting-option-id" => Some(("exchangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gender-details.gender" => Some(("genderDetails.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gender-details.targeting-option-id" => Some(("genderDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.display-name" => Some(("geoRegionDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.geo-region-type" => Some(("geoRegionDetails.geoRegionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.negative" => Some(("geoRegionDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "geo-region-details.targeting-option-id" => Some(("geoRegionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "household-income-details.household-income" => Some(("householdIncomeDetails.householdIncome", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "household-income-details.targeting-option-id" => Some(("householdIncomeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inheritance" => Some(("inheritance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-details.inventory-source-id" => Some(("inventorySourceDetails.inventorySourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-group-details.inventory-source-group-id" => Some(("inventorySourceGroupDetails.inventorySourceGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keyword-details.keyword" => Some(("keywordDetails.keyword", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keyword-details.negative" => Some(("keywordDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "language-details.display-name" => Some(("languageDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-details.negative" => Some(("languageDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "language-details.targeting-option-id" => Some(("languageDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-content-position-details.content-position" => Some(("nativeContentPositionDetails.contentPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-content-position-details.targeting-option-id" => Some(("nativeContentPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negative-keyword-list-details.negative-keyword-list-id" => Some(("negativeKeywordListDetails.negativeKeywordListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "omid-details.omid" => Some(("omidDetails.omid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "omid-details.targeting-option-id" => Some(("omidDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.ad-type" => Some(("onScreenPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.on-screen-position" => Some(("onScreenPositionDetails.onScreenPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.targeting-option-id" => Some(("onScreenPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operating-system-details.display-name" => Some(("operatingSystemDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operating-system-details.negative" => Some(("operatingSystemDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "operating-system-details.targeting-option-id" => Some(("operatingSystemDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parental-status-details.parental-status" => Some(("parentalStatusDetails.parentalStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parental-status-details.targeting-option-id" => Some(("parentalStatusDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.display-name" => Some(("poiDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.latitude" => Some(("poiDetails.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.longitude" => Some(("poiDetails.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.proximity-radius-amount" => Some(("poiDetails.proximityRadiusAmount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.proximity-radius-unit" => Some(("poiDetails.proximityRadiusUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.targeting-option-id" => Some(("poiDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proximity-location-list-details.proximity-location-list-id" => Some(("proximityLocationListDetails.proximityLocationListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proximity-location-list-details.proximity-radius-range" => Some(("proximityLocationListDetails.proximityRadiusRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "regional-location-list-details.negative" => Some(("regionalLocationListDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "regional-location-list-details.regional-location-list-id" => Some(("regionalLocationListDetails.regionalLocationListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sensitive-category-exclusion-details.excluded-targeting-option-id" => Some(("sensitiveCategoryExclusionDetails.excludedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sensitive-category-exclusion-details.sensitive-category" => Some(("sensitiveCategoryExclusionDetails.sensitiveCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sub-exchange-details.targeting-option-id" => Some(("subExchangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting-type" => Some(("targetingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.adloox.excluded-adloox-categories" => Some(("thirdPartyVerifierDetails.adloox.excludedAdlooxCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.app-star-rating.avoid-insufficient-star-rating" => Some(("thirdPartyVerifierDetails.doubleVerify.appStarRating.avoidInsufficientStarRating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.app-star-rating.avoided-star-rating" => Some(("thirdPartyVerifierDetails.doubleVerify.appStarRating.avoidedStarRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.avoided-age-ratings" => Some(("thirdPartyVerifierDetails.doubleVerify.avoidedAgeRatings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoid-unknown-brand-safety-category" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidUnknownBrandSafetyCategory", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoided-high-severity-categories" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidedHighSeverityCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoided-medium-severity-categories" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidedMediumSeverityCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.custom-segment-id" => Some(("thirdPartyVerifierDetails.doubleVerify.customSegmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.display-viewability.iab" => Some(("thirdPartyVerifierDetails.doubleVerify.displayViewability.iab", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.display-viewability.viewable-during" => Some(("thirdPartyVerifierDetails.doubleVerify.displayViewability.viewableDuring", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.fraud-invalid-traffic.avoid-insufficient-option" => Some(("thirdPartyVerifierDetails.doubleVerify.fraudInvalidTraffic.avoidInsufficientOption", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.fraud-invalid-traffic.avoided-fraud-option" => Some(("thirdPartyVerifierDetails.doubleVerify.fraudInvalidTraffic.avoidedFraudOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.player-impression-rate" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.playerImpressionRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.video-iab" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.videoIab", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.video-viewable-rate" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.videoViewableRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.custom-segment-id" => Some(("thirdPartyVerifierDetails.integralAdScience.customSegmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.integral-ad-science.display-viewability" => Some(("thirdPartyVerifierDetails.integralAdScience.displayViewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.exclude-unrateable" => Some(("thirdPartyVerifierDetails.integralAdScience.excludeUnrateable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-ad-fraud-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAdFraudRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-adult-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAdultRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-alcohol-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAlcoholRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-drugs-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedDrugsRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-gambling-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedGamblingRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-hate-speech-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedHateSpeechRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-illegal-downloads-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedIllegalDownloadsRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-offensive-language-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedOffensiveLanguageRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-violence-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedViolenceRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.traq-score-option" => Some(("thirdPartyVerifierDetails.integralAdScience.traqScoreOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.video-viewability" => Some(("thirdPartyVerifierDetails.integralAdScience.videoViewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-details.negative" => Some(("urlDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-details.url" => Some(("urlDetails.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-rewarded-content-details.targeting-option-id" => Some(("userRewardedContentDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-rewarded-content-details.user-rewarded-content" => Some(("userRewardedContentDetails.userRewardedContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-player-size-details.targeting-option-id" => Some(("videoPlayerSizeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-player-size-details.video-player-size" => Some(("videoPlayerSizeDetails.videoPlayerSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewability-details.targeting-option-id" => Some(("viewabilityDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewability-details.viewability" => Some(("viewabilityDetails.viewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-type", "adloox", "age-range", "age-range-details", "app-category-details", "app-details", "app-id", "app-platform", "app-star-rating", "assigned-targeting-option-id", "audio-content-type", "audio-content-type-details", "authorized-seller-status", "authorized-seller-status-details", "avoid-insufficient-option", "avoid-insufficient-star-rating", "avoid-unknown-brand-safety-category", "avoided-age-ratings", "avoided-fraud-option", "avoided-high-severity-categories", "avoided-medium-severity-categories", "avoided-star-rating", "brand-safety-categories", "browser-details", "business-chain-details", "carrier-and-isp-details", "category-details", "channel-details", "channel-id", "content-duration", "content-duration-details", "content-genre-details", "content-instream-position", "content-instream-position-details", "content-outstream-position", "content-outstream-position-details", "content-position", "content-rating-tier", "content-stream-type", "content-stream-type-details", "custom-segment-id", "day-and-time-details", "day-of-week", "device-make-model-details", "device-type", "device-type-details", "digital-content-label-exclusion-details", "display-name", "display-viewability", "double-verify", "end-hour", "environment", "environment-details", "exchange-details", "exclude-unrateable", "excluded-ad-fraud-risk", "excluded-adloox-categories", "excluded-adult-risk", "excluded-alcohol-risk", "excluded-drugs-risk", "excluded-gambling-risk", "excluded-hate-speech-risk", "excluded-illegal-downloads-risk", "excluded-offensive-language-risk", "excluded-targeting-option-id", "excluded-violence-risk", "fraud-invalid-traffic", "gender", "gender-details", "geo-region-details", "geo-region-type", "household-income", "household-income-details", "iab", "inheritance", "integral-ad-science", "inventory-source-details", "inventory-source-group-details", "inventory-source-group-id", "inventory-source-id", "keyword", "keyword-details", "language-details", "latitude", "longitude", "name", "native-content-position-details", "negative", "negative-keyword-list-details", "negative-keyword-list-id", "omid", "omid-details", "on-screen-position", "on-screen-position-details", "operating-system-details", "parental-status", "parental-status-details", "player-impression-rate", "poi-details", "proximity-location-list-details", "proximity-location-list-id", "proximity-radius-amount", "proximity-radius-range", "proximity-radius-unit", "regional-location-list-details", "regional-location-list-id", "sensitive-category", "sensitive-category-exclusion-details", "start-hour", "sub-exchange-details", "targeting-option-id", "targeting-type", "third-party-verifier-details", "time-zone-resolution", "traq-score-option", "url", "url-details", "user-rewarded-content", "user-rewarded-content-details", "video-iab", "video-player-size", "video-player-size-details", "video-viewability", "video-viewable-rate", "viewability", "viewability-details", "viewable-during"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AssignedTargetingOption = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().line_items_targeting_types_assigned_targeting_options_create(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_targeting_types_assigned_targeting_options_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().line_items_targeting_types_assigned_targeting_options_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""), opt.value_of("assigned-targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_targeting_types_assigned_targeting_options_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().line_items_targeting_types_assigned_targeting_options_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""), opt.value_of("assigned-targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_line_items_targeting_types_assigned_targeting_options_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().line_items_targeting_types_assigned_targeting_options_list(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("line-item-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_location_lists_assigned_locations_bulk_edit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "deleted-assigned-locations" => Some(("deletedAssignedLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["deleted-assigned-locations"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditAssignedLocationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().location_lists_assigned_locations_bulk_edit(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("location-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_location_lists_assigned_locations_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "assigned-location-id" => Some(("assignedLocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting-option-id" => Some(("targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["assigned-location-id", "name", "targeting-option-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AssignedLocation = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().location_lists_assigned_locations_create(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("location-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_location_lists_assigned_locations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().location_lists_assigned_locations_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("location-list-id").unwrap_or(""), opt.value_of("assigned-location-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_location_lists_assigned_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().location_lists_assigned_locations_list(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("location-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_location_lists_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-list-id" => Some(("locationListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-type" => Some(("locationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "display-name", "location-list-id", "location-type", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LocationList = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().location_lists_create(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_location_lists_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().location_lists_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("location-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_location_lists_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().location_lists_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_location_lists_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-list-id" => Some(("locationListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-type" => Some(("locationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "display-name", "location-list-id", "location-type", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LocationList = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().location_lists_patch(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("location-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_manual_triggers_activate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ActivateManualTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().manual_triggers_activate(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_manual_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "activation-duration-minutes" => Some(("activationDurationMinutes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-activation-time" => Some(("latestActivationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-duration-minutes", "advertiser-id", "display-name", "latest-activation-time", "name", "state", "trigger-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ManualTrigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().manual_triggers_create(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_manual_triggers_deactivate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DeactivateManualTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().manual_triggers_deactivate(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_manual_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().manual_triggers_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_manual_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().manual_triggers_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_manual_triggers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "activation-duration-minutes" => Some(("activationDurationMinutes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-activation-time" => Some(("latestActivationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-duration-minutes", "advertiser-id", "display-name", "latest-activation-time", "name", "state", "trigger-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ManualTrigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().manual_triggers_patch(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negative-keyword-list-id" => Some(("negativeKeywordListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeted-line-item-count" => Some(("targetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "display-name", "name", "negative-keyword-list-id", "targeted-line-item-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::NegativeKeywordList = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().negative_keyword_lists_create(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().negative_keyword_lists_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("negative-keyword-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().negative_keyword_lists_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("negative-keyword-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().negative_keyword_lists_list(opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_negative_keywords_bulk_edit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "deleted-negative-keywords" => Some(("deletedNegativeKeywords", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["deleted-negative-keywords"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditNegativeKeywordsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().negative_keyword_lists_negative_keywords_bulk_edit(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("negative-keyword-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_negative_keywords_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "keyword-value" => Some(("keywordValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["keyword-value", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::NegativeKeyword = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().negative_keyword_lists_negative_keywords_create(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("negative-keyword-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_negative_keywords_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().negative_keyword_lists_negative_keywords_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("negative-keyword-list-id").unwrap_or(""), opt.value_of("keyword-value").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_negative_keywords_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().negative_keyword_lists_negative_keywords_list(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("negative-keyword-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_negative_keywords_replace(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReplaceNegativeKeywordsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().negative_keyword_lists_negative_keywords_replace(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("negative-keyword-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_negative_keyword_lists_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negative-keyword-list-id" => Some(("negativeKeywordListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeted-line-item-count" => Some(("targetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "display-name", "name", "negative-keyword-list-id", "targeted-line-item-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::NegativeKeywordList = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().negative_keyword_lists_patch(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("negative-keyword-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "ad-server-config.cm-hybrid-config.cm-account-id" => Some(("adServerConfig.cmHybridConfig.cmAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ad-server-config.cm-hybrid-config.cm-floodlight-config-id" => Some(("adServerConfig.cmHybridConfig.cmFloodlightConfigId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ad-server-config.cm-hybrid-config.cm-floodlight-linking-authorized" => Some(("adServerConfig.cmHybridConfig.cmFloodlightLinkingAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ad-server-config.cm-hybrid-config.cm-syncable-site-ids" => Some(("adServerConfig.cmHybridConfig.cmSyncableSiteIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ad-server-config.cm-hybrid-config.dv360-to-cm-cost-reporting-enabled" => Some(("adServerConfig.cmHybridConfig.dv360ToCmCostReportingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ad-server-config.cm-hybrid-config.dv360-to-cm-data-sharing-enabled" => Some(("adServerConfig.cmHybridConfig.dv360ToCmDataSharingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "ad-server-config.third-party-only-config.pixel-order-id-reporting-enabled" => Some(("adServerConfig.thirdPartyOnlyConfig.pixelOrderIdReportingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-config.dynamic-creative-enabled" => Some(("creativeConfig.dynamicCreativeEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creative-config.ias-client-id" => Some(("creativeConfig.iasClientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-config.oba-compliance-disabled" => Some(("creativeConfig.obaComplianceDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creative-config.video-creative-data-sharing-authorized" => Some(("creativeConfig.videoCreativeDataSharingAuthorized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "data-access-config.sdf-config.override-partner-sdf-config" => Some(("dataAccessConfig.sdfConfig.overridePartnerSdfConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "data-access-config.sdf-config.sdf-config.admin-email" => Some(("dataAccessConfig.sdfConfig.sdfConfig.adminEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-access-config.sdf-config.sdf-config.version" => Some(("dataAccessConfig.sdfConfig.sdfConfig.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "general-config.currency-code" => Some(("generalConfig.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "general-config.domain-url" => Some(("generalConfig.domainUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "general-config.time-zone" => Some(("generalConfig.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.details" => Some(("integrationDetails.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "integration-details.integration-code" => Some(("integrationDetails.integrationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "prisma-enabled" => Some(("prismaEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "serving-config.exempt-tv-from-viewability-targeting" => Some(("servingConfig.exemptTvFromViewabilityTargeting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-server-config", "admin-email", "advertiser-id", "cm-account-id", "cm-floodlight-config-id", "cm-floodlight-linking-authorized", "cm-hybrid-config", "cm-syncable-site-ids", "creative-config", "currency-code", "data-access-config", "details", "display-name", "domain-url", "dv360-to-cm-cost-reporting-enabled", "dv360-to-cm-data-sharing-enabled", "dynamic-creative-enabled", "entity-status", "exempt-tv-from-viewability-targeting", "general-config", "ias-client-id", "integration-code", "integration-details", "name", "oba-compliance-disabled", "override-partner-sdf-config", "partner-id", "pixel-order-id-reporting-enabled", "prisma-enabled", "sdf-config", "serving-config", "third-party-only-config", "time-zone", "update-time", "version", "video-creative-data-sharing-authorized"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Advertiser = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().patch(request, opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_targeting_types_assigned_targeting_options_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "age-range-details.age-range" => Some(("ageRangeDetails.ageRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "age-range-details.targeting-option-id" => Some(("ageRangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-category-details.display-name" => Some(("appCategoryDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-category-details.negative" => Some(("appCategoryDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "app-category-details.targeting-option-id" => Some(("appCategoryDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.app-id" => Some(("appDetails.appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.app-platform" => Some(("appDetails.appPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.display-name" => Some(("appDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.negative" => Some(("appDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "assigned-targeting-option-id" => Some(("assignedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-content-type-details.audio-content-type" => Some(("audioContentTypeDetails.audioContentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-content-type-details.targeting-option-id" => Some(("audioContentTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorized-seller-status-details.authorized-seller-status" => Some(("authorizedSellerStatusDetails.authorizedSellerStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorized-seller-status-details.targeting-option-id" => Some(("authorizedSellerStatusDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "browser-details.display-name" => Some(("browserDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "browser-details.negative" => Some(("browserDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "browser-details.targeting-option-id" => Some(("browserDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.display-name" => Some(("businessChainDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.proximity-radius-amount" => Some(("businessChainDetails.proximityRadiusAmount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "business-chain-details.proximity-radius-unit" => Some(("businessChainDetails.proximityRadiusUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.targeting-option-id" => Some(("businessChainDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.display-name" => Some(("carrierAndIspDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.negative" => Some(("carrierAndIspDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.targeting-option-id" => Some(("carrierAndIspDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "category-details.display-name" => Some(("categoryDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "category-details.negative" => Some(("categoryDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "category-details.targeting-option-id" => Some(("categoryDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-details.channel-id" => Some(("channelDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-details.negative" => Some(("channelDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-duration-details.content-duration" => Some(("contentDurationDetails.contentDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-duration-details.targeting-option-id" => Some(("contentDurationDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-genre-details.display-name" => Some(("contentGenreDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-genre-details.negative" => Some(("contentGenreDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-genre-details.targeting-option-id" => Some(("contentGenreDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.ad-type" => Some(("contentInstreamPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.content-instream-position" => Some(("contentInstreamPositionDetails.contentInstreamPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.targeting-option-id" => Some(("contentInstreamPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.ad-type" => Some(("contentOutstreamPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.content-outstream-position" => Some(("contentOutstreamPositionDetails.contentOutstreamPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.targeting-option-id" => Some(("contentOutstreamPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-stream-type-details.content-stream-type" => Some(("contentStreamTypeDetails.contentStreamType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-stream-type-details.targeting-option-id" => Some(("contentStreamTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "day-and-time-details.day-of-week" => Some(("dayAndTimeDetails.dayOfWeek", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "day-and-time-details.end-hour" => Some(("dayAndTimeDetails.endHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "day-and-time-details.start-hour" => Some(("dayAndTimeDetails.startHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "day-and-time-details.time-zone-resolution" => Some(("dayAndTimeDetails.timeZoneResolution", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-make-model-details.display-name" => Some(("deviceMakeModelDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-make-model-details.negative" => Some(("deviceMakeModelDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-make-model-details.targeting-option-id" => Some(("deviceMakeModelDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-type-details.device-type" => Some(("deviceTypeDetails.deviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-type-details.targeting-option-id" => Some(("deviceTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "digital-content-label-exclusion-details.content-rating-tier" => Some(("digitalContentLabelExclusionDetails.contentRatingTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "digital-content-label-exclusion-details.excluded-targeting-option-id" => Some(("digitalContentLabelExclusionDetails.excludedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-details.environment" => Some(("environmentDetails.environment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-details.targeting-option-id" => Some(("environmentDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exchange-details.targeting-option-id" => Some(("exchangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gender-details.gender" => Some(("genderDetails.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gender-details.targeting-option-id" => Some(("genderDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.display-name" => Some(("geoRegionDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.geo-region-type" => Some(("geoRegionDetails.geoRegionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.negative" => Some(("geoRegionDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "geo-region-details.targeting-option-id" => Some(("geoRegionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "household-income-details.household-income" => Some(("householdIncomeDetails.householdIncome", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "household-income-details.targeting-option-id" => Some(("householdIncomeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inheritance" => Some(("inheritance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-details.inventory-source-id" => Some(("inventorySourceDetails.inventorySourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-group-details.inventory-source-group-id" => Some(("inventorySourceGroupDetails.inventorySourceGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keyword-details.keyword" => Some(("keywordDetails.keyword", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keyword-details.negative" => Some(("keywordDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "language-details.display-name" => Some(("languageDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-details.negative" => Some(("languageDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "language-details.targeting-option-id" => Some(("languageDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-content-position-details.content-position" => Some(("nativeContentPositionDetails.contentPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-content-position-details.targeting-option-id" => Some(("nativeContentPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negative-keyword-list-details.negative-keyword-list-id" => Some(("negativeKeywordListDetails.negativeKeywordListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "omid-details.omid" => Some(("omidDetails.omid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "omid-details.targeting-option-id" => Some(("omidDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.ad-type" => Some(("onScreenPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.on-screen-position" => Some(("onScreenPositionDetails.onScreenPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.targeting-option-id" => Some(("onScreenPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operating-system-details.display-name" => Some(("operatingSystemDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operating-system-details.negative" => Some(("operatingSystemDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "operating-system-details.targeting-option-id" => Some(("operatingSystemDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parental-status-details.parental-status" => Some(("parentalStatusDetails.parentalStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parental-status-details.targeting-option-id" => Some(("parentalStatusDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.display-name" => Some(("poiDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.latitude" => Some(("poiDetails.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.longitude" => Some(("poiDetails.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.proximity-radius-amount" => Some(("poiDetails.proximityRadiusAmount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.proximity-radius-unit" => Some(("poiDetails.proximityRadiusUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.targeting-option-id" => Some(("poiDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proximity-location-list-details.proximity-location-list-id" => Some(("proximityLocationListDetails.proximityLocationListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proximity-location-list-details.proximity-radius-range" => Some(("proximityLocationListDetails.proximityRadiusRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "regional-location-list-details.negative" => Some(("regionalLocationListDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "regional-location-list-details.regional-location-list-id" => Some(("regionalLocationListDetails.regionalLocationListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sensitive-category-exclusion-details.excluded-targeting-option-id" => Some(("sensitiveCategoryExclusionDetails.excludedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sensitive-category-exclusion-details.sensitive-category" => Some(("sensitiveCategoryExclusionDetails.sensitiveCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sub-exchange-details.targeting-option-id" => Some(("subExchangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting-type" => Some(("targetingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.adloox.excluded-adloox-categories" => Some(("thirdPartyVerifierDetails.adloox.excludedAdlooxCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.app-star-rating.avoid-insufficient-star-rating" => Some(("thirdPartyVerifierDetails.doubleVerify.appStarRating.avoidInsufficientStarRating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.app-star-rating.avoided-star-rating" => Some(("thirdPartyVerifierDetails.doubleVerify.appStarRating.avoidedStarRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.avoided-age-ratings" => Some(("thirdPartyVerifierDetails.doubleVerify.avoidedAgeRatings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoid-unknown-brand-safety-category" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidUnknownBrandSafetyCategory", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoided-high-severity-categories" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidedHighSeverityCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoided-medium-severity-categories" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidedMediumSeverityCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.custom-segment-id" => Some(("thirdPartyVerifierDetails.doubleVerify.customSegmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.display-viewability.iab" => Some(("thirdPartyVerifierDetails.doubleVerify.displayViewability.iab", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.display-viewability.viewable-during" => Some(("thirdPartyVerifierDetails.doubleVerify.displayViewability.viewableDuring", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.fraud-invalid-traffic.avoid-insufficient-option" => Some(("thirdPartyVerifierDetails.doubleVerify.fraudInvalidTraffic.avoidInsufficientOption", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.fraud-invalid-traffic.avoided-fraud-option" => Some(("thirdPartyVerifierDetails.doubleVerify.fraudInvalidTraffic.avoidedFraudOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.player-impression-rate" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.playerImpressionRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.video-iab" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.videoIab", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.video-viewable-rate" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.videoViewableRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.custom-segment-id" => Some(("thirdPartyVerifierDetails.integralAdScience.customSegmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.integral-ad-science.display-viewability" => Some(("thirdPartyVerifierDetails.integralAdScience.displayViewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.exclude-unrateable" => Some(("thirdPartyVerifierDetails.integralAdScience.excludeUnrateable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-ad-fraud-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAdFraudRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-adult-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAdultRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-alcohol-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAlcoholRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-drugs-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedDrugsRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-gambling-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedGamblingRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-hate-speech-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedHateSpeechRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-illegal-downloads-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedIllegalDownloadsRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-offensive-language-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedOffensiveLanguageRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-violence-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedViolenceRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.traq-score-option" => Some(("thirdPartyVerifierDetails.integralAdScience.traqScoreOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.video-viewability" => Some(("thirdPartyVerifierDetails.integralAdScience.videoViewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-details.negative" => Some(("urlDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-details.url" => Some(("urlDetails.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-rewarded-content-details.targeting-option-id" => Some(("userRewardedContentDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-rewarded-content-details.user-rewarded-content" => Some(("userRewardedContentDetails.userRewardedContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-player-size-details.targeting-option-id" => Some(("videoPlayerSizeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-player-size-details.video-player-size" => Some(("videoPlayerSizeDetails.videoPlayerSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewability-details.targeting-option-id" => Some(("viewabilityDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewability-details.viewability" => Some(("viewabilityDetails.viewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-type", "adloox", "age-range", "age-range-details", "app-category-details", "app-details", "app-id", "app-platform", "app-star-rating", "assigned-targeting-option-id", "audio-content-type", "audio-content-type-details", "authorized-seller-status", "authorized-seller-status-details", "avoid-insufficient-option", "avoid-insufficient-star-rating", "avoid-unknown-brand-safety-category", "avoided-age-ratings", "avoided-fraud-option", "avoided-high-severity-categories", "avoided-medium-severity-categories", "avoided-star-rating", "brand-safety-categories", "browser-details", "business-chain-details", "carrier-and-isp-details", "category-details", "channel-details", "channel-id", "content-duration", "content-duration-details", "content-genre-details", "content-instream-position", "content-instream-position-details", "content-outstream-position", "content-outstream-position-details", "content-position", "content-rating-tier", "content-stream-type", "content-stream-type-details", "custom-segment-id", "day-and-time-details", "day-of-week", "device-make-model-details", "device-type", "device-type-details", "digital-content-label-exclusion-details", "display-name", "display-viewability", "double-verify", "end-hour", "environment", "environment-details", "exchange-details", "exclude-unrateable", "excluded-ad-fraud-risk", "excluded-adloox-categories", "excluded-adult-risk", "excluded-alcohol-risk", "excluded-drugs-risk", "excluded-gambling-risk", "excluded-hate-speech-risk", "excluded-illegal-downloads-risk", "excluded-offensive-language-risk", "excluded-targeting-option-id", "excluded-violence-risk", "fraud-invalid-traffic", "gender", "gender-details", "geo-region-details", "geo-region-type", "household-income", "household-income-details", "iab", "inheritance", "integral-ad-science", "inventory-source-details", "inventory-source-group-details", "inventory-source-group-id", "inventory-source-id", "keyword", "keyword-details", "language-details", "latitude", "longitude", "name", "native-content-position-details", "negative", "negative-keyword-list-details", "negative-keyword-list-id", "omid", "omid-details", "on-screen-position", "on-screen-position-details", "operating-system-details", "parental-status", "parental-status-details", "player-impression-rate", "poi-details", "proximity-location-list-details", "proximity-location-list-id", "proximity-radius-amount", "proximity-radius-range", "proximity-radius-unit", "regional-location-list-details", "regional-location-list-id", "sensitive-category", "sensitive-category-exclusion-details", "start-hour", "sub-exchange-details", "targeting-option-id", "targeting-type", "third-party-verifier-details", "time-zone-resolution", "traq-score-option", "url", "url-details", "user-rewarded-content", "user-rewarded-content-details", "video-iab", "video-player-size", "video-player-size-details", "video-viewability", "video-viewable-rate", "viewability", "viewability-details", "viewable-during"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AssignedTargetingOption = json::value::from_value(object).unwrap();
        let mut call = self.hub.advertisers().targeting_types_assigned_targeting_options_create(request, opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_targeting_types_assigned_targeting_options_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().targeting_types_assigned_targeting_options_delete(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""), opt.value_of("assigned-targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_targeting_types_assigned_targeting_options_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().targeting_types_assigned_targeting_options_get(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""), opt.value_of("assigned-targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _advertisers_targeting_types_assigned_targeting_options_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.advertisers().targeting_types_assigned_targeting_options_list(opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _combined_audiences_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.combined_audiences().get(opt.value_of("combined-audience-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _combined_audiences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.combined_audiences().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_bidding_algorithms_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-bidding-algorithm-id" => Some(("customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-bidding-algorithm-state" => Some(("customBiddingAlgorithmState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-bidding-algorithm-type" => Some(("customBiddingAlgorithmType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shared-advertiser-ids" => Some(("sharedAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "custom-bidding-algorithm-id", "custom-bidding-algorithm-state", "custom-bidding-algorithm-type", "display-name", "entity-status", "name", "partner-id", "shared-advertiser-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomBiddingAlgorithm = json::value::from_value(object).unwrap();
        let mut call = self.hub.custom_bidding_algorithms().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_bidding_algorithms_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.custom_bidding_algorithms().get(opt.value_of("custom-bidding-algorithm-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_bidding_algorithms_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.custom_bidding_algorithms().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_bidding_algorithms_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-bidding-algorithm-id" => Some(("customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-bidding-algorithm-state" => Some(("customBiddingAlgorithmState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-bidding-algorithm-type" => Some(("customBiddingAlgorithmType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity-status" => Some(("entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "shared-advertiser-ids" => Some(("sharedAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "custom-bidding-algorithm-id", "custom-bidding-algorithm-state", "custom-bidding-algorithm-type", "display-name", "entity-status", "name", "partner-id", "shared-advertiser-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomBiddingAlgorithm = json::value::from_value(object).unwrap();
        let mut call = self.hub.custom_bidding_algorithms().patch(request, opt.value_of("custom-bidding-algorithm-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_bidding_algorithms_scripts_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-bidding-algorithm-id" => Some(("customBiddingAlgorithmId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-bidding-script-id" => Some(("customBiddingScriptId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "script.resource-name" => Some(("script.resourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active", "create-time", "custom-bidding-algorithm-id", "custom-bidding-script-id", "name", "resource-name", "script", "state"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomBiddingScript = json::value::from_value(object).unwrap();
        let mut call = self.hub.custom_bidding_algorithms().scripts_create(request, opt.value_of("custom-bidding-algorithm-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_bidding_algorithms_scripts_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.custom_bidding_algorithms().scripts_get(opt.value_of("custom-bidding-algorithm-id").unwrap_or(""), opt.value_of("custom-bidding-script-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_bidding_algorithms_scripts_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.custom_bidding_algorithms().scripts_list(opt.value_of("custom-bidding-algorithm-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_bidding_algorithms_upload_script(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.custom_bidding_algorithms().upload_script(opt.value_of("custom-bidding-algorithm-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_lists_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.custom_lists().get(opt.value_of("custom-list-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _custom_lists_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.custom_lists().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _first_and_third_party_audiences_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "active-display-audience-size" => Some(("activeDisplayAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-id" => Some(("appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-source" => Some(("audienceSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-type" => Some(("audienceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-audience-size" => Some(("displayAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-desktop-audience-size" => Some(("displayDesktopAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-mobile-app-audience-size" => Some(("displayMobileAppAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-mobile-web-audience-size" => Some(("displayMobileWebAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "first-and-third-party-audience-id" => Some(("firstAndThirdPartyAudienceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "first-and-third-party-audience-type" => Some(("firstAndThirdPartyAudienceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gmail-audience-size" => Some(("gmailAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "membership-duration-days" => Some(("membershipDurationDays", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-device-id-list.mobile-device-ids" => Some(("mobileDeviceIdList.mobileDeviceIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "youtube-audience-size" => Some(("youtubeAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-display-audience-size", "app-id", "audience-source", "audience-type", "description", "display-audience-size", "display-desktop-audience-size", "display-mobile-app-audience-size", "display-mobile-web-audience-size", "display-name", "first-and-third-party-audience-id", "first-and-third-party-audience-type", "gmail-audience-size", "membership-duration-days", "mobile-device-id-list", "mobile-device-ids", "name", "youtube-audience-size"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FirstAndThirdPartyAudience = json::value::from_value(object).unwrap();
        let mut call = self.hub.first_and_third_party_audiences().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _first_and_third_party_audiences_edit_customer_match_members(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "added-mobile-device-id-list.mobile-device-ids" => Some(("addedMobileDeviceIdList.mobileDeviceIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["added-mobile-device-id-list", "advertiser-id", "mobile-device-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EditCustomerMatchMembersRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.first_and_third_party_audiences().edit_customer_match_members(request, opt.value_of("first-and-third-party-audience-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _first_and_third_party_audiences_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.first_and_third_party_audiences().get(opt.value_of("first-and-third-party-audience-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _first_and_third_party_audiences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.first_and_third_party_audiences().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _first_and_third_party_audiences_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "active-display-audience-size" => Some(("activeDisplayAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-id" => Some(("appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-source" => Some(("audienceSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-type" => Some(("audienceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-audience-size" => Some(("displayAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-desktop-audience-size" => Some(("displayDesktopAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-mobile-app-audience-size" => Some(("displayMobileAppAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-mobile-web-audience-size" => Some(("displayMobileWebAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "first-and-third-party-audience-id" => Some(("firstAndThirdPartyAudienceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "first-and-third-party-audience-type" => Some(("firstAndThirdPartyAudienceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gmail-audience-size" => Some(("gmailAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "membership-duration-days" => Some(("membershipDurationDays", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "mobile-device-id-list.mobile-device-ids" => Some(("mobileDeviceIdList.mobileDeviceIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "youtube-audience-size" => Some(("youtubeAudienceSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-display-audience-size", "app-id", "audience-source", "audience-type", "description", "display-audience-size", "display-desktop-audience-size", "display-mobile-app-audience-size", "display-mobile-web-audience-size", "display-name", "first-and-third-party-audience-id", "first-and-third-party-audience-type", "gmail-audience-size", "membership-duration-days", "mobile-device-id-list", "mobile-device-ids", "name", "youtube-audience-size"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FirstAndThirdPartyAudience = json::value::from_value(object).unwrap();
        let mut call = self.hub.first_and_third_party_audiences().patch(request, opt.value_of("first-and-third-party-audience-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _floodlight_groups_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.floodlight_groups().get(opt.value_of("floodlight-group-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _floodlight_groups_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "active-view-config.display-name" => Some(("activeViewConfig.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-view-config.minimum-duration" => Some(("activeViewConfig.minimumDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-view-config.minimum-quartile" => Some(("activeViewConfig.minimumQuartile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-view-config.minimum-viewability" => Some(("activeViewConfig.minimumViewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-view-config.minimum-volume" => Some(("activeViewConfig.minimumVolume", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "floodlight-group-id" => Some(("floodlightGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lookback-window.click-days" => Some(("lookbackWindow.clickDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "lookback-window.impression-days" => Some(("lookbackWindow.impressionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-tag-type" => Some(("webTagType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-view-config", "click-days", "display-name", "floodlight-group-id", "impression-days", "lookback-window", "minimum-duration", "minimum-quartile", "minimum-viewability", "minimum-volume", "name", "web-tag-type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FloodlightGroup = json::value::from_value(object).unwrap();
        let mut call = self.hub.floodlight_groups().patch(request, opt.value_of("floodlight-group-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["partner-id", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _google_audiences_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.google_audiences().get(opt.value_of("google-audience-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _google_audiences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.google_audiences().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _guaranteed_orders_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "default-advertiser-id" => Some(("defaultAdvertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-campaign-id" => Some(("defaultCampaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exchange" => Some(("exchange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "guaranteed-order-id" => Some(("guaranteedOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "legacy-guaranteed-order-id" => Some(("legacyGuaranteedOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publisher-name" => Some(("publisherName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-access-inherited" => Some(("readAccessInherited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "read-advertiser-ids" => Some(("readAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "read-write-advertiser-id" => Some(("readWriteAdvertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-write-partner-id" => Some(("readWritePartnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.config-status" => Some(("status.configStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.entity-pause-reason" => Some(("status.entityPauseReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.entity-status" => Some(("status.entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["config-status", "default-advertiser-id", "default-campaign-id", "display-name", "entity-pause-reason", "entity-status", "exchange", "guaranteed-order-id", "legacy-guaranteed-order-id", "name", "publisher-name", "read-access-inherited", "read-advertiser-ids", "read-write-advertiser-id", "read-write-partner-id", "status", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GuaranteedOrder = json::value::from_value(object).unwrap();
        let mut call = self.hub.guaranteed_orders().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _guaranteed_orders_edit_guaranteed_order_read_accessors(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "added-advertisers" => Some(("addedAdvertisers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-access-inherited" => Some(("readAccessInherited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "removed-advertisers" => Some(("removedAdvertisers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["added-advertisers", "partner-id", "read-access-inherited", "removed-advertisers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EditGuaranteedOrderReadAccessorsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.guaranteed_orders().edit_guaranteed_order_read_accessors(request, opt.value_of("guaranteed-order-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _guaranteed_orders_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.guaranteed_orders().get(opt.value_of("guaranteed-order-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _guaranteed_orders_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.guaranteed_orders().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _guaranteed_orders_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "default-advertiser-id" => Some(("defaultAdvertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-campaign-id" => Some(("defaultCampaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exchange" => Some(("exchange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "guaranteed-order-id" => Some(("guaranteedOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "legacy-guaranteed-order-id" => Some(("legacyGuaranteedOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publisher-name" => Some(("publisherName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-access-inherited" => Some(("readAccessInherited", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "read-advertiser-ids" => Some(("readAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "read-write-advertiser-id" => Some(("readWriteAdvertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-write-partner-id" => Some(("readWritePartnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.config-status" => Some(("status.configStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.entity-pause-reason" => Some(("status.entityPauseReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.entity-status" => Some(("status.entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["config-status", "default-advertiser-id", "default-campaign-id", "display-name", "entity-pause-reason", "entity-status", "exchange", "guaranteed-order-id", "legacy-guaranteed-order-id", "name", "publisher-name", "read-access-inherited", "read-advertiser-ids", "read-write-advertiser-id", "read-write-partner-id", "status", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GuaranteedOrder = json::value::from_value(object).unwrap();
        let mut call = self.hub.guaranteed_orders().patch(request, opt.value_of("guaranteed-order-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_assigned_inventory_sources_bulk_edit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted-assigned-inventory-sources" => Some(("deletedAssignedInventorySources", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "deleted-assigned-inventory-sources", "partner-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditAssignedInventorySourcesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.inventory_source_groups().assigned_inventory_sources_bulk_edit(request, opt.value_of("inventory-source-group-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_assigned_inventory_sources_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "assigned-inventory-source-id" => Some(("assignedInventorySourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-id" => Some(("inventorySourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["assigned-inventory-source-id", "inventory-source-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AssignedInventorySource = json::value::from_value(object).unwrap();
        let mut call = self.hub.inventory_source_groups().assigned_inventory_sources_create(request, opt.value_of("inventory-source-group-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_assigned_inventory_sources_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.inventory_source_groups().assigned_inventory_sources_delete(opt.value_of("inventory-source-group-id").unwrap_or(""), opt.value_of("assigned-inventory-source-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_assigned_inventory_sources_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.inventory_source_groups().assigned_inventory_sources_list(opt.value_of("inventory-source-group-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-group-id" => Some(("inventorySourceGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "inventory-source-group-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InventorySourceGroup = json::value::from_value(object).unwrap();
        let mut call = self.hub.inventory_source_groups().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.inventory_source_groups().delete(opt.value_of("inventory-source-group-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.inventory_source_groups().get(opt.value_of("inventory-source-group-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.inventory_source_groups().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_source_groups_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-group-id" => Some(("inventorySourceGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "inventory-source-group-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InventorySourceGroup = json::value::from_value(object).unwrap();
        let mut call = self.hub.inventory_source_groups().patch(request, opt.value_of("inventory-source-group-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_sources_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "commitment" => Some(("commitment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deal-id" => Some(("dealId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delivery-method" => Some(("deliveryMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exchange" => Some(("exchange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "guaranteed-order-id" => Some(("guaranteedOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-id" => Some(("inventorySourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-product-type" => Some(("inventorySourceProductType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-type" => Some(("inventorySourceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publisher-name" => Some(("publisherName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.inventory-source-rate-type" => Some(("rateDetails.inventorySourceRateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.minimum-spend.currency-code" => Some(("rateDetails.minimumSpend.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.minimum-spend.nanos" => Some(("rateDetails.minimumSpend.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "rate-details.minimum-spend.units" => Some(("rateDetails.minimumSpend.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.rate.currency-code" => Some(("rateDetails.rate.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.rate.nanos" => Some(("rateDetails.rate.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "rate-details.rate.units" => Some(("rateDetails.rate.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.units-purchased" => Some(("rateDetails.unitsPurchased", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-advertiser-ids" => Some(("readAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "read-partner-ids" => Some(("readPartnerIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "read-write-accessors.advertisers.advertiser-ids" => Some(("readWriteAccessors.advertisers.advertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "read-write-accessors.partner.partner-id" => Some(("readWriteAccessors.partner.partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.config-status" => Some(("status.configStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.entity-pause-reason" => Some(("status.entityPauseReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.entity-status" => Some(("status.entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.seller-pause-reason" => Some(("status.sellerPauseReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.seller-status" => Some(("status.sellerStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sub-site-property-id" => Some(("subSitePropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.end-time" => Some(("timeRange.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.start-time" => Some(("timeRange.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-ids", "advertisers", "commitment", "config-status", "currency-code", "deal-id", "delivery-method", "display-name", "end-time", "entity-pause-reason", "entity-status", "exchange", "guaranteed-order-id", "inventory-source-id", "inventory-source-product-type", "inventory-source-rate-type", "inventory-source-type", "minimum-spend", "name", "nanos", "partner", "partner-id", "publisher-name", "rate", "rate-details", "read-advertiser-ids", "read-partner-ids", "read-write-accessors", "seller-pause-reason", "seller-status", "start-time", "status", "sub-site-property-id", "time-range", "units", "units-purchased", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InventorySource = json::value::from_value(object).unwrap();
        let mut call = self.hub.inventory_sources().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_sources_edit_inventory_source_read_write_accessors(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertisers-update.added-advertisers" => Some(("advertisersUpdate.addedAdvertisers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "advertisers-update.removed-advertisers" => Some(("advertisersUpdate.removedAdvertisers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "assign-partner" => Some(("assignPartner", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["added-advertisers", "advertisers-update", "assign-partner", "partner-id", "removed-advertisers"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EditInventorySourceReadWriteAccessorsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.inventory_sources().edit_inventory_source_read_write_accessors(request, opt.value_of("inventory-source-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_sources_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.inventory_sources().get(opt.value_of("inventory-source-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_sources_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.inventory_sources().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token", "partner-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _inventory_sources_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "commitment" => Some(("commitment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deal-id" => Some(("dealId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delivery-method" => Some(("deliveryMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exchange" => Some(("exchange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "guaranteed-order-id" => Some(("guaranteedOrderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-id" => Some(("inventorySourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-product-type" => Some(("inventorySourceProductType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-type" => Some(("inventorySourceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publisher-name" => Some(("publisherName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.inventory-source-rate-type" => Some(("rateDetails.inventorySourceRateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.minimum-spend.currency-code" => Some(("rateDetails.minimumSpend.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.minimum-spend.nanos" => Some(("rateDetails.minimumSpend.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "rate-details.minimum-spend.units" => Some(("rateDetails.minimumSpend.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.rate.currency-code" => Some(("rateDetails.rate.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.rate.nanos" => Some(("rateDetails.rate.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "rate-details.rate.units" => Some(("rateDetails.rate.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rate-details.units-purchased" => Some(("rateDetails.unitsPurchased", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-advertiser-ids" => Some(("readAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "read-partner-ids" => Some(("readPartnerIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "read-write-accessors.advertisers.advertiser-ids" => Some(("readWriteAccessors.advertisers.advertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "read-write-accessors.partner.partner-id" => Some(("readWriteAccessors.partner.partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.config-status" => Some(("status.configStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.entity-pause-reason" => Some(("status.entityPauseReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.entity-status" => Some(("status.entityStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.seller-pause-reason" => Some(("status.sellerPauseReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.seller-status" => Some(("status.sellerStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sub-site-property-id" => Some(("subSitePropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.end-time" => Some(("timeRange.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.start-time" => Some(("timeRange.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-ids", "advertisers", "commitment", "config-status", "currency-code", "deal-id", "delivery-method", "display-name", "end-time", "entity-pause-reason", "entity-status", "exchange", "guaranteed-order-id", "inventory-source-id", "inventory-source-product-type", "inventory-source-rate-type", "inventory-source-type", "minimum-spend", "name", "nanos", "partner", "partner-id", "publisher-name", "rate", "rate-details", "read-advertiser-ids", "read-partner-ids", "read-write-accessors", "seller-pause-reason", "seller-status", "start-time", "status", "sub-site-property-id", "time-range", "units", "units-purchased", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InventorySource = json::value::from_value(object).unwrap();
        let mut call = self.hub.inventory_sources().patch(request, opt.value_of("inventory-source-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "partner-id" => {
                    call = call.partner_id(        value.map(|v| arg_from_str(v, err, "partner-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "partner-id", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _media_download(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.media().download(opt.value_of("resource-name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    } else {
                    let bytes = hyper::body::to_bytes(response.into_body()).await.expect("a string as API currently is inefficient").to_vec();
                    ostream.write_all(&bytes).expect("write to be complete");
                    ostream.flush().expect("io to never fail which should really be fixed one day");
                    }
                    Ok(())
                }
            }
        }
    }

    async fn _media_upload(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "resource-name" => Some(("resourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["resource-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleBytestreamMedia = json::value::from_value(object).unwrap();
        let mut call = self.hub.media().upload(request, opt.value_of("resource-name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_bulk_edit_partner_assigned_targeting_options(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditPartnerAssignedTargetingOptionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.partners().bulk_edit_partner_assigned_targeting_options(request, opt.value_of("partner-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-id" => Some(("channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negatively-targeted-line-item-count" => Some(("negativelyTargetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "positively-targeted-line-item-count" => Some(("positivelyTargetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "channel-id", "display-name", "name", "negatively-targeted-line-item-count", "partner-id", "positively-targeted-line-item-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.partners().channels_create(request, opt.value_of("partner-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().channels_get(opt.value_of("partner-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().channels_list(opt.value_of("partner-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-id" => Some(("channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negatively-targeted-line-item-count" => Some(("negativelyTargetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "positively-targeted-line-item-count" => Some(("positivelyTargetedLineItemCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "channel-id", "display-name", "name", "negatively-targeted-line-item-count", "partner-id", "positively-targeted-line-item-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Channel = json::value::from_value(object).unwrap();
        let mut call = self.hub.partners().channels_patch(request, opt.value_of("partner-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_sites_bulk_edit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted-sites" => Some(("deletedSites", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "deleted-sites", "partner-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditSitesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.partners().channels_sites_bulk_edit(request, opt.value_of("partner-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_sites_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-or-app-id" => Some(("urlOrAppId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["name", "url-or-app-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Site = json::value::from_value(object).unwrap();
        let mut call = self.hub.partners().channels_sites_create(request, opt.value_of("partner-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_sites_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().channels_sites_delete(opt.value_of("partner-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""), opt.value_of("url-or-app-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_sites_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().channels_sites_list(opt.value_of("partner-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_channels_sites_replace(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "partner-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReplaceSitesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.partners().channels_sites_replace(request, opt.value_of("partner-id").unwrap_or(""), opt.value_of("channel-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().get(opt.value_of("partner-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_targeting_types_assigned_targeting_options_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "age-range-details.age-range" => Some(("ageRangeDetails.ageRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "age-range-details.targeting-option-id" => Some(("ageRangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-category-details.display-name" => Some(("appCategoryDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-category-details.negative" => Some(("appCategoryDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "app-category-details.targeting-option-id" => Some(("appCategoryDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.app-id" => Some(("appDetails.appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.app-platform" => Some(("appDetails.appPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.display-name" => Some(("appDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "app-details.negative" => Some(("appDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "assigned-targeting-option-id" => Some(("assignedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-content-type-details.audio-content-type" => Some(("audioContentTypeDetails.audioContentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio-content-type-details.targeting-option-id" => Some(("audioContentTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorized-seller-status-details.authorized-seller-status" => Some(("authorizedSellerStatusDetails.authorizedSellerStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorized-seller-status-details.targeting-option-id" => Some(("authorizedSellerStatusDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "browser-details.display-name" => Some(("browserDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "browser-details.negative" => Some(("browserDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "browser-details.targeting-option-id" => Some(("browserDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.display-name" => Some(("businessChainDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.proximity-radius-amount" => Some(("businessChainDetails.proximityRadiusAmount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "business-chain-details.proximity-radius-unit" => Some(("businessChainDetails.proximityRadiusUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-details.targeting-option-id" => Some(("businessChainDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.display-name" => Some(("carrierAndIspDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.negative" => Some(("carrierAndIspDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "carrier-and-isp-details.targeting-option-id" => Some(("carrierAndIspDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "category-details.display-name" => Some(("categoryDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "category-details.negative" => Some(("categoryDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "category-details.targeting-option-id" => Some(("categoryDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-details.channel-id" => Some(("channelDetails.channelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "channel-details.negative" => Some(("channelDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-duration-details.content-duration" => Some(("contentDurationDetails.contentDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-duration-details.targeting-option-id" => Some(("contentDurationDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-genre-details.display-name" => Some(("contentGenreDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-genre-details.negative" => Some(("contentGenreDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "content-genre-details.targeting-option-id" => Some(("contentGenreDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.ad-type" => Some(("contentInstreamPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.content-instream-position" => Some(("contentInstreamPositionDetails.contentInstreamPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-instream-position-details.targeting-option-id" => Some(("contentInstreamPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.ad-type" => Some(("contentOutstreamPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.content-outstream-position" => Some(("contentOutstreamPositionDetails.contentOutstreamPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-outstream-position-details.targeting-option-id" => Some(("contentOutstreamPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-stream-type-details.content-stream-type" => Some(("contentStreamTypeDetails.contentStreamType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "content-stream-type-details.targeting-option-id" => Some(("contentStreamTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "day-and-time-details.day-of-week" => Some(("dayAndTimeDetails.dayOfWeek", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "day-and-time-details.end-hour" => Some(("dayAndTimeDetails.endHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "day-and-time-details.start-hour" => Some(("dayAndTimeDetails.startHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "day-and-time-details.time-zone-resolution" => Some(("dayAndTimeDetails.timeZoneResolution", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-make-model-details.display-name" => Some(("deviceMakeModelDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-make-model-details.negative" => Some(("deviceMakeModelDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-make-model-details.targeting-option-id" => Some(("deviceMakeModelDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-type-details.device-type" => Some(("deviceTypeDetails.deviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-type-details.targeting-option-id" => Some(("deviceTypeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "digital-content-label-exclusion-details.content-rating-tier" => Some(("digitalContentLabelExclusionDetails.contentRatingTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "digital-content-label-exclusion-details.excluded-targeting-option-id" => Some(("digitalContentLabelExclusionDetails.excludedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-details.environment" => Some(("environmentDetails.environment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "environment-details.targeting-option-id" => Some(("environmentDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exchange-details.targeting-option-id" => Some(("exchangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gender-details.gender" => Some(("genderDetails.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gender-details.targeting-option-id" => Some(("genderDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.display-name" => Some(("geoRegionDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.geo-region-type" => Some(("geoRegionDetails.geoRegionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-details.negative" => Some(("geoRegionDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "geo-region-details.targeting-option-id" => Some(("geoRegionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "household-income-details.household-income" => Some(("householdIncomeDetails.householdIncome", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "household-income-details.targeting-option-id" => Some(("householdIncomeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inheritance" => Some(("inheritance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-details.inventory-source-id" => Some(("inventorySourceDetails.inventorySourceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inventory-source-group-details.inventory-source-group-id" => Some(("inventorySourceGroupDetails.inventorySourceGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keyword-details.keyword" => Some(("keywordDetails.keyword", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keyword-details.negative" => Some(("keywordDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "language-details.display-name" => Some(("languageDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-details.negative" => Some(("languageDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "language-details.targeting-option-id" => Some(("languageDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-content-position-details.content-position" => Some(("nativeContentPositionDetails.contentPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-content-position-details.targeting-option-id" => Some(("nativeContentPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negative-keyword-list-details.negative-keyword-list-id" => Some(("negativeKeywordListDetails.negativeKeywordListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "omid-details.omid" => Some(("omidDetails.omid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "omid-details.targeting-option-id" => Some(("omidDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.ad-type" => Some(("onScreenPositionDetails.adType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.on-screen-position" => Some(("onScreenPositionDetails.onScreenPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-screen-position-details.targeting-option-id" => Some(("onScreenPositionDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operating-system-details.display-name" => Some(("operatingSystemDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operating-system-details.negative" => Some(("operatingSystemDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "operating-system-details.targeting-option-id" => Some(("operatingSystemDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parental-status-details.parental-status" => Some(("parentalStatusDetails.parentalStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parental-status-details.targeting-option-id" => Some(("parentalStatusDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.display-name" => Some(("poiDetails.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.latitude" => Some(("poiDetails.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.longitude" => Some(("poiDetails.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.proximity-radius-amount" => Some(("poiDetails.proximityRadiusAmount", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "poi-details.proximity-radius-unit" => Some(("poiDetails.proximityRadiusUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-details.targeting-option-id" => Some(("poiDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proximity-location-list-details.proximity-location-list-id" => Some(("proximityLocationListDetails.proximityLocationListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proximity-location-list-details.proximity-radius-range" => Some(("proximityLocationListDetails.proximityRadiusRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "regional-location-list-details.negative" => Some(("regionalLocationListDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "regional-location-list-details.regional-location-list-id" => Some(("regionalLocationListDetails.regionalLocationListId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sensitive-category-exclusion-details.excluded-targeting-option-id" => Some(("sensitiveCategoryExclusionDetails.excludedTargetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sensitive-category-exclusion-details.sensitive-category" => Some(("sensitiveCategoryExclusionDetails.sensitiveCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sub-exchange-details.targeting-option-id" => Some(("subExchangeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "targeting-type" => Some(("targetingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.adloox.excluded-adloox-categories" => Some(("thirdPartyVerifierDetails.adloox.excludedAdlooxCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.app-star-rating.avoid-insufficient-star-rating" => Some(("thirdPartyVerifierDetails.doubleVerify.appStarRating.avoidInsufficientStarRating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.app-star-rating.avoided-star-rating" => Some(("thirdPartyVerifierDetails.doubleVerify.appStarRating.avoidedStarRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.avoided-age-ratings" => Some(("thirdPartyVerifierDetails.doubleVerify.avoidedAgeRatings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoid-unknown-brand-safety-category" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidUnknownBrandSafetyCategory", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoided-high-severity-categories" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidedHighSeverityCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.brand-safety-categories.avoided-medium-severity-categories" => Some(("thirdPartyVerifierDetails.doubleVerify.brandSafetyCategories.avoidedMediumSeverityCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.double-verify.custom-segment-id" => Some(("thirdPartyVerifierDetails.doubleVerify.customSegmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.display-viewability.iab" => Some(("thirdPartyVerifierDetails.doubleVerify.displayViewability.iab", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.display-viewability.viewable-during" => Some(("thirdPartyVerifierDetails.doubleVerify.displayViewability.viewableDuring", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.fraud-invalid-traffic.avoid-insufficient-option" => Some(("thirdPartyVerifierDetails.doubleVerify.fraudInvalidTraffic.avoidInsufficientOption", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.fraud-invalid-traffic.avoided-fraud-option" => Some(("thirdPartyVerifierDetails.doubleVerify.fraudInvalidTraffic.avoidedFraudOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.player-impression-rate" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.playerImpressionRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.video-iab" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.videoIab", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.double-verify.video-viewability.video-viewable-rate" => Some(("thirdPartyVerifierDetails.doubleVerify.videoViewability.videoViewableRate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.custom-segment-id" => Some(("thirdPartyVerifierDetails.integralAdScience.customSegmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "third-party-verifier-details.integral-ad-science.display-viewability" => Some(("thirdPartyVerifierDetails.integralAdScience.displayViewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.exclude-unrateable" => Some(("thirdPartyVerifierDetails.integralAdScience.excludeUnrateable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-ad-fraud-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAdFraudRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-adult-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAdultRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-alcohol-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedAlcoholRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-drugs-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedDrugsRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-gambling-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedGamblingRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-hate-speech-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedHateSpeechRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-illegal-downloads-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedIllegalDownloadsRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-offensive-language-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedOffensiveLanguageRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.excluded-violence-risk" => Some(("thirdPartyVerifierDetails.integralAdScience.excludedViolenceRisk", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.traq-score-option" => Some(("thirdPartyVerifierDetails.integralAdScience.traqScoreOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "third-party-verifier-details.integral-ad-science.video-viewability" => Some(("thirdPartyVerifierDetails.integralAdScience.videoViewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-details.negative" => Some(("urlDetails.negative", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-details.url" => Some(("urlDetails.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-rewarded-content-details.targeting-option-id" => Some(("userRewardedContentDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-rewarded-content-details.user-rewarded-content" => Some(("userRewardedContentDetails.userRewardedContent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-player-size-details.targeting-option-id" => Some(("videoPlayerSizeDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-player-size-details.video-player-size" => Some(("videoPlayerSizeDetails.videoPlayerSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewability-details.targeting-option-id" => Some(("viewabilityDetails.targetingOptionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "viewability-details.viewability" => Some(("viewabilityDetails.viewability", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-type", "adloox", "age-range", "age-range-details", "app-category-details", "app-details", "app-id", "app-platform", "app-star-rating", "assigned-targeting-option-id", "audio-content-type", "audio-content-type-details", "authorized-seller-status", "authorized-seller-status-details", "avoid-insufficient-option", "avoid-insufficient-star-rating", "avoid-unknown-brand-safety-category", "avoided-age-ratings", "avoided-fraud-option", "avoided-high-severity-categories", "avoided-medium-severity-categories", "avoided-star-rating", "brand-safety-categories", "browser-details", "business-chain-details", "carrier-and-isp-details", "category-details", "channel-details", "channel-id", "content-duration", "content-duration-details", "content-genre-details", "content-instream-position", "content-instream-position-details", "content-outstream-position", "content-outstream-position-details", "content-position", "content-rating-tier", "content-stream-type", "content-stream-type-details", "custom-segment-id", "day-and-time-details", "day-of-week", "device-make-model-details", "device-type", "device-type-details", "digital-content-label-exclusion-details", "display-name", "display-viewability", "double-verify", "end-hour", "environment", "environment-details", "exchange-details", "exclude-unrateable", "excluded-ad-fraud-risk", "excluded-adloox-categories", "excluded-adult-risk", "excluded-alcohol-risk", "excluded-drugs-risk", "excluded-gambling-risk", "excluded-hate-speech-risk", "excluded-illegal-downloads-risk", "excluded-offensive-language-risk", "excluded-targeting-option-id", "excluded-violence-risk", "fraud-invalid-traffic", "gender", "gender-details", "geo-region-details", "geo-region-type", "household-income", "household-income-details", "iab", "inheritance", "integral-ad-science", "inventory-source-details", "inventory-source-group-details", "inventory-source-group-id", "inventory-source-id", "keyword", "keyword-details", "language-details", "latitude", "longitude", "name", "native-content-position-details", "negative", "negative-keyword-list-details", "negative-keyword-list-id", "omid", "omid-details", "on-screen-position", "on-screen-position-details", "operating-system-details", "parental-status", "parental-status-details", "player-impression-rate", "poi-details", "proximity-location-list-details", "proximity-location-list-id", "proximity-radius-amount", "proximity-radius-range", "proximity-radius-unit", "regional-location-list-details", "regional-location-list-id", "sensitive-category", "sensitive-category-exclusion-details", "start-hour", "sub-exchange-details", "targeting-option-id", "targeting-type", "third-party-verifier-details", "time-zone-resolution", "traq-score-option", "url", "url-details", "user-rewarded-content", "user-rewarded-content-details", "video-iab", "video-player-size", "video-player-size-details", "video-viewability", "video-viewable-rate", "viewability", "viewability-details", "viewable-during"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AssignedTargetingOption = json::value::from_value(object).unwrap();
        let mut call = self.hub.partners().targeting_types_assigned_targeting_options_create(request, opt.value_of("partner-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_targeting_types_assigned_targeting_options_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().targeting_types_assigned_targeting_options_delete(opt.value_of("partner-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""), opt.value_of("assigned-targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_targeting_types_assigned_targeting_options_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().targeting_types_assigned_targeting_options_get(opt.value_of("partner-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""), opt.value_of("assigned-targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _partners_targeting_types_assigned_targeting_options_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.partners().targeting_types_assigned_targeting_options_list(opt.value_of("partner-id").unwrap_or(""), opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _sdfdownloadtasks_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id-filter.ad-group-ad-ids" => Some(("idFilter.adGroupAdIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "id-filter.ad-group-ids" => Some(("idFilter.adGroupIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "id-filter.campaign-ids" => Some(("idFilter.campaignIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "id-filter.insertion-order-ids" => Some(("idFilter.insertionOrderIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "id-filter.line-item-ids" => Some(("idFilter.lineItemIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "id-filter.media-product-ids" => Some(("idFilter.mediaProductIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inventory-source-filter.inventory-source-ids" => Some(("inventorySourceFilter.inventorySourceIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "parent-entity-filter.file-type" => Some(("parentEntityFilter.fileType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "parent-entity-filter.filter-ids" => Some(("parentEntityFilter.filterIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "parent-entity-filter.filter-type" => Some(("parentEntityFilter.filterType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partner-id" => Some(("partnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-group-ad-ids", "ad-group-ids", "advertiser-id", "campaign-ids", "file-type", "filter-ids", "filter-type", "id-filter", "insertion-order-ids", "inventory-source-filter", "inventory-source-ids", "line-item-ids", "media-product-ids", "parent-entity-filter", "partner-id", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateSdfDownloadTaskRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.sdfdownloadtasks().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _sdfdownloadtasks_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.sdfdownloadtasks().operations_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _targeting_types_targeting_options_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.targeting_types().targeting_options_get(opt.value_of("targeting-type").unwrap_or(""), opt.value_of("targeting-option-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _targeting_types_targeting_options_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.targeting_types().targeting_options_list(opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["advertiser-id", "filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _targeting_types_targeting_options_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-search-terms.business-chain-query" => Some(("businessChainSearchTerms.businessChainQuery", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "business-chain-search-terms.region-query" => Some(("businessChainSearchTerms.regionQuery", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "geo-region-search-terms.geo-region-query" => Some(("geoRegionSearchTerms.geoRegionQuery", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "poi-search-terms.poi-query" => Some(("poiSearchTerms.poiQuery", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advertiser-id", "business-chain-query", "business-chain-search-terms", "geo-region-query", "geo-region-search-terms", "page-size", "page-token", "poi-query", "poi-search-terms", "region-query"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SearchTargetingOptionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.targeting_types().targeting_options_search(request, opt.value_of("targeting-type").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _users_bulk_edit_assigned_user_roles(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "deleted-assigned-user-roles" => Some(("deletedAssignedUserRoles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["deleted-assigned-user-roles"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BulkEditAssignedUserRolesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.users().bulk_edit_assigned_user_roles(request, opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _users_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-id" => Some(("userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "email", "name", "user-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::User = json::value::from_value(object).unwrap();
        let mut call = self.hub.users().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _users_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().delete(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _users_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().get(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _users_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _users_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email" => Some(("email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-id" => Some(("userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "email", "name", "user-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::User = json::value::from_value(object).unwrap();
        let mut call = self.hub.users().patch(request, opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("advertisers", Some(opt)) => {
                match opt.subcommand() {
                    ("assets-upload", Some(opt)) => {
                        call_result = self._advertisers_assets_upload(opt, dry_run, &mut err).await;
                    },
                    ("audit", Some(opt)) => {
                        call_result = self._advertisers_audit(opt, dry_run, &mut err).await;
                    },
                    ("bulk-edit-advertiser-assigned-targeting-options", Some(opt)) => {
                        call_result = self._advertisers_bulk_edit_advertiser_assigned_targeting_options(opt, dry_run, &mut err).await;
                    },
                    ("bulk-list-advertiser-assigned-targeting-options", Some(opt)) => {
                        call_result = self._advertisers_bulk_list_advertiser_assigned_targeting_options(opt, dry_run, &mut err).await;
                    },
                    ("campaigns-bulk-list-campaign-assigned-targeting-options", Some(opt)) => {
                        call_result = self._advertisers_campaigns_bulk_list_campaign_assigned_targeting_options(opt, dry_run, &mut err).await;
                    },
                    ("campaigns-create", Some(opt)) => {
                        call_result = self._advertisers_campaigns_create(opt, dry_run, &mut err).await;
                    },
                    ("campaigns-delete", Some(opt)) => {
                        call_result = self._advertisers_campaigns_delete(opt, dry_run, &mut err).await;
                    },
                    ("campaigns-get", Some(opt)) => {
                        call_result = self._advertisers_campaigns_get(opt, dry_run, &mut err).await;
                    },
                    ("campaigns-list", Some(opt)) => {
                        call_result = self._advertisers_campaigns_list(opt, dry_run, &mut err).await;
                    },
                    ("campaigns-patch", Some(opt)) => {
                        call_result = self._advertisers_campaigns_patch(opt, dry_run, &mut err).await;
                    },
                    ("campaigns-targeting-types-assigned-targeting-options-get", Some(opt)) => {
                        call_result = self._advertisers_campaigns_targeting_types_assigned_targeting_options_get(opt, dry_run, &mut err).await;
                    },
                    ("campaigns-targeting-types-assigned-targeting-options-list", Some(opt)) => {
                        call_result = self._advertisers_campaigns_targeting_types_assigned_targeting_options_list(opt, dry_run, &mut err).await;
                    },
                    ("channels-create", Some(opt)) => {
                        call_result = self._advertisers_channels_create(opt, dry_run, &mut err).await;
                    },
                    ("channels-get", Some(opt)) => {
                        call_result = self._advertisers_channels_get(opt, dry_run, &mut err).await;
                    },
                    ("channels-list", Some(opt)) => {
                        call_result = self._advertisers_channels_list(opt, dry_run, &mut err).await;
                    },
                    ("channels-patch", Some(opt)) => {
                        call_result = self._advertisers_channels_patch(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-bulk-edit", Some(opt)) => {
                        call_result = self._advertisers_channels_sites_bulk_edit(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-create", Some(opt)) => {
                        call_result = self._advertisers_channels_sites_create(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-delete", Some(opt)) => {
                        call_result = self._advertisers_channels_sites_delete(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-list", Some(opt)) => {
                        call_result = self._advertisers_channels_sites_list(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-replace", Some(opt)) => {
                        call_result = self._advertisers_channels_sites_replace(opt, dry_run, &mut err).await;
                    },
                    ("create", Some(opt)) => {
                        call_result = self._advertisers_create(opt, dry_run, &mut err).await;
                    },
                    ("creatives-create", Some(opt)) => {
                        call_result = self._advertisers_creatives_create(opt, dry_run, &mut err).await;
                    },
                    ("creatives-delete", Some(opt)) => {
                        call_result = self._advertisers_creatives_delete(opt, dry_run, &mut err).await;
                    },
                    ("creatives-get", Some(opt)) => {
                        call_result = self._advertisers_creatives_get(opt, dry_run, &mut err).await;
                    },
                    ("creatives-list", Some(opt)) => {
                        call_result = self._advertisers_creatives_list(opt, dry_run, &mut err).await;
                    },
                    ("creatives-patch", Some(opt)) => {
                        call_result = self._advertisers_creatives_patch(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._advertisers_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._advertisers_get(opt, dry_run, &mut err).await;
                    },
                    ("insertion-orders-bulk-list-insertion-order-assigned-targeting-options", Some(opt)) => {
                        call_result = self._advertisers_insertion_orders_bulk_list_insertion_order_assigned_targeting_options(opt, dry_run, &mut err).await;
                    },
                    ("insertion-orders-create", Some(opt)) => {
                        call_result = self._advertisers_insertion_orders_create(opt, dry_run, &mut err).await;
                    },
                    ("insertion-orders-delete", Some(opt)) => {
                        call_result = self._advertisers_insertion_orders_delete(opt, dry_run, &mut err).await;
                    },
                    ("insertion-orders-get", Some(opt)) => {
                        call_result = self._advertisers_insertion_orders_get(opt, dry_run, &mut err).await;
                    },
                    ("insertion-orders-list", Some(opt)) => {
                        call_result = self._advertisers_insertion_orders_list(opt, dry_run, &mut err).await;
                    },
                    ("insertion-orders-patch", Some(opt)) => {
                        call_result = self._advertisers_insertion_orders_patch(opt, dry_run, &mut err).await;
                    },
                    ("insertion-orders-targeting-types-assigned-targeting-options-get", Some(opt)) => {
                        call_result = self._advertisers_insertion_orders_targeting_types_assigned_targeting_options_get(opt, dry_run, &mut err).await;
                    },
                    ("insertion-orders-targeting-types-assigned-targeting-options-list", Some(opt)) => {
                        call_result = self._advertisers_insertion_orders_targeting_types_assigned_targeting_options_list(opt, dry_run, &mut err).await;
                    },
                    ("invoices-list", Some(opt)) => {
                        call_result = self._advertisers_invoices_list(opt, dry_run, &mut err).await;
                    },
                    ("invoices-lookup-invoice-currency", Some(opt)) => {
                        call_result = self._advertisers_invoices_lookup_invoice_currency(opt, dry_run, &mut err).await;
                    },
                    ("line-items-bulk-edit-line-item-assigned-targeting-options", Some(opt)) => {
                        call_result = self._advertisers_line_items_bulk_edit_line_item_assigned_targeting_options(opt, dry_run, &mut err).await;
                    },
                    ("line-items-bulk-list-line-item-assigned-targeting-options", Some(opt)) => {
                        call_result = self._advertisers_line_items_bulk_list_line_item_assigned_targeting_options(opt, dry_run, &mut err).await;
                    },
                    ("line-items-create", Some(opt)) => {
                        call_result = self._advertisers_line_items_create(opt, dry_run, &mut err).await;
                    },
                    ("line-items-delete", Some(opt)) => {
                        call_result = self._advertisers_line_items_delete(opt, dry_run, &mut err).await;
                    },
                    ("line-items-generate-default", Some(opt)) => {
                        call_result = self._advertisers_line_items_generate_default(opt, dry_run, &mut err).await;
                    },
                    ("line-items-get", Some(opt)) => {
                        call_result = self._advertisers_line_items_get(opt, dry_run, &mut err).await;
                    },
                    ("line-items-list", Some(opt)) => {
                        call_result = self._advertisers_line_items_list(opt, dry_run, &mut err).await;
                    },
                    ("line-items-patch", Some(opt)) => {
                        call_result = self._advertisers_line_items_patch(opt, dry_run, &mut err).await;
                    },
                    ("line-items-targeting-types-assigned-targeting-options-create", Some(opt)) => {
                        call_result = self._advertisers_line_items_targeting_types_assigned_targeting_options_create(opt, dry_run, &mut err).await;
                    },
                    ("line-items-targeting-types-assigned-targeting-options-delete", Some(opt)) => {
                        call_result = self._advertisers_line_items_targeting_types_assigned_targeting_options_delete(opt, dry_run, &mut err).await;
                    },
                    ("line-items-targeting-types-assigned-targeting-options-get", Some(opt)) => {
                        call_result = self._advertisers_line_items_targeting_types_assigned_targeting_options_get(opt, dry_run, &mut err).await;
                    },
                    ("line-items-targeting-types-assigned-targeting-options-list", Some(opt)) => {
                        call_result = self._advertisers_line_items_targeting_types_assigned_targeting_options_list(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._advertisers_list(opt, dry_run, &mut err).await;
                    },
                    ("location-lists-assigned-locations-bulk-edit", Some(opt)) => {
                        call_result = self._advertisers_location_lists_assigned_locations_bulk_edit(opt, dry_run, &mut err).await;
                    },
                    ("location-lists-assigned-locations-create", Some(opt)) => {
                        call_result = self._advertisers_location_lists_assigned_locations_create(opt, dry_run, &mut err).await;
                    },
                    ("location-lists-assigned-locations-delete", Some(opt)) => {
                        call_result = self._advertisers_location_lists_assigned_locations_delete(opt, dry_run, &mut err).await;
                    },
                    ("location-lists-assigned-locations-list", Some(opt)) => {
                        call_result = self._advertisers_location_lists_assigned_locations_list(opt, dry_run, &mut err).await;
                    },
                    ("location-lists-create", Some(opt)) => {
                        call_result = self._advertisers_location_lists_create(opt, dry_run, &mut err).await;
                    },
                    ("location-lists-get", Some(opt)) => {
                        call_result = self._advertisers_location_lists_get(opt, dry_run, &mut err).await;
                    },
                    ("location-lists-list", Some(opt)) => {
                        call_result = self._advertisers_location_lists_list(opt, dry_run, &mut err).await;
                    },
                    ("location-lists-patch", Some(opt)) => {
                        call_result = self._advertisers_location_lists_patch(opt, dry_run, &mut err).await;
                    },
                    ("manual-triggers-activate", Some(opt)) => {
                        call_result = self._advertisers_manual_triggers_activate(opt, dry_run, &mut err).await;
                    },
                    ("manual-triggers-create", Some(opt)) => {
                        call_result = self._advertisers_manual_triggers_create(opt, dry_run, &mut err).await;
                    },
                    ("manual-triggers-deactivate", Some(opt)) => {
                        call_result = self._advertisers_manual_triggers_deactivate(opt, dry_run, &mut err).await;
                    },
                    ("manual-triggers-get", Some(opt)) => {
                        call_result = self._advertisers_manual_triggers_get(opt, dry_run, &mut err).await;
                    },
                    ("manual-triggers-list", Some(opt)) => {
                        call_result = self._advertisers_manual_triggers_list(opt, dry_run, &mut err).await;
                    },
                    ("manual-triggers-patch", Some(opt)) => {
                        call_result = self._advertisers_manual_triggers_patch(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-create", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_create(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-delete", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_delete(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-get", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_get(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-list", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_list(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-negative-keywords-bulk-edit", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_negative_keywords_bulk_edit(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-negative-keywords-create", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_negative_keywords_create(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-negative-keywords-delete", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_negative_keywords_delete(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-negative-keywords-list", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_negative_keywords_list(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-negative-keywords-replace", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_negative_keywords_replace(opt, dry_run, &mut err).await;
                    },
                    ("negative-keyword-lists-patch", Some(opt)) => {
                        call_result = self._advertisers_negative_keyword_lists_patch(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._advertisers_patch(opt, dry_run, &mut err).await;
                    },
                    ("targeting-types-assigned-targeting-options-create", Some(opt)) => {
                        call_result = self._advertisers_targeting_types_assigned_targeting_options_create(opt, dry_run, &mut err).await;
                    },
                    ("targeting-types-assigned-targeting-options-delete", Some(opt)) => {
                        call_result = self._advertisers_targeting_types_assigned_targeting_options_delete(opt, dry_run, &mut err).await;
                    },
                    ("targeting-types-assigned-targeting-options-get", Some(opt)) => {
                        call_result = self._advertisers_targeting_types_assigned_targeting_options_get(opt, dry_run, &mut err).await;
                    },
                    ("targeting-types-assigned-targeting-options-list", Some(opt)) => {
                        call_result = self._advertisers_targeting_types_assigned_targeting_options_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("advertisers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("combined-audiences", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._combined_audiences_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._combined_audiences_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("combined-audiences".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("custom-bidding-algorithms", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._custom_bidding_algorithms_create(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._custom_bidding_algorithms_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._custom_bidding_algorithms_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._custom_bidding_algorithms_patch(opt, dry_run, &mut err).await;
                    },
                    ("scripts-create", Some(opt)) => {
                        call_result = self._custom_bidding_algorithms_scripts_create(opt, dry_run, &mut err).await;
                    },
                    ("scripts-get", Some(opt)) => {
                        call_result = self._custom_bidding_algorithms_scripts_get(opt, dry_run, &mut err).await;
                    },
                    ("scripts-list", Some(opt)) => {
                        call_result = self._custom_bidding_algorithms_scripts_list(opt, dry_run, &mut err).await;
                    },
                    ("upload-script", Some(opt)) => {
                        call_result = self._custom_bidding_algorithms_upload_script(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("custom-bidding-algorithms".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("custom-lists", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._custom_lists_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._custom_lists_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("custom-lists".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("first-and-third-party-audiences", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._first_and_third_party_audiences_create(opt, dry_run, &mut err).await;
                    },
                    ("edit-customer-match-members", Some(opt)) => {
                        call_result = self._first_and_third_party_audiences_edit_customer_match_members(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._first_and_third_party_audiences_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._first_and_third_party_audiences_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._first_and_third_party_audiences_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("first-and-third-party-audiences".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("floodlight-groups", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._floodlight_groups_get(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._floodlight_groups_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("floodlight-groups".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("google-audiences", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._google_audiences_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._google_audiences_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("google-audiences".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("guaranteed-orders", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._guaranteed_orders_create(opt, dry_run, &mut err).await;
                    },
                    ("edit-guaranteed-order-read-accessors", Some(opt)) => {
                        call_result = self._guaranteed_orders_edit_guaranteed_order_read_accessors(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._guaranteed_orders_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._guaranteed_orders_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._guaranteed_orders_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("guaranteed-orders".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("inventory-source-groups", Some(opt)) => {
                match opt.subcommand() {
                    ("assigned-inventory-sources-bulk-edit", Some(opt)) => {
                        call_result = self._inventory_source_groups_assigned_inventory_sources_bulk_edit(opt, dry_run, &mut err).await;
                    },
                    ("assigned-inventory-sources-create", Some(opt)) => {
                        call_result = self._inventory_source_groups_assigned_inventory_sources_create(opt, dry_run, &mut err).await;
                    },
                    ("assigned-inventory-sources-delete", Some(opt)) => {
                        call_result = self._inventory_source_groups_assigned_inventory_sources_delete(opt, dry_run, &mut err).await;
                    },
                    ("assigned-inventory-sources-list", Some(opt)) => {
                        call_result = self._inventory_source_groups_assigned_inventory_sources_list(opt, dry_run, &mut err).await;
                    },
                    ("create", Some(opt)) => {
                        call_result = self._inventory_source_groups_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._inventory_source_groups_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._inventory_source_groups_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._inventory_source_groups_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._inventory_source_groups_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("inventory-source-groups".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("inventory-sources", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._inventory_sources_create(opt, dry_run, &mut err).await;
                    },
                    ("edit-inventory-source-read-write-accessors", Some(opt)) => {
                        call_result = self._inventory_sources_edit_inventory_source_read_write_accessors(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._inventory_sources_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._inventory_sources_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._inventory_sources_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("inventory-sources".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("media", Some(opt)) => {
                match opt.subcommand() {
                    ("download", Some(opt)) => {
                        call_result = self._media_download(opt, dry_run, &mut err).await;
                    },
                    ("upload", Some(opt)) => {
                        call_result = self._media_upload(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("media".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("partners", Some(opt)) => {
                match opt.subcommand() {
                    ("bulk-edit-partner-assigned-targeting-options", Some(opt)) => {
                        call_result = self._partners_bulk_edit_partner_assigned_targeting_options(opt, dry_run, &mut err).await;
                    },
                    ("channels-create", Some(opt)) => {
                        call_result = self._partners_channels_create(opt, dry_run, &mut err).await;
                    },
                    ("channels-get", Some(opt)) => {
                        call_result = self._partners_channels_get(opt, dry_run, &mut err).await;
                    },
                    ("channels-list", Some(opt)) => {
                        call_result = self._partners_channels_list(opt, dry_run, &mut err).await;
                    },
                    ("channels-patch", Some(opt)) => {
                        call_result = self._partners_channels_patch(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-bulk-edit", Some(opt)) => {
                        call_result = self._partners_channels_sites_bulk_edit(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-create", Some(opt)) => {
                        call_result = self._partners_channels_sites_create(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-delete", Some(opt)) => {
                        call_result = self._partners_channels_sites_delete(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-list", Some(opt)) => {
                        call_result = self._partners_channels_sites_list(opt, dry_run, &mut err).await;
                    },
                    ("channels-sites-replace", Some(opt)) => {
                        call_result = self._partners_channels_sites_replace(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._partners_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._partners_list(opt, dry_run, &mut err).await;
                    },
                    ("targeting-types-assigned-targeting-options-create", Some(opt)) => {
                        call_result = self._partners_targeting_types_assigned_targeting_options_create(opt, dry_run, &mut err).await;
                    },
                    ("targeting-types-assigned-targeting-options-delete", Some(opt)) => {
                        call_result = self._partners_targeting_types_assigned_targeting_options_delete(opt, dry_run, &mut err).await;
                    },
                    ("targeting-types-assigned-targeting-options-get", Some(opt)) => {
                        call_result = self._partners_targeting_types_assigned_targeting_options_get(opt, dry_run, &mut err).await;
                    },
                    ("targeting-types-assigned-targeting-options-list", Some(opt)) => {
                        call_result = self._partners_targeting_types_assigned_targeting_options_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("partners".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("sdfdownloadtasks", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._sdfdownloadtasks_create(opt, dry_run, &mut err).await;
                    },
                    ("operations-get", Some(opt)) => {
                        call_result = self._sdfdownloadtasks_operations_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("sdfdownloadtasks".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("targeting-types", Some(opt)) => {
                match opt.subcommand() {
                    ("targeting-options-get", Some(opt)) => {
                        call_result = self._targeting_types_targeting_options_get(opt, dry_run, &mut err).await;
                    },
                    ("targeting-options-list", Some(opt)) => {
                        call_result = self._targeting_types_targeting_options_list(opt, dry_run, &mut err).await;
                    },
                    ("targeting-options-search", Some(opt)) => {
                        call_result = self._targeting_types_targeting_options_search(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("targeting-types".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("users", Some(opt)) => {
                match opt.subcommand() {
                    ("bulk-edit-assigned-user-roles", Some(opt)) => {
                        call_result = self._users_bulk_edit_assigned_user_roles(opt, dry_run, &mut err).await;
                    },
                    ("create", Some(opt)) => {
                        call_result = self._users_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._users_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._users_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._users_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._users_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("users".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    async fn new(opt: ArgMatches<'n>, connector: S) -> Result<Engine<'n, S>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "displayvideo1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/displayvideo1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::DisplayVideo::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
                ]
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    async fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false).await {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("advertisers", "methods: 'assets-upload', 'audit', 'bulk-edit-advertiser-assigned-targeting-options', 'bulk-list-advertiser-assigned-targeting-options', 'campaigns-bulk-list-campaign-assigned-targeting-options', 'campaigns-create', 'campaigns-delete', 'campaigns-get', 'campaigns-list', 'campaigns-patch', 'campaigns-targeting-types-assigned-targeting-options-get', 'campaigns-targeting-types-assigned-targeting-options-list', 'channels-create', 'channels-get', 'channels-list', 'channels-patch', 'channels-sites-bulk-edit', 'channels-sites-create', 'channels-sites-delete', 'channels-sites-list', 'channels-sites-replace', 'create', 'creatives-create', 'creatives-delete', 'creatives-get', 'creatives-list', 'creatives-patch', 'delete', 'get', 'insertion-orders-bulk-list-insertion-order-assigned-targeting-options', 'insertion-orders-create', 'insertion-orders-delete', 'insertion-orders-get', 'insertion-orders-list', 'insertion-orders-patch', 'insertion-orders-targeting-types-assigned-targeting-options-get', 'insertion-orders-targeting-types-assigned-targeting-options-list', 'invoices-list', 'invoices-lookup-invoice-currency', 'line-items-bulk-edit-line-item-assigned-targeting-options', 'line-items-bulk-list-line-item-assigned-targeting-options', 'line-items-create', 'line-items-delete', 'line-items-generate-default', 'line-items-get', 'line-items-list', 'line-items-patch', 'line-items-targeting-types-assigned-targeting-options-create', 'line-items-targeting-types-assigned-targeting-options-delete', 'line-items-targeting-types-assigned-targeting-options-get', 'line-items-targeting-types-assigned-targeting-options-list', 'list', 'location-lists-assigned-locations-bulk-edit', 'location-lists-assigned-locations-create', 'location-lists-assigned-locations-delete', 'location-lists-assigned-locations-list', 'location-lists-create', 'location-lists-get', 'location-lists-list', 'location-lists-patch', 'manual-triggers-activate', 'manual-triggers-create', 'manual-triggers-deactivate', 'manual-triggers-get', 'manual-triggers-list', 'manual-triggers-patch', 'negative-keyword-lists-create', 'negative-keyword-lists-delete', 'negative-keyword-lists-get', 'negative-keyword-lists-list', 'negative-keyword-lists-negative-keywords-bulk-edit', 'negative-keyword-lists-negative-keywords-create', 'negative-keyword-lists-negative-keywords-delete', 'negative-keyword-lists-negative-keywords-list', 'negative-keyword-lists-negative-keywords-replace', 'negative-keyword-lists-patch', 'patch', 'targeting-types-assigned-targeting-options-create', 'targeting-types-assigned-targeting-options-delete', 'targeting-types-assigned-targeting-options-get' and 'targeting-types-assigned-targeting-options-list'", vec![
            ("assets-upload",
                    Some(r##"Uploads an asset. Returns the ID of the newly uploaded asset if successful. The asset file size should be no more than 10 MB for images, 200 MB for ZIP files, and 1 GB for videos. Must be used within the [multipart media upload process](/display-video/api/guides/how-tos/upload#multipart). Examples using provided client libraries can be found in our [Creating Creatives guide](/display-video/api/guides/creating-creatives/overview#upload_an_asset)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_assets-upload",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser this asset belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("audit",
                    Some(r##"Audits an advertiser. Returns the counts of used entities per resource type under the advertiser provided. Used entities count towards their respective resource limit. See https://support.google.com/displayvideo/answer/6071450."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_audit",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser to audit."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("bulk-edit-advertiser-assigned-targeting-options",
                    Some(r##"Bulk edits targeting options under a single advertiser. The operation will delete the assigned targeting options provided in BulkEditAdvertiserAssignedTargetingOptionsRequest.delete_requests and then create the assigned targeting options provided in BulkEditAdvertiserAssignedTargetingOptionsRequest.create_requests ."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_bulk-edit-advertiser-assigned-targeting-options",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("bulk-list-advertiser-assigned-targeting-options",
                    Some(r##"Lists assigned targeting options of an advertiser across targeting types."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_bulk-list-advertiser-assigned-targeting-options",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("campaigns-bulk-list-campaign-assigned-targeting-options",
                    Some(r##"Lists assigned targeting options of a campaign across targeting types."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_campaigns-bulk-list-campaign-assigned-targeting-options",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the campaign belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"campaign-id"##),
                     None,
                     Some(r##"Required. The ID of the campaign to list assigned targeting options for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("campaigns-create",
                    Some(r##"Creates a new campaign. Returns the newly created campaign if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_campaigns-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser the campaign belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("campaigns-delete",
                    Some(r##"Permanently deletes a campaign. A deleted campaign cannot be recovered. The campaign should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_campaigns-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser this campaign belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"campaign-id"##),
                     None,
                     Some(r##"The ID of the campaign we need to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("campaigns-get",
                    Some(r##"Gets a campaign."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_campaigns-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser this campaign belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"campaign-id"##),
                     None,
                     Some(r##"Required. The ID of the campaign to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("campaigns-list",
                    Some(r##"Lists campaigns in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, campaigns with `ENTITY_STATUS_ARCHIVED` will not be included in the results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_campaigns-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser to list campaigns for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("campaigns-patch",
                    Some(r##"Updates an existing campaign. Returns the updated campaign if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_campaigns-patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser the campaign belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"campaign-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the campaign. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("campaigns-targeting-types-assigned-targeting-options-get",
                    Some(r##"Gets a single targeting option assigned to a campaign."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_campaigns-targeting-types-assigned-targeting-options-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the campaign belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"campaign-id"##),
                     None,
                     Some(r##"Required. The ID of the campaign the assigned targeting option belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_AGE_RANGE` * `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS` * `TARGETING_TYPE_CONTENT_INSTREAM_POSITION` * `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_ENVIRONMENT` * `TARGETING_TYPE_EXCHANGE` * `TARGETING_TYPE_GENDER` * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_HOUSEHOLD_INCOME` * `TARGETING_TYPE_INVENTORY_SOURCE` * `TARGETING_TYPE_INVENTORY_SOURCE_GROUP` * `TARGETING_TYPE_LANGUAGE` * `TARGETING_TYPE_ON_SCREEN_POSITION` * `TARGETING_TYPE_PARENTAL_STATUS` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_SUB_EXCHANGE` * `TARGETING_TYPE_THIRD_PARTY_VERIFIER` * `TARGETING_TYPE_VIEWABILITY`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-targeting-option-id"##),
                     None,
                     Some(r##"Required. An identifier unique to the targeting type in this campaign that identifies the assigned targeting option being requested."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("campaigns-targeting-types-assigned-targeting-options-list",
                    Some(r##"Lists the targeting options assigned to a campaign for a specified targeting type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_campaigns-targeting-types-assigned-targeting-options-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the campaign belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"campaign-id"##),
                     None,
                     Some(r##"Required. The ID of the campaign to list assigned targeting options for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of assigned targeting options to list. Supported targeting types: * `TARGETING_TYPE_AGE_RANGE` * `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS` * `TARGETING_TYPE_CONTENT_INSTREAM_POSITION` * `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_ENVIRONMENT` * `TARGETING_TYPE_EXCHANGE` * `TARGETING_TYPE_GENDER` * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_HOUSEHOLD_INCOME` * `TARGETING_TYPE_INVENTORY_SOURCE` * `TARGETING_TYPE_INVENTORY_SOURCE_GROUP` * `TARGETING_TYPE_LANGUAGE` * `TARGETING_TYPE_ON_SCREEN_POSITION` * `TARGETING_TYPE_PARENTAL_STATUS` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_SUB_EXCHANGE` * `TARGETING_TYPE_THIRD_PARTY_VERIFIER` * `TARGETING_TYPE_VIEWABILITY`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-create",
                    Some(r##"Creates a new channel. Returns the newly created channel if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the created channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-get",
                    Some(r##"Gets a channel for a partner or advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the fetched channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the channel to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-list",
                    Some(r##"Lists channels for a partner or advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the channels."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-patch",
                    Some(r##"Updates a channel. Returns the updated channel if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the created channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the channel. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-bulk-edit",
                    Some(r##"Bulk edits sites under a single channel. The operation will delete the sites provided in BulkEditSitesRequest.deleted_sites and then create the sites provided in BulkEditSitesRequest.created_sites."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-sites-bulk-edit",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel to which the sites belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-create",
                    Some(r##"Creates a site in a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-sites-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel in which the site will be created."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-delete",
                    Some(r##"Deletes a site from a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-sites-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel to which the site belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"url-or-app-id"##),
                     None,
                     Some(r##"Required. The URL or app ID of the site to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-list",
                    Some(r##"Lists sites in a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-sites-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel to which the requested sites belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-replace",
                    Some(r##"Replaces all of the sites under a single channel. The operation will replace the sites under a channel with the sites provided in ReplaceSitesRequest.new_sites."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_channels-sites-replace",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel whose sites will be replaced."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("create",
                    Some(r##"Creates a new advertiser. Returns the newly created advertiser if successful. This method can take up to 180 seconds to complete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("creatives-create",
                    Some(r##"Creates a new creative. Returns the newly created creative if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_creatives-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser the creative belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("creatives-delete",
                    Some(r##"Deletes a creative. Returns error code `NOT_FOUND` if the creative does not exist. The creative should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, before it can be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_creatives-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser this creative belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"creative-id"##),
                     None,
                     Some(r##"The ID of the creative to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("creatives-get",
                    Some(r##"Gets a creative."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_creatives-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser this creative belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"creative-id"##),
                     None,
                     Some(r##"Required. The ID of the creative to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("creatives-list",
                    Some(r##"Lists creatives in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, creatives with `ENTITY_STATUS_ARCHIVED` will not be included in the results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_creatives-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser to list creatives for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("creatives-patch",
                    Some(r##"Updates an existing creative. Returns the updated creative if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_creatives-patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser the creative belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"creative-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the creative. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("delete",
                    Some(r##"Deletes an advertiser. Deleting an advertiser will delete all of its child resources, for example, campaigns, insertion orders and line items. A deleted advertiser cannot be recovered."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser we need to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets an advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insertion-orders-bulk-list-insertion-order-assigned-targeting-options",
                    Some(r##"Lists assigned targeting options of an insertion order across targeting types."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_insertion-orders-bulk-list-insertion-order-assigned-targeting-options",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the insertion order belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"insertion-order-id"##),
                     None,
                     Some(r##"Required. The ID of the insertion order to list assigned targeting options for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insertion-orders-create",
                    Some(r##"Creates a new insertion order. Returns the newly created insertion order if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_insertion-orders-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser the insertion order belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insertion-orders-delete",
                    Some(r##"Deletes an insertion order. Returns error code `NOT_FOUND` if the insertion order does not exist. The insertion order should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_insertion-orders-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser this insertion order belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"insertion-order-id"##),
                     None,
                     Some(r##"The ID of the insertion order to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insertion-orders-get",
                    Some(r##"Gets an insertion order. Returns error code `NOT_FOUND` if the insertion order does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_insertion-orders-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser this insertion order belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"insertion-order-id"##),
                     None,
                     Some(r##"Required. The ID of the insertion order to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insertion-orders-list",
                    Some(r##"Lists insertion orders in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, insertion orders with `ENTITY_STATUS_ARCHIVED` will not be included in the results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_insertion-orders-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser to list insertion orders for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insertion-orders-patch",
                    Some(r##"Updates an existing insertion order. Returns the updated insertion order if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_insertion-orders-patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser the insertion order belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"insertion-order-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the insertion order. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insertion-orders-targeting-types-assigned-targeting-options-get",
                    Some(r##"Gets a single targeting option assigned to an insertion order."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_insertion-orders-targeting-types-assigned-targeting-options-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the insertion order belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"insertion-order-id"##),
                     None,
                     Some(r##"Required. The ID of the insertion order the assigned targeting option belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-targeting-option-id"##),
                     None,
                     Some(r##"Required. An identifier unique to the targeting type in this insertion order that identifies the assigned targeting option being requested."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("insertion-orders-targeting-types-assigned-targeting-options-list",
                    Some(r##"Lists the targeting options assigned to an insertion order."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_insertion-orders-targeting-types-assigned-targeting-options-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the insertion order belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"insertion-order-id"##),
                     None,
                     Some(r##"Required. The ID of the insertion order to list assigned targeting options for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of assigned targeting options to list."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("invoices-list",
                    Some(r##"Lists invoices posted for an advertiser in a given month. Invoices generated by billing profiles with a "Partner" invoice level are not retrievable through this method."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_invoices-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser to list invoices for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("invoices-lookup-invoice-currency",
                    Some(r##"Retrieves the invoice currency used by an advertiser in a given month."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_invoices-lookup-invoice-currency",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser to lookup currency for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-bulk-edit-line-item-assigned-targeting-options",
                    Some(r##"Bulk edits targeting options under a single line item. The operation will delete the assigned targeting options provided in BulkEditLineItemAssignedTargetingOptionsRequest.delete_requests and then create the assigned targeting options provided in BulkEditLineItemAssignedTargetingOptionsRequest.create_requests. Requests to this endpoint cannot be made concurrently with the following requests updating the same line item: * UpdateLineItem * CreateLineItemAssignedTargetingOption * DeleteLineItemAssignedTargetingOption"##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-bulk-edit-line-item-assigned-targeting-options",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"Required. The ID of the line item the assigned targeting option will belong to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-bulk-list-line-item-assigned-targeting-options",
                    Some(r##"Lists assigned targeting options of a line item across targeting types."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-bulk-list-line-item-assigned-targeting-options",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"Required. The ID of the line item to list assigned targeting options for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-create",
                    Some(r##"Creates a new line item. Returns the newly created line item if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-delete",
                    Some(r##"Deletes a line item. Returns error code `NOT_FOUND` if the line item does not exist. The line item should be archived first, i.e. set entity_status to `ENTITY_STATUS_ARCHIVED`, to be able to delete it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"The ID of the advertiser this line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"The ID of the line item to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-generate-default",
                    Some(r##"Creates a new line item with settings (including targeting) inherited from the insertion order and an `ENTITY_STATUS_DRAFT` entity_status. Returns the newly created line item if successful. There are default values based on the three fields: * The insertion order's insertion_order_type * The insertion order's automation_type * The given line_item_type"##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-generate-default",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser this line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-get",
                    Some(r##"Gets a line item."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser this line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"Required. The ID of the line item to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-list",
                    Some(r##"Lists line items in an advertiser. The order is defined by the order_by parameter. If a filter by entity_status is not specified, line items with `ENTITY_STATUS_ARCHIVED` will not be included in the results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser to list line items for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-patch",
                    Some(r##"Updates an existing line item. Returns the updated line item if successful. Requests to this endpoint cannot be made concurrently with the following requests updating the same line item: * BulkEditAssignedTargetingOptions * BulkUpdateLineItems * CreateLineItemAssignedTargetingOption * DeleteLineItemAssignedTargetingOption"##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the line item. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-targeting-types-assigned-targeting-options-create",
                    Some(r##"Assigns a targeting option to a line item. Returns the assigned targeting option if successful. Requests to this endpoint cannot be made concurrently with the following requests updating the same line item: * BulkEditAssignedTargetingOptions * BulkUpdate * UpdateLineItem * DeleteLineItemAssignedTargetingOption"##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-targeting-types-assigned-targeting-options-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"Required. The ID of the line item the assigned targeting option will belong to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-targeting-types-assigned-targeting-options-delete",
                    Some(r##"Deletes an assigned targeting option from a line item. Requests to this endpoint cannot be made concurrently with the following requests updating the same line item: * BulkEditAssignedTargetingOptions * BulkUpdate * UpdateLineItem * CreateLineItemAssignedTargetingOption"##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-targeting-types-assigned-targeting-options-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"Required. The ID of the line item the assigned targeting option belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-targeting-option-id"##),
                     None,
                     Some(r##"Required. The ID of the assigned targeting option to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-targeting-types-assigned-targeting-options-get",
                    Some(r##"Gets a single targeting option assigned to a line item."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-targeting-types-assigned-targeting-options-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"Required. The ID of the line item the assigned targeting option belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-targeting-option-id"##),
                     None,
                     Some(r##"Required. An identifier unique to the targeting type in this line item that identifies the assigned targeting option being requested."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("line-items-targeting-types-assigned-targeting-options-list",
                    Some(r##"Lists the targeting options assigned to a line item."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_line-items-targeting-types-assigned-targeting-options-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser the line item belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"line-item-id"##),
                     None,
                     Some(r##"Required. The ID of the line item to list assigned targeting options for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of assigned targeting options to list."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists advertisers that are accessible to the current user. The order is defined by the order_by parameter. A single partner_id is required. Cross-partner listing is not supported."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("location-lists-assigned-locations-bulk-edit",
                    Some(r##"Bulk edits multiple assignments between locations and a single location list. The operation will delete the assigned locations provided in BulkEditAssignedLocationsRequest.deleted_assigned_locations and then create the assigned locations provided in BulkEditAssignedLocationsRequest.created_assigned_locations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_location-lists-assigned-locations-bulk-edit",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the location list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"location-list-id"##),
                     None,
                     Some(r##"Required. The ID of the location list to which these assignments are assigned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("location-lists-assigned-locations-create",
                    Some(r##"Creates an assignment between a location and a location list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_location-lists-assigned-locations-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the location list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"location-list-id"##),
                     None,
                     Some(r##"Required. The ID of the location list for which the assignment will be created."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("location-lists-assigned-locations-delete",
                    Some(r##"Deletes the assignment between a location and a location list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_location-lists-assigned-locations-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the location list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"location-list-id"##),
                     None,
                     Some(r##"Required. The ID of the location list to which this assignment is assigned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-location-id"##),
                     None,
                     Some(r##"Required. The ID of the assigned location to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("location-lists-assigned-locations-list",
                    Some(r##"Lists locations assigned to a location list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_location-lists-assigned-locations-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the location list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"location-list-id"##),
                     None,
                     Some(r##"Required. The ID of the location list to which these assignments are assigned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("location-lists-create",
                    Some(r##"Creates a new location list. Returns the newly created location list if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_location-lists-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the location list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("location-lists-get",
                    Some(r##"Gets a location list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_location-lists-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the fetched location list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"location-list-id"##),
                     None,
                     Some(r##"Required. The ID of the location list to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("location-lists-list",
                    Some(r##"Lists location lists based on a given advertiser id."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_location-lists-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the fetched location lists belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("location-lists-patch",
                    Some(r##"Updates a location list. Returns the updated location list if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_location-lists-patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the location lists belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"location-list-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the location list. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("manual-triggers-activate",
                    Some(r##"Activates a manual trigger. Each activation of the manual trigger must be at least 5 minutes apart, otherwise an error will be returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_manual-triggers-activate",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser that the manual trigger belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"Required. The ID of the manual trigger to activate."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("manual-triggers-create",
                    Some(r##"Creates a new manual trigger. Returns the newly created manual trigger if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_manual-triggers-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. Immutable. The unique ID of the advertiser that the manual trigger belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("manual-triggers-deactivate",
                    Some(r##"Deactivates a manual trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_manual-triggers-deactivate",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser that the manual trigger belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"Required. The ID of the manual trigger to deactivate."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("manual-triggers-get",
                    Some(r##"Gets a manual trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_manual-triggers-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser this manual trigger belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"Required. The ID of the manual trigger to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("manual-triggers-list",
                    Some(r##"Lists manual triggers that are accessible to the current user for a given advertiser ID. The order is defined by the order_by parameter. A single advertiser_id is required."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_manual-triggers-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser that the fetched manual triggers belong to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("manual-triggers-patch",
                    Some(r##"Updates a manual trigger. Returns the updated manual trigger if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_manual-triggers-patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. Immutable. The unique ID of the advertiser that the manual trigger belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the manual trigger."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-create",
                    Some(r##"Creates a new negative keyword list. Returns the newly created negative keyword list if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the negative keyword list will belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-delete",
                    Some(r##"Deletes a negative keyword list given an advertiser ID and a negative keyword list ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the negative keyword list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"negative-keyword-list-id"##),
                     None,
                     Some(r##"Required. The ID of the negative keyword list to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-get",
                    Some(r##"Gets a negative keyword list given an advertiser ID and a negative keyword list ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the fetched negative keyword list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"negative-keyword-list-id"##),
                     None,
                     Some(r##"Required. The ID of the negative keyword list to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-list",
                    Some(r##"Lists negative keyword lists based on a given advertiser id."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the fetched negative keyword lists belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-negative-keywords-bulk-edit",
                    Some(r##"Bulk edits negative keywords in a single negative keyword list. The operation will delete the negative keywords provided in BulkEditNegativeKeywordsRequest.deleted_negative_keywords and then create the negative keywords provided in BulkEditNegativeKeywordsRequest.created_negative_keywords. This operation is guaranteed to be atomic and will never result in a partial success or partial failure."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-negative-keywords-bulk-edit",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"negative-keyword-list-id"##),
                     None,
                     Some(r##"Required. The ID of the parent negative keyword list to which the negative keywords belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-negative-keywords-create",
                    Some(r##"Creates a negative keyword in a negative keyword list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-negative-keywords-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"negative-keyword-list-id"##),
                     None,
                     Some(r##"Required. The ID of the parent negative keyword list in which the negative keyword will be created."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-negative-keywords-delete",
                    Some(r##"Deletes a negative keyword from a negative keyword list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-negative-keywords-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"negative-keyword-list-id"##),
                     None,
                     Some(r##"Required. The ID of the parent negative keyword list to which the negative keyword belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"keyword-value"##),
                     None,
                     Some(r##"Required. The keyword value of the negative keyword to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-negative-keywords-list",
                    Some(r##"Lists negative keywords in a negative keyword list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-negative-keywords-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"negative-keyword-list-id"##),
                     None,
                     Some(r##"Required. The ID of the parent negative keyword list to which the requested negative keywords belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-negative-keywords-replace",
                    Some(r##"Replaces all negative keywords in a single negative keyword list. The operation will replace the keywords in a negative keyword list with keywords provided in ReplaceNegativeKeywordsRequest.new_negative_keywords."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-negative-keywords-replace",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the parent negative keyword list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"negative-keyword-list-id"##),
                     None,
                     Some(r##"Required. The ID of the parent negative keyword list to which the negative keywords belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("negative-keyword-lists-patch",
                    Some(r##"Updates a negative keyword list. Returns the updated negative keyword list if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_negative-keyword-lists-patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the DV360 advertiser to which the negative keyword list belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"negative-keyword-list-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the negative keyword list. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an existing advertiser. Returns the updated advertiser if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_patch",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the advertiser. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-types-assigned-targeting-options-create",
                    Some(r##"Assigns a targeting option to an advertiser. Returns the assigned targeting option if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_targeting-types-assigned-targeting-options-create",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-types-assigned-targeting-options-delete",
                    Some(r##"Deletes an assigned targeting option from an advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_targeting-types-assigned-targeting-options-delete",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-targeting-option-id"##),
                     None,
                     Some(r##"Required. The ID of the assigned targeting option to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-types-assigned-targeting-options-get",
                    Some(r##"Gets a single targeting option assigned to an advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_targeting-types-assigned-targeting-options-get",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-targeting-option-id"##),
                     None,
                     Some(r##"Required. An identifier unique to the targeting type in this advertiser that identifies the assigned targeting option being requested."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-types-assigned-targeting-options-list",
                    Some(r##"Lists the targeting options assigned to an advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/advertisers_targeting-types-assigned-targeting-options-list",
                  vec![
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Required. The ID of the advertiser."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of assigned targeting options to list. Supported targeting types: * `TARGETING_TYPE_CHANNEL` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_OMID` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("combined-audiences", "methods: 'get' and 'list'", vec![
            ("get",
                    Some(r##"Gets a combined audience."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/combined-audiences_get",
                  vec![
                    (Some(r##"combined-audience-id"##),
                     None,
                     Some(r##"Required. The ID of the combined audience to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists combined audiences. The order is defined by the order_by parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/combined-audiences_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("custom-bidding-algorithms", "methods: 'create', 'get', 'list', 'patch', 'scripts-create', 'scripts-get', 'scripts-list' and 'upload-script'", vec![
            ("create",
                    Some(r##"Creates a new custom bidding algorithm. Returns the newly created custom bidding algorithm if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-bidding-algorithms_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets a custom bidding algorithm."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-bidding-algorithms_get",
                  vec![
                    (Some(r##"custom-bidding-algorithm-id"##),
                     None,
                     Some(r##"Required. The ID of the custom bidding algorithm to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists custom bidding algorithms that are accessible to the current user and can be used in bidding stratgies. The order is defined by the order_by parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-bidding-algorithms_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an existing custom bidding algorithm. Returns the updated custom bidding algorithm if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-bidding-algorithms_patch",
                  vec![
                    (Some(r##"custom-bidding-algorithm-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the custom bidding algorithm. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("scripts-create",
                    Some(r##"Creates a new custom bidding script. Returns the newly created script if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-bidding-algorithms_scripts-create",
                  vec![
                    (Some(r##"custom-bidding-algorithm-id"##),
                     None,
                     Some(r##"Required. The ID of the custom bidding algorithm that owns the script."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("scripts-get",
                    Some(r##"Gets a custom bidding script."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-bidding-algorithms_scripts-get",
                  vec![
                    (Some(r##"custom-bidding-algorithm-id"##),
                     None,
                     Some(r##"Required. The ID of the custom bidding algorithm owns the script."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-bidding-script-id"##),
                     None,
                     Some(r##"Required. The ID of the custom bidding script to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("scripts-list",
                    Some(r##"Lists custom bidding scripts that belong to the given algorithm. The order is defined by the order_by parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-bidding-algorithms_scripts-list",
                  vec![
                    (Some(r##"custom-bidding-algorithm-id"##),
                     None,
                     Some(r##"Required. The ID of the custom bidding algorithm owns the script."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("upload-script",
                    Some(r##"Creates a custom bidding script reference object for a script file. The resulting reference object provides a resource path to which the script file should be uploaded. This reference object should be included in when creating a new custom bidding script object."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-bidding-algorithms_upload-script",
                  vec![
                    (Some(r##"custom-bidding-algorithm-id"##),
                     None,
                     Some(r##"Required. The ID of the custom bidding algorithm owns the script."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("custom-lists", "methods: 'get' and 'list'", vec![
            ("get",
                    Some(r##"Gets a custom list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-lists_get",
                  vec![
                    (Some(r##"custom-list-id"##),
                     None,
                     Some(r##"Required. The ID of the custom list to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists custom lists. The order is defined by the order_by parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/custom-lists_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("first-and-third-party-audiences", "methods: 'create', 'edit-customer-match-members', 'get', 'list' and 'patch'", vec![
            ("create",
                    Some(r##"Creates a FirstAndThirdPartyAudience. Only supported for the following audience_type: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`"##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/first-and-third-party-audiences_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("edit-customer-match-members",
                    Some(r##"Updates the member list of a Customer Match audience. Only supported for the following audience_type: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`"##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/first-and-third-party-audiences_edit-customer-match-members",
                  vec![
                    (Some(r##"first-and-third-party-audience-id"##),
                     None,
                     Some(r##"Required. The ID of the Customer Match FirstAndThirdPartyAudience whose members will be edited."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets a first and third party audience."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/first-and-third-party-audiences_get",
                  vec![
                    (Some(r##"first-and-third-party-audience-id"##),
                     None,
                     Some(r##"Required. The ID of the first and third party audience to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists first and third party audiences. The order is defined by the order_by parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/first-and-third-party-audiences_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an existing FirstAndThirdPartyAudience. Only supported for the following audience_type: * `CUSTOMER_MATCH_CONTACT_INFO` * `CUSTOMER_MATCH_DEVICE_ID`"##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/first-and-third-party-audiences_patch",
                  vec![
                    (Some(r##"first-and-third-party-audience-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the first and third party audience. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("floodlight-groups", "methods: 'get' and 'patch'", vec![
            ("get",
                    Some(r##"Gets a Floodlight group."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/floodlight-groups_get",
                  vec![
                    (Some(r##"floodlight-group-id"##),
                     None,
                     Some(r##"Required. The ID of the Floodlight group to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an existing Floodlight group. Returns the updated Floodlight group if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/floodlight-groups_patch",
                  vec![
                    (Some(r##"floodlight-group-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the Floodlight group. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("google-audiences", "methods: 'get' and 'list'", vec![
            ("get",
                    Some(r##"Gets a Google audience."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/google-audiences_get",
                  vec![
                    (Some(r##"google-audience-id"##),
                     None,
                     Some(r##"Required. The ID of the Google audience to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists Google audiences. The order is defined by the order_by parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/google-audiences_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("guaranteed-orders", "methods: 'create', 'edit-guaranteed-order-read-accessors', 'get', 'list' and 'patch'", vec![
            ("create",
                    Some(r##"Creates a new guaranteed order. Returns the newly created guaranteed order if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/guaranteed-orders_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("edit-guaranteed-order-read-accessors",
                    Some(r##"Edits read advertisers of a guaranteed order."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/guaranteed-orders_edit-guaranteed-order-read-accessors",
                  vec![
                    (Some(r##"guaranteed-order-id"##),
                     None,
                     Some(r##"Required. The ID of the guaranteed order to edit. The ID is of the format `{exchange}-{legacy_guaranteed_order_id}`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets a guaranteed order."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/guaranteed-orders_get",
                  vec![
                    (Some(r##"guaranteed-order-id"##),
                     None,
                     Some(r##"Required. The ID of the guaranteed order to fetch. The ID is of the format `{exchange}-{legacy_guaranteed_order_id}`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists guaranteed orders that are accessible to the current user. The order is defined by the order_by parameter. If a filter by entity_status is not specified, guaranteed orders with entity status `ENTITY_STATUS_ARCHIVED` will not be included in the results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/guaranteed-orders_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an existing guaranteed order. Returns the updated guaranteed order if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/guaranteed-orders_patch",
                  vec![
                    (Some(r##"guaranteed-order-id"##),
                     None,
                     Some(r##"Output only. The unique identifier of the guaranteed order. The guaranteed order IDs have the format `{exchange}-{legacy_guaranteed_order_id}`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("inventory-source-groups", "methods: 'assigned-inventory-sources-bulk-edit', 'assigned-inventory-sources-create', 'assigned-inventory-sources-delete', 'assigned-inventory-sources-list', 'create', 'delete', 'get', 'list' and 'patch'", vec![
            ("assigned-inventory-sources-bulk-edit",
                    Some(r##"Bulk edits multiple assignments between inventory sources and a single inventory source group. The operation will delete the assigned inventory sources provided in BulkEditAssignedInventorySourcesRequest.deleted_assigned_inventory_sources and then create the assigned inventory sources provided in BulkEditAssignedInventorySourcesRequest.created_assigned_inventory_sources."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_assigned-inventory-sources-bulk-edit",
                  vec![
                    (Some(r##"inventory-source-group-id"##),
                     None,
                     Some(r##"Required. The ID of the inventory source group to which the assignments are assigned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("assigned-inventory-sources-create",
                    Some(r##"Creates an assignment between an inventory source and an inventory source group."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_assigned-inventory-sources-create",
                  vec![
                    (Some(r##"inventory-source-group-id"##),
                     None,
                     Some(r##"Required. The ID of the inventory source group to which the assignment will be assigned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("assigned-inventory-sources-delete",
                    Some(r##"Deletes the assignment between an inventory source and an inventory source group."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_assigned-inventory-sources-delete",
                  vec![
                    (Some(r##"inventory-source-group-id"##),
                     None,
                     Some(r##"Required. The ID of the inventory source group to which this assignment is assigned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-inventory-source-id"##),
                     None,
                     Some(r##"Required. The ID of the assigned inventory source to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("assigned-inventory-sources-list",
                    Some(r##"Lists inventory sources assigned to an inventory source group."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_assigned-inventory-sources-list",
                  vec![
                    (Some(r##"inventory-source-group-id"##),
                     None,
                     Some(r##"Required. The ID of the inventory source group to which these assignments are assigned."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("create",
                    Some(r##"Creates a new inventory source group. Returns the newly created inventory source group if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("delete",
                    Some(r##"Deletes an inventory source group."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_delete",
                  vec![
                    (Some(r##"inventory-source-group-id"##),
                     None,
                     Some(r##"Required. The ID of the inventory source group to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets an inventory source group."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_get",
                  vec![
                    (Some(r##"inventory-source-group-id"##),
                     None,
                     Some(r##"Required. The ID of the inventory source group to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists inventory source groups that are accessible to the current user. The order is defined by the order_by parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an inventory source group. Returns the updated inventory source group if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-source-groups_patch",
                  vec![
                    (Some(r##"inventory-source-group-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the inventory source group. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("inventory-sources", "methods: 'create', 'edit-inventory-source-read-write-accessors', 'get', 'list' and 'patch'", vec![
            ("create",
                    Some(r##"Creates a new inventory source. Returns the newly created inventory source if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-sources_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("edit-inventory-source-read-write-accessors",
                    Some(r##"Edits read/write accessors of an inventory source. Returns the updated read_write_accessors for the inventory source."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-sources_edit-inventory-source-read-write-accessors",
                  vec![
                    (Some(r##"inventory-source-id"##),
                     None,
                     Some(r##"Required. The ID of inventory source to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets an inventory source."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-sources_get",
                  vec![
                    (Some(r##"inventory-source-id"##),
                     None,
                     Some(r##"Required. The ID of the inventory source to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists inventory sources that are accessible to the current user. The order is defined by the order_by parameter. If a filter by entity_status is not specified, inventory sources with entity status `ENTITY_STATUS_ARCHIVED` will not be included in the results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-sources_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an existing inventory source. Returns the updated inventory source if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/inventory-sources_patch",
                  vec![
                    (Some(r##"inventory-source-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the inventory source. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("media", "methods: 'download' and 'upload'", vec![
            ("download",
                    Some(r##"Downloads media. Download is supported on the URI `/download/{resource_name=**}?alt=media.` **Note**: Download requests will not be successful without including `alt=media` query string."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/media_download",
                  vec![
                    (Some(r##"resource-name"##),
                     None,
                     Some(r##"Name of the media that is being downloaded. See ReadRequest.resource_name."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("upload",
                    Some(r##"Uploads media. Upload is supported on the URI `/upload/media/{resource_name=**}?upload_type=media.` **Note**: Upload requests will not be successful without including `upload_type=media` query string."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/media_upload",
                  vec![
                    (Some(r##"resource-name"##),
                     None,
                     Some(r##"Name of the media that is being downloaded. See ReadRequest.resource_name."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("partners", "methods: 'bulk-edit-partner-assigned-targeting-options', 'channels-create', 'channels-get', 'channels-list', 'channels-patch', 'channels-sites-bulk-edit', 'channels-sites-create', 'channels-sites-delete', 'channels-sites-list', 'channels-sites-replace', 'get', 'list', 'targeting-types-assigned-targeting-options-create', 'targeting-types-assigned-targeting-options-delete', 'targeting-types-assigned-targeting-options-get' and 'targeting-types-assigned-targeting-options-list'", vec![
            ("bulk-edit-partner-assigned-targeting-options",
                    Some(r##"Bulk edits targeting options under a single partner. The operation will delete the assigned targeting options provided in BulkEditPartnerAssignedTargetingOptionsRequest.deleteRequests and then create the assigned targeting options provided in BulkEditPartnerAssignedTargetingOptionsRequest.createRequests ."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_bulk-edit-partner-assigned-targeting-options",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"Required. The ID of the partner."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-create",
                    Some(r##"Creates a new channel. Returns the newly created channel if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-create",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the created channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-get",
                    Some(r##"Gets a channel for a partner or advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-get",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the fetched channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the channel to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-list",
                    Some(r##"Lists channels for a partner or advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-list",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the channels."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-patch",
                    Some(r##"Updates a channel. Returns the updated channel if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-patch",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the created channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the channel. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-bulk-edit",
                    Some(r##"Bulk edits sites under a single channel. The operation will delete the sites provided in BulkEditSitesRequest.deleted_sites and then create the sites provided in BulkEditSitesRequest.created_sites."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-sites-bulk-edit",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel to which the sites belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-create",
                    Some(r##"Creates a site in a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-sites-create",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel in which the site will be created."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-delete",
                    Some(r##"Deletes a site from a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-sites-delete",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel to which the site belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"url-or-app-id"##),
                     None,
                     Some(r##"Required. The URL or app ID of the site to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-list",
                    Some(r##"Lists sites in a channel."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-sites-list",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel to which the requested sites belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("channels-sites-replace",
                    Some(r##"Replaces all of the sites under a single channel. The operation will replace the sites under a channel with the sites provided in ReplaceSitesRequest.new_sites."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_channels-sites-replace",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"The ID of the partner that owns the parent channel."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"channel-id"##),
                     None,
                     Some(r##"Required. The ID of the parent channel whose sites will be replaced."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets a partner."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_get",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"Required. The ID of the partner to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists partners that are accessible to the current user. The order is defined by the order_by parameter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-types-assigned-targeting-options-create",
                    Some(r##"Assigns a targeting option to a partner. Returns the assigned targeting option if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_targeting-types-assigned-targeting-options-create",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"Required. The ID of the partner."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-types-assigned-targeting-options-delete",
                    Some(r##"Deletes an assigned targeting option from a partner."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_targeting-types-assigned-targeting-options-delete",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"Required. The ID of the partner."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-targeting-option-id"##),
                     None,
                     Some(r##"Required. The ID of the assigned targeting option to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-types-assigned-targeting-options-get",
                    Some(r##"Gets a single targeting option assigned to a partner."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_targeting-types-assigned-targeting-options-get",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"Required. The ID of the partner."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of this assigned targeting option. Supported targeting types: * `TARGETING_TYPE_CHANNEL`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"assigned-targeting-option-id"##),
                     None,
                     Some(r##"Required. An identifier unique to the targeting type in this partner that identifies the assigned targeting option being requested."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-types-assigned-targeting-options-list",
                    Some(r##"Lists the targeting options assigned to a partner."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/partners_targeting-types-assigned-targeting-options-list",
                  vec![
                    (Some(r##"partner-id"##),
                     None,
                     Some(r##"Required. The ID of the partner."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. Identifies the type of assigned targeting options to list. Supported targeting types: * `TARGETING_TYPE_CHANNEL`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("sdfdownloadtasks", "methods: 'create' and 'operations-get'", vec![
            ("create",
                    Some(r##"Creates an SDF Download Task. Returns an Operation. An SDF Download Task is a long-running, asynchronous operation. The metadata type of this operation is SdfDownloadTaskMetadata. If the request is successful, the response type of the operation is SdfDownloadTask. The response will not include the download files, which must be retrieved with media.download. The state of operation can be retrieved with sdfdownloadtask.operations.get. Any errors can be found in the error.message. Note that error.details is expected to be empty."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/sdfdownloadtasks_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("operations-get",
                    Some(r##"Gets the latest state of an asynchronous SDF download task operation. Clients should poll this method at intervals of 30 seconds."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/sdfdownloadtasks_operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("targeting-types", "methods: 'targeting-options-get', 'targeting-options-list' and 'targeting-options-search'", vec![
            ("targeting-options-get",
                    Some(r##"Gets a single targeting option."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/targeting-types_targeting-options-get",
                  vec![
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. The type of targeting option to retrieve. Accepted values are: * `TARGETING_TYPE_APP_CATEGORY` * `TARGETING_TYPE_AGE_RANGE` * `TARGETING_TYPE_GENDER` * `TARGETING_TYPE_VIDEO_PLAYER_SIZE` * `TARGETING_TYPE_USER_REWARDED_CONTENT` * `TARGETING_TYPE_PARENTAL_STATUS` * `TARGETING_TYPE_CONTENT_INSTREAM_POSITION` * `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION` * `TARGETING_TYPE_DEVICE_TYPE` * `TARGETING_TYPE_BROWSER` * `TARGETING_TYPE_HOUSEHOLD_INCOME` * `TARGETING_TYPE_ON_SCREEN_POSITION` * `TARGETING_TYPE_CARRIER_AND_ISP` * `TARGETING_TYPE_OPERATING_SYSTEM` * `TARGETING_TYPE_DEVICE_MAKE_MODEL` * `TARGETING_TYPE_ENVIRONMENT` * `TARGETING_TYPE_CATEGORY` * `TARGETING_TYPE_VIEWABILITY` * `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS` * `TARGETING_TYPE_LANGUAGE` * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_EXCHANGE` * `TARGETING_TYPE_SUB_EXCHANGE` * `TARGETING_TYPE_NATIVE_CONTENT_POSITION` * `TARGETING_TYPE_OMID`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"targeting-option-id"##),
                     None,
                     Some(r##"Required. The ID of the of targeting option to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-options-list",
                    Some(r##"Lists targeting options of a given type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/targeting-types_targeting-options-list",
                  vec![
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. The type of targeting option to be listed. Accepted values are: * `TARGETING_TYPE_APP_CATEGORY` * `TARGETING_TYPE_AGE_RANGE` * `TARGETING_TYPE_GENDER` * `TARGETING_TYPE_VIDEO_PLAYER_SIZE` * `TARGETING_TYPE_USER_REWARDED_CONTENT` * `TARGETING_TYPE_PARENTAL_STATUS` * `TARGETING_TYPE_CONTENT_INSTREAM_POSITION` * `TARGETING_TYPE_CONTENT_OUTSTREAM_POSITION` * `TARGETING_TYPE_DEVICE_TYPE` * `TARGETING_TYPE_BROWSER` * `TARGETING_TYPE_HOUSEHOLD_INCOME` * `TARGETING_TYPE_ON_SCREEN_POSITION` * `TARGETING_TYPE_CARRIER_AND_ISP` * `TARGETING_TYPE_OPERATING_SYSTEM` * `TARGETING_TYPE_DEVICE_MAKE_MODEL` * `TARGETING_TYPE_ENVIRONMENT` * `TARGETING_TYPE_CATEGORY` * `TARGETING_TYPE_VIEWABILITY` * `TARGETING_TYPE_AUTHORIZED_SELLER_STATUS` * `TARGETING_TYPE_LANGUAGE` * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_DIGITAL_CONTENT_LABEL_EXCLUSION` * `TARGETING_TYPE_SENSITIVE_CATEGORY_EXCLUSION` * `TARGETING_TYPE_EXCHANGE` * `TARGETING_TYPE_SUB_EXCHANGE` * `TARGETING_TYPE_NATIVE_CONTENT_POSITION` * `TARGETING_TYPE_OMID`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("targeting-options-search",
                    Some(r##"Searches for targeting options of a given type based on the given search terms."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/targeting-types_targeting-options-search",
                  vec![
                    (Some(r##"targeting-type"##),
                     None,
                     Some(r##"Required. The type of targeting options to retrieve. Accepted values are: * `TARGETING_TYPE_GEO_REGION` * `TARGETING_TYPE_POI` * `TARGETING_TYPE_BUSINESS_CHAIN`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
        ("users", "methods: 'bulk-edit-assigned-user-roles', 'create', 'delete', 'get', 'list' and 'patch'", vec![
            ("bulk-edit-assigned-user-roles",
                    Some(r##"Bulk edits user roles for a user. The operation will delete the assigned user roles provided in BulkEditAssignedUserRolesRequest.deletedAssignedUserRoles and then assign the user roles provided in BulkEditAssignedUserRolesRequest.createdAssignedUserRoles."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/users_bulk-edit-assigned-user-roles",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Required. The ID of the user to which the assigned user roles belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("create",
                    Some(r##"Creates a new user. Returns the newly created user if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/users_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("delete",
                    Some(r##"Deletes a user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/users_delete",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Required. The ID of the user to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets a user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/users_get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Required. The ID of the user to fetch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("list",
                    Some(r##"Lists users that are accessible to the current user. If two users have user roles on the same partner or advertiser, they can access each other."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/users_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates an existing user. Returns the updated user if successful."##),
                    "Details at http://byron.github.io/google-apis-rs/google_displayvideo1_cli/users_patch",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Output only. The unique ID of the user. Assigned by the system."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("displayvideo1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.2+20230119")
           .about("Display & Video 360 API allows users to automate complex Display & Video 360 workflows, such as creating insertion orders and setting targeting options for individual line items.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_displayvideo1_cli")
           .arg(Arg::with_name("url")
                   .long("scope")
                   .help("Specify the authentication a method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
                   .multiple(true)
                   .takes_value(true))
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Debug print all errors")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       if arg_name_str == "mode" {
                           arg = arg.number_of_values(2);
                           arg = arg.value_names(&upload_value_names);
           
                           scmd = scmd.arg(Arg::with_name("mime")
                                               .short("m")
                                               .requires("mode")
                                               .required(false)
                                               .help("The file's mime time, like 'application/octet-stream'")
                                               .takes_value(true));
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("adebug");
    let connector = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
